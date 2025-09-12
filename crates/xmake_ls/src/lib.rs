pub mod cmd_args;
mod context;
mod handlers;
mod logger;
mod meta_text;
mod util;

use crate::handlers::{
    initialized_handler, on_notification_handler, on_request_handler, on_response_handler,
};
pub use clap::Parser;
pub use cmd_args::*;
use handlers::server_capabilities;
use lsp_server::{Connection, Message, Response};
use lsp_types::InitializeParams;
use std::sync::Arc;
use std::{env, error::Error};
use tokio::sync::{mpsc, oneshot};

#[macro_use]
extern crate rust_i18n;
rust_i18n::i18n!("./locales", fallback = "en");

const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Async wrapper for LSP Connection with tokio support
pub struct AsyncConnection {
    connection: Arc<Connection>,
    receiver: mpsc::UnboundedReceiver<Message>,
    _receiver_task: tokio::task::JoinHandle<()>,
}

impl AsyncConnection {
    /// Create async version from sync Connection
    pub fn from_sync(connection: Connection) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        let connection = Arc::new(connection);

        // Spawn blocking task to convert sync receiver to async
        let connection_clone = connection.clone();
        let receiver_task = tokio::task::spawn_blocking(move || {
            for msg in &connection_clone.receiver {
                if tx.send(msg).is_err() {
                    break; // Receiver closed
                }
            }
        });

        Self {
            connection,
            receiver: rx,
            _receiver_task: receiver_task,
        }
    }

    /// Receive message asynchronously
    pub async fn recv(&mut self) -> Option<Message> {
        self.receiver.recv().await
    }

    /// Send message to client
    pub fn send(&self, msg: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.connection
            .sender
            .send(msg)
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
    }

    /// Handle shutdown request
    pub async fn handle_shutdown(
        &mut self,
        req: &lsp_server::Request,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        if req.method != "shutdown" {
            return Ok(false);
        }
        let resp = Response::new_ok(req.id.clone(), ());
        let _ = self.connection.sender.send(resp.into());
        match tokio::time::timeout(std::time::Duration::from_secs(30), self.receiver.recv()).await {
            Ok(Some(Message::Notification(n))) if n.method == "exit" => (),
            Ok(Some(msg)) => {
                return Err(Box::new(ExitError(format!(
                    "unexpected message during shutdown: {msg:?}"
                ))));
            }
            Ok(None) => {
                return Err(Box::new(ExitError(
                    "channel closed while waiting for exit notification".to_owned(),
                )));
            }
            Err(_) => {
                return Err(Box::new(ExitError(
                    "timed out waiting for exit notification".to_owned(),
                )));
            }
        }
        Ok(true)
    }
}

pub struct ExitError(pub String);

impl std::fmt::Debug for ExitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ExitError: {}", self.0)
    }
}

impl std::fmt::Display for ExitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ExitError {}

/// Server initialization and message processing state
struct ServerMessageProcessor {
    initialization_complete: bool,
    pending_messages: Vec<Message>,
    init_rx: oneshot::Receiver<()>,
}

impl ServerMessageProcessor {
    fn new(init_rx: oneshot::Receiver<()>) -> Self {
        Self {
            initialization_complete: false,
            pending_messages: Vec::new(),
            init_rx,
        }
    }

    /// Check if message can be processed during initialization
    fn can_process_during_init(&self, msg: &Message) -> bool {
        match msg {
            // Allow all responses (including configuration responses)
            Message::Response(_) => true,
            // Allow specific notifications
            Message::Notification(notify) => {
                matches!(
                    notify.method.as_str(),
                    "workspace/didChangeConfiguration" | "$/cancelRequest" | "initialized"
                )
            }
            // Don't process other requests during initialization
            Message::Request(_) => false,
        }
    }

    /// Process message during normal operation (after initialization)
    async fn process_message(
        &mut self,
        msg: Message,
        connection: &mut AsyncConnection,
        server_context: &mut context::ServerContext,
    ) -> Result<bool, Box<dyn Error + Sync + Send>> {
        // During normal operation, process all messages
        self.handle_message(msg, connection, server_context).await
    }

    /// Check if initialization is complete and process pending messages
    fn check_initialization_complete(&mut self) -> Result<bool, Box<dyn Error + Sync + Send>> {
        if !self.initialization_complete {
            match self.init_rx.try_recv() {
                Ok(_) => {
                    self.initialization_complete = true;
                    return Ok(true); // Signal to process pending messages
                }
                Err(oneshot::error::TryRecvError::Empty) => {
                    // Still initializing
                }
                Err(oneshot::error::TryRecvError::Closed) => {
                    // Initialization task closed unexpectedly
                    self.initialization_complete = true;
                    return Ok(true); // Signal to process pending messages
                }
            }
        }
        Ok(false)
    }

    /// Process all pending messages after initialization
    async fn process_pending_messages(
        &mut self,
        connection: &mut AsyncConnection,
        server_context: &mut context::ServerContext,
    ) -> Result<bool, Box<dyn Error + Sync + Send>> {
        let messages = std::mem::take(&mut self.pending_messages);
        for msg in messages {
            if self.handle_message(msg, connection, server_context).await? {
                return Ok(true); // Shutdown requested
            }
        }
        Ok(false)
    }

