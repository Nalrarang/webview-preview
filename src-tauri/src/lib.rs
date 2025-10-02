use tauri::{WebviewUrl, WebviewWindowBuilder, LogicalPosition, LogicalSize, Manager};
use tauri::webview::WebviewBuilder;
use url::Url;
use std::sync::atomic::{AtomicU32, Ordering};

// 전역 창 카운터
static WINDOW_COUNTER: AtomicU32 = AtomicU32::new(0);

// 헬퍼 함수: 차일드 웹뷰 찾기 (webview-* 패턴으로 검색)
fn find_child_webview(window: &tauri::Window) -> Result<tauri::Webview, String> {
    for webview in window.webviews() {
        let label = webview.label();
        if label.starts_with("webview-") {
            return Ok(webview);
        }
    }
    Err("Child webview not found".to_string())
}

// 새 창 생성 커맨드
#[tauri::command]
fn create_new_window(app: tauri::AppHandle) -> Result<(), String> {
    let window_id = WINDOW_COUNTER.fetch_add(1, Ordering::SeqCst);
    let window_label = format!("main-{}", window_id);
    let webview_label = format!("webview-{}", window_id);

    let main_window = WebviewWindowBuilder::new(
        &app,
        &window_label,
        WebviewUrl::App("index.html".into())
    )
    .title("Mobile WebView Preview")
    .inner_size(375.0, 667.0)
    .position(100.0 + (window_id as f64 * 30.0), 100.0 + (window_id as f64 * 30.0))
    .always_on_top(true)
    .build()
    .map_err(|e| format!("Failed to create window: {}", e))?;

    let window = main_window.as_ref().window();
    window.add_child(
        WebviewBuilder::new(
            &webview_label,
            WebviewUrl::External(Url::parse("about:blank").unwrap())
        ),
        LogicalPosition::new(0.0, 0.0),
        LogicalSize::new(375.0, 617.0)
    )
    .map_err(|e| format!("Failed to add webview: {}", e))?;

    Ok(())
}

#[tauri::command]
fn navigate_to_url(window: tauri::Window, url: String) -> Result<(), String> {
    if Url::parse(&url).is_err() {
        return Err("Invalid URL format".to_string());
    }

    let webview = find_child_webview(&window)?;
    webview.navigate(Url::parse(&url).unwrap())
        .map_err(|e| format!("Failed to navigate: {}", e))
}

#[tauri::command]
fn execute_js_in_webview(window: tauri::Window, js_code: String) -> Result<(), String> {
    let webview = find_child_webview(&window)?;
    webview.eval(&js_code)
        .map_err(|e| format!("Failed to execute JavaScript: {}", e))
}

#[tauri::command]
fn open_devtools(window: tauri::Window) -> Result<(), String> {
    let webview = find_child_webview(&window)?;
    webview.open_devtools();
    Ok(())
}

#[tauri::command]
fn resize_window(window: tauri::Window, width: f64, height: f64) -> Result<(), String> {
    window.set_size(LogicalSize::new(width, height))
        .map_err(|e| format!("Failed to resize window: {}", e))
}

#[tauri::command]
fn resize_webview(window: tauri::Window, width: f64, height: f64) -> Result<(), String> {
    let webview = find_child_webview(&window)?;
    webview.set_size(LogicalSize::new(width, height))
        .map_err(|e| format!("Failed to resize webview: {}", e))
}

#[tauri::command]
fn set_always_on_top(window: tauri::Window, always_on_top: bool) -> Result<(), String> {
    window.set_always_on_top(always_on_top)
        .map_err(|e| format!("Failed to set always on top: {}", e))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            create_new_window,
            navigate_to_url,
            execute_js_in_webview,
            open_devtools,
            resize_window,
            resize_webview,
            set_always_on_top
        ])
        .setup(|app| {
            // 고유한 창 ID 생성 (여러 인스턴스 허용)
            let window_id = WINDOW_COUNTER.fetch_add(1, Ordering::SeqCst);
            let window_label = format!("main-{}", window_id);
            let webview_label = format!("webview-{}", window_id);

            // 메인 창 생성: 375x667 (웹뷰 617px + 바코드 입력 50px)
            let main_window = WebviewWindowBuilder::new(
                app,
                &window_label,
                WebviewUrl::App("index.html".into())
            )
            .title("Mobile WebView Preview")
            .inner_size(375.0, 667.0)
            .position(100.0, 100.0)
            .always_on_top(true)
            .build()
            .expect("Failed to create main window");

            // 차일드 웹뷰 추가 (초기 URL은 about:blank, JavaScript에서 저장된 URL로 네비게이션)
            let window = main_window.as_ref().window();
            window.add_child(
                WebviewBuilder::new(
                    &webview_label,
                    WebviewUrl::External(Url::parse("about:blank").unwrap())
                ),
                LogicalPosition::new(0.0, 0.0),
                LogicalSize::new(375.0, 617.0)
            )
            .expect("Failed to add child webview");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
