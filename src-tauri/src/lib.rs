// Import our gesture detection module
use tauri::Manager;
#[cfg(target_os = "windows")]
mod gesture;
mod converter;

// Default Tauri command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tauri::command]
async fn convert_image(input_path: String,output_format: String,) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
        converter::convert_image(input_path, output_format)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, convert_image])

        // Run this when Tauri initializes
        .setup(|app| {
            println!("TAURI: Setup function running!");

            #[cfg(target_os = "windows")]
            {
                println!("TAURI: Starting gesture detector");

                gesture::start_gesture_detector(
                    app.handle().clone()
                );
            }
            // Automatically open Developer Tools
            // Only when running a development build.
            #[cfg(debug_assertions)]
            {
                if let Some(window) = app.get_webview_window("main") {
                    window.open_devtools();
                }
            }


            Ok(())
        })

        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}