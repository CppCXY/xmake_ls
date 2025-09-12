use std::time::Duration;

use tokio_util::sync::CancellationToken;

pub fn time_cancel_token(time: Duration) -> CancellationToken {
    let cancel_token = CancellationToken::new();
    let cancel_handle = cancel_token.clone();
    tokio::spawn(async move {
        tokio::time::sleep(time).await;
        cancel_handle.cancel();
    });
    cancel_token
}
