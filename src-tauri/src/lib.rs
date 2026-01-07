use std::process::Command;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![clone_repo])
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn clone_repo(url: String, path: String) -> Result<String, String> {
    let output = Command::new("git")
        .env("GIT_ASKPASS", "true") // Set environment variables
        .args(["-c", "credential.helper=", "clone", &url, &path])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok("Repository cloned successfully.".into())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
