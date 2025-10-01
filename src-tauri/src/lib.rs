use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};
use url::Url;

#[tauri::command]
fn open_webview(app_handle: tauri::AppHandle, url: String, enable_devtools: bool) -> Result<(), String> {
    // URL 유효성 검사
    if let Err(_) = Url::parse(&url) {
        return Err("Invalid URL format".to_string());
    }

    let webview_url = WebviewUrl::External(url.parse().unwrap());
    
    let mut webview_builder = WebviewWindowBuilder::new(
        &app_handle,
        "webview",
        webview_url,
    )
    .title("Mobile WebView")
    .inner_size(375.0, 812.0) // iPhone 크기
    .min_inner_size(320.0, 568.0)
    .resizable(true)
    .center();

    if enable_devtools {
        webview_builder = webview_builder.devtools(true);
    }

    match webview_builder.build() {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to create webview: {}", e)),
    }
}

#[tauri::command]
fn execute_js(app_handle: tauri::AppHandle, js_code: String) -> Result<(), String> {
    if let Some(webview) = app_handle.get_webview_window("webview") {
        match webview.eval(&js_code) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to execute JavaScript: {}", e)),
        }
    } else {
        Err("Webview not found".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![open_webview, execute_js])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
