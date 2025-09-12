use std::path::PathBuf;

pub fn get_best_resources_dir() -> PathBuf {
    if cfg!(debug_assertions) {
        return exe_dir().join("resources");
    }

    if cfg!(target_os = "windows") {
        // On Windows, try LOCALAPPDATA first
        if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
            PathBuf::from(local_app_data)
                .join("xmake_ls")
                .join("resources")
        } else {
            // Fall back to the directory next to the executable
            exe_dir().join("resources")
        }
    } else {
        // On non-Windows platforms, try XDG_DATA_HOME first
        if let Ok(xdg_data_home) = std::env::var("XDG_DATA_HOME") {
            PathBuf::from(xdg_data_home)
                .join("xmake_ls")
                .join("resources")
        } else {
            // If XDG_DATA_HOME is not set, use default XDG path
            if let Ok(home) = std::env::var("HOME") {
                PathBuf::from(home)
                    .join(".local")
                    .join("share")
                    .join("xmake_ls")
                    .join("resources")
            } else {
                // Fall back to the directory next to the executable
                exe_dir().join("resources")
            }
        }
    }
}

fn exe_dir() -> PathBuf {
    let mut exe = std::env::current_exe().expect("executable available");
    exe.pop();
    exe
}