    /// Handle individual message
    async fn handle_message(
        &self,
        msg: Message,
        connection: &mut AsyncConnection,
        server_context: &mut context::ServerContext,
    ) -> Result<bool, Box<dyn Error + Sync + Send>> {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req).await? {
                    server_context.close().await;
                    return Ok(true); // Shutdown requested
                }
                on_request_handler(req, server_context).await?;
            }
            Message::Notification(notify) => {
                on_notification_handler(notify, server_context).await?;
            }
            Message::Response(response) => {
                on_response_handler(response, server_context).await?;
            }
        }
        Ok(false)
    }
}

#[allow(unused)]
pub async fn run_ls(cmd_args: CmdArgs) -> Result<(), Box<dyn Error + Sync + Send>> {
    let (connection, threads) = match cmd_args.communication {
        cmd_args::Communication::Stdio => Connection::stdio(),
        cmd_args::Communication::Tcp => {
            let port = cmd_args.port;
            let ip = cmd_args.ip.clone();
            let addr = (ip.as_str(), port);
            Connection::listen(addr).unwrap()
        }
    };

    let (id, params) = connection.initialize_start()?;
    let initialization_params: InitializeParams = serde_json::from_value(params).unwrap();
    let server_capbilities = server_capabilities(&initialization_params.capabilities);
    let initialize_data = serde_json::json!({
        "capabilities": server_capbilities,
        "serverInfo": {
            "name": CRATE_NAME,
            "version": CRATE_VERSION
        }
    });

    connection.initialize_finish(id, initialize_data)?;

    // Create async connection wrapper
    let mut async_connection = AsyncConnection::from_sync(connection);
    main_loop(async_connection, initialization_params, cmd_args).await?;
    threads.join()?;

    eprintln!("Server shutting down.");
    Ok(())
}

/// LSP Server manages the entire server lifecycle
struct LspServer {
    connection: AsyncConnection,
    server_context: context::ServerContext,
    processor: ServerMessageProcessor,
}

impl LspServer {
    /// Create a new LSP server instance
    fn new(
        connection: AsyncConnection,
        params: &InitializeParams,
        init_rx: oneshot::Receiver<()>,
    ) -> Self {
        let server_context = context::ServerContext::new(
            Connection {
                sender: connection.connection.sender.clone(),
                receiver: connection.connection.receiver.clone(),
            },
            Arc::new(params.capabilities.clone()),
        );

        Self {
            connection,
            server_context,
            processor: ServerMessageProcessor::new(init_rx),
        }
    }

    /// Run the main server loop
    async fn run(mut self) -> Result<(), Box<dyn Error + Sync + Send>> {
        // First, wait for initialization to complete while handling allowed messages
        self.wait_for_initialization().await?;

        // Process all pending messages after initialization
        if self
            .processor
            .process_pending_messages(&mut self.connection, &mut self.server_context)
            .await?
        {
            self.server_context.close().await;
            return Ok(()); // Shutdown requested during pending message processing
        }

        // Now focus on normal message processing
        while let Some(msg) = self.connection.recv().await {
            if self
                .processor
                .process_message(msg, &mut self.connection, &mut self.server_context)
                .await?
            {
                break; // Shutdown requested
            }
        }

        self.server_context.close().await;
        Ok(())
    }

    /// Wait for initialization to complete while handling initialization-allowed messages
    async fn wait_for_initialization(&mut self) -> Result<(), Box<dyn Error + Sync + Send>> {
        loop {
            // Check if initialization is complete
            if self.processor.check_initialization_complete()? {
                break; // Initialization completed
            }

            // Use a short timeout to check for messages during initialization
            match tokio::time::timeout(
                tokio::time::Duration::from_millis(50),
                self.connection.recv(),
            )
            .await
            {
                Ok(Some(msg)) => {
                    // Process message if allowed during initialization, otherwise queue it
                    if self.processor.can_process_during_init(&msg) {
                        self.processor
                            .handle_message(msg, &mut self.connection, &mut self.server_context)
                            .await?;
                    } else {
                        self.processor.pending_messages.push(msg);
                    }
                }
                Ok(None) => {
                    // Connection closed during initialization
                    return Ok(());
                }
                Err(_) => {
                    // Timeout - continue checking for initialization completion
                    continue;
                }
            }
        }
        Ok(())
    }
}

async fn main_loop(
    connection: AsyncConnection,
    params: InitializeParams,
    cmd_args: CmdArgs,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    // Setup initialization completion signal
    let (init_tx, init_rx) = oneshot::channel::<()>();

    // Create and configure server instance
    let server = LspServer::new(connection, &params, init_rx);

    // Start initialization process
    let server_context_snapshot = server.server_context.snapshot();
    tokio::spawn(async move {
        initialized_handler(server_context_snapshot, params, cmd_args).await;
        let _ = init_tx.send(());
    });

    // Run the server
    server.run().await
}
