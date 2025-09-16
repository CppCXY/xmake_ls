use std::path::PathBuf;

use lsp_types::ShowMessageParams;

use crate::{
    context::{ProgressTask, ServerContextSnapshot},
    handlers::initialized::collect_files::collect_files,
};

pub async fn init_xmake(context: &ServerContextSnapshot) {
    if context.xmake().check_available().await {
        log::info!("xmake is available");
    } else {
        log::warn!("xmake is not available");
        let message = ShowMessageParams {
            typ: lsp_types::MessageType::ERROR,
            message: "xmake is not available. Please ensure xmake is installed and accessible in your system PATH.".to_string(),
        };

        context.client().show_message(message);
        return;
    }

    // donot need load xmake lib files for now

    let Some(xmake_path) = context.xmake().get_xmake_path().await else {
        log::warn!("xmake directory not found");
        let message = ShowMessageParams {
            typ: lsp_types::MessageType::ERROR,
            message: "xmake directory not found, Please set the XMAKE_ROOT or XMAKE_HOME environment variable. ".to_string(),
        };

        context.client().show_message(message);
        return;
    };

    log::info!("xmake path: {:?}", xmake_path);
    let xmake_dir_path = match xmake_path.parent().map(|p| p.to_path_buf()) {
        Some(path) => path,
        None => {
            log::warn!("xmake directory not found");
            let message = ShowMessageParams {
                typ: lsp_types::MessageType::ERROR,
                message: "xmake directory not found, Please set the XMAKE_ROOT or XMAKE_HOME environment variable. ".to_string(),
            };

            context.client().show_message(message);
            return;
        }
    };
    log::info!("start to load xmake lib files from {:?}", xmake_dir_path);
    let status_bar = context.status_bar();
    status_bar
        .create_progress_task(ProgressTask::XmakeLoad)
        .await;

    let mut analysis = context.analysis().write().await;
    let emmyrc = analysis.get_emmyrc();
    let xmake_worksapce = vec![
        xmake_dir_path.join("core/sandbox/modules/import"),
        // other xmake lib paths can be added here if needed
    ];
    for lib_workspace in &xmake_worksapce {
        analysis.add_builtin_import_workspace(lib_workspace.clone());
    }

    let xmake_lib_files = collect_files(&xmake_worksapce, &emmyrc);

    let files: Vec<(PathBuf, Option<String>)> = xmake_lib_files
        .into_iter()
        .map(|file| file.into_tuple())
        .collect();
    let file_count = files.len();
    if file_count != 0 {
        status_bar.update_progress_task(
            ProgressTask::XmakeLoad,
            None,
            Some(format!("Indexing {} xmake lib files", file_count)),
        );

        analysis.update_files_by_path(files);
    }

    status_bar.finish_progress_task(ProgressTask::XmakeLoad, None);
    log::info!("finished loading xmake lib files");
}
