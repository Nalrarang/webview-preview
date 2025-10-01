use tauri::{WebviewUrl, WebviewWindowBuilder, LogicalPosition, LogicalSize};
use tauri::webview::WebviewBuilder;
use url::Url;

#[tauri::command]
fn get_url_validation(url: String) -> Result<String, String> {
    match url::Url::parse(&url) {
        Ok(_) => Ok(url),
        Err(_) => Err("Invalid URL format".to_string()),
    }
}

#[tauri::command]
fn navigate_to_url(window: tauri::Window, url: String) -> Result<(), String> {
    // URL 유효성 검사
    if let Err(_) = Url::parse(&url) {
        return Err("Invalid URL format".to_string());
    }

    println!("Looking for child webview...");

    // 현재 윈도우에서 차일드 웹뷰 찾기
    let webviews = window.webviews();
    println!("Found {} webviews in current window", webviews.len());

    for webview in webviews {
        let label = webview.label();
        println!("Checking webview: {}", label);
        if label == "webview" {
            match webview.navigate(url::Url::parse(&url).unwrap()) {
                Ok(_) => {
                    println!("Navigation successful");
                    return Ok(());
                }
                Err(e) => return Err(format!("Failed to navigate: {}", e)),
            }
        }
    }
    Err("Child webview 'webview' not found".to_string())
}

#[tauri::command]
fn execute_js_in_webview(window: tauri::Window, js_code: String) -> Result<(), String> {
    println!("Executing JS in child webview...");

    // 현재 윈도우에서 차일드 웹뷰 찾기
    let webviews = window.webviews();
    println!("Found {} webviews", webviews.len());

    for webview in webviews {
        let label = webview.label();
        if label == "webview" {
            match webview.eval(&js_code) {
                Ok(_) => {
                    println!("JS execution successful");
                    return Ok(());
                }
                Err(e) => return Err(format!("Failed to execute JavaScript: {}", e)),
            }
        }
    }
    Err("Child webview 'webview' not found".to_string())
}

#[tauri::command]
fn open_devtools(window: tauri::Window) -> Result<(), String> {
    println!("Opening DevTools for child webview...");

    // 현재 윈도우에서 차일드 웹뷰 찾기
    let webviews = window.webviews();

    for webview in webviews {
        let label = webview.label();
        if label == "webview" {
            webview.open_devtools();
            println!("DevTools opened");
            return Ok(());
        }
    }
    Err("Child webview 'webview' not found".to_string())
}

#[tauri::command]
fn resize_window(window: tauri::Window, width: f64, height: f64) -> Result<(), String> {
    match window.set_size(LogicalSize::new(width, height)) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to resize window: {}", e))
    }
}

#[tauri::command]
fn move_webview(window: tauri::Window, x: f64, y: f64) -> Result<(), String> {
    let webviews = window.webviews();

    for webview in webviews {
        let label = webview.label();
        if label == "webview" {
            match webview.set_position(LogicalPosition::new(x, y)) {
                Ok(_) => return Ok(()),
                Err(e) => return Err(format!("Failed to move webview: {}", e))
            }
        }
    }
    Err("Child webview 'webview' not found".to_string())
}

#[tauri::command]
fn set_always_on_top(window: tauri::Window, always_on_top: bool) -> Result<(), String> {
    match window.set_always_on_top(always_on_top) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to set always on top: {}", e))
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_url_validation,
            navigate_to_url,
            execute_js_in_webview,
            open_devtools,
            resize_window,
            move_webview,
            set_always_on_top
        ])
        .setup(|app| {
            // 메인 창 생성 (모바일 사이즈 + 하단 바코드 입력)
            // 전체 창 크기: 375px x 667px (웹뷰 617px + 바코드 입력 50px)
            let main_window = WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::App("index.html".into())
            )
            .title("Mobile WebView App")
            .inner_size(375.0, 667.0)
            .position(100.0, 100.0)
            .always_on_top(true)
            .build()
            .expect("Failed to create main window");

            // Window 객체로 접근하여 차일드 웹뷰 추가 (상단 617px)
            let window = main_window.as_ref().window();
            let _child_webview = window.add_child(
                WebviewBuilder::new("webview", WebviewUrl::External(
                    url::Url::parse("https://alpha.wms.kakaostyle.com").unwrap()
                )),
                LogicalPosition::new(0.0, 0.0),
                LogicalSize::new(375.0, 617.0)
            )
            .expect("Failed to add child webview");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
