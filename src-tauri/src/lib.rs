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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_url_validation,
            navigate_to_url,
            execute_js_in_webview,
            open_devtools
        ])
        .setup(|app| {
            // 메인 창 생성 (컨트롤 패널이 포함된 HTML)
            // 전체 창 크기: 컨트롤 패널(350px) + 웹뷰(375px) = 725px 너비
            let main_window = WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::App("index.html".into())
            )
            .title("Mobile WebView App")
            .inner_size(725.0, 667.0)
            .position(100.0, 100.0)
            .build()
            .expect("Failed to create main window");

            // Window 객체로 접근하여 차일드 웹뷰 추가 (모바일 사이즈: 375x667)
            let window = main_window.as_ref().window();
            let _child_webview = window.add_child(
                WebviewBuilder::new("webview", WebviewUrl::External(
                    url::Url::parse("https://alpha.wms.kakaostyle.com").unwrap()
                )),
                LogicalPosition::new(350.0, 0.0),
                LogicalSize::new(375.0, 667.0)
            )
            .expect("Failed to add child webview");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
