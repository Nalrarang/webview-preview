use tauri::{WebviewUrl, WebviewWindowBuilder, LogicalPosition, LogicalSize};
use tauri::webview::WebviewBuilder;
use url::Url;
use std::sync::atomic::{AtomicU32, Ordering};

// 전역 창 카운터
static WINDOW_COUNTER: AtomicU32 = AtomicU32::new(0);
// 웹뷰 재생성 카운터
static WEBVIEW_RECREATION_COUNTER: AtomicU32 = AtomicU32::new(0);

// User-Agent 상수
const MOBILE_USER_AGENT: &str = "Mozilla/5.0 (iPhone; CPU iPhone OS 16_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.6 Mobile/15E148 Safari/604.1";
const DESKTOP_USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

// 헬퍼 함수: 차일드 웹뷰 찾기 (webview-* 패턴으로 검색)
// recreation 웹뷰를 우선적으로 찾고, 없으면 기본 webview-* 찾기
fn find_child_webview(window: &tauri::Window) -> Result<tauri::Webview, String> {
    let mut fallback_webview: Option<tauri::Webview> = None;
    let mut max_recreation_id = -1i32;
    let mut recreation_webview: Option<tauri::Webview> = None;

    for webview in window.webviews() {
        let label = webview.label();

        // webview-recreation-* 패턴 확인 (가장 최신 것 찾기)
        if label.starts_with("webview-recreation-") {
            if let Some(id_str) = label.strip_prefix("webview-recreation-") {
                if let Ok(id) = id_str.parse::<i32>() {
                    if id > max_recreation_id {
                        max_recreation_id = id;
                        recreation_webview = Some(webview.clone());
                    }
                }
            }
        }
        // 기본 webview-* 패턴 (fallback용)
        else if label.starts_with("webview-") && fallback_webview.is_none() {
            fallback_webview = Some(webview.clone());
        }
    }

    // recreation 웹뷰가 있으면 그것을 반환, 없으면 fallback
    recreation_webview
        .or(fallback_webview)
        .ok_or_else(|| "Child webview not found".to_string())
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
        )
        .user_agent(MOBILE_USER_AGENT),
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

#[tauri::command]
fn set_user_agent(window: tauri::Window, is_mobile: bool, current_url: String) -> Result<(), String> {
    let user_agent = if is_mobile {
        MOBILE_USER_AGENT
    } else {
        DESKTOP_USER_AGENT
    };

    // 현재 웹뷰 찾기 및 정보 저장
    let old_webview = find_child_webview(&window)?;
    let webview_size = old_webview.size().map_err(|e| format!("Failed to get size: {}", e))?;

    // 기존 웹뷰 닫기 시도
    let _ = old_webview.close();

    // 새로운 고유 라벨 생성
    let recreation_id = WEBVIEW_RECREATION_COUNTER.fetch_add(1, Ordering::SeqCst);
    let new_webview_label = format!("webview-recreation-{}", recreation_id);

    // 새 URL 파싱
    let url_to_load = if current_url.is_empty() || current_url == "about:blank" {
        Url::parse("about:blank").unwrap()
    } else {
        Url::parse(&current_url).map_err(|e| format!("Invalid URL: {}", e))?
    };

    // 새 웹뷰 생성 with User-Agent
    window.add_child(
        WebviewBuilder::new(
            &new_webview_label,
            WebviewUrl::External(url_to_load)
        )
        .user_agent(user_agent),
        LogicalPosition::new(0.0, 0.0),
        webview_size
    )
    .map_err(|e| format!("Failed to create new webview: {}", e))?;

    Ok(())
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
            set_always_on_top,
            set_user_agent
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
            // 기본적으로 모바일 User-Agent 설정
            let window = main_window.as_ref().window();
            window.add_child(
                WebviewBuilder::new(
                    &webview_label,
                    WebviewUrl::External(Url::parse("about:blank").unwrap())
                )
                .user_agent(MOBILE_USER_AGENT),
                LogicalPosition::new(0.0, 0.0),
                LogicalSize::new(375.0, 617.0)
            )
            .expect("Failed to add child webview");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
