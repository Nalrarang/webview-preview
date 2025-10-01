const { invoke } = window.__TAURI__.core;

let urlInputEl;
let devtoolsCheckboxEl;
let alwaysOnTopCheckboxEl;
let jsInputEl;
let statusMsgEl;
let loadingScreen;
let controlPanel;
let barcodeInputEl;

async function loadUrl() {
  const url = urlInputEl.value.trim();

  if (!url) {
    return;
  }

  try {
    await invoke("navigate_to_url", { url: url });
    hideLoadingScreen();
  } catch (error) {
    console.error("Navigation error:", error);
  }
}

async function executeJS() {
  const jsCode = jsInputEl.value.trim();

  if (!jsCode) {
    return;
  }

  try {
    await invoke("execute_js_in_webview", { jsCode: jsCode });
  } catch (error) {
    console.error("JS execution error:", error);
  }
}

function loadDefaultUrl() {
  urlInputEl.value = "https://alpha.wms.kakaostyle.com";
  loadUrl();
}

function reloadWebView() {
  loadUrl();
}

async function toggleDevTools() {
  try {
    await invoke("open_devtools");
  } catch (error) {
    console.error("DevTools error:", error);
  }
}

async function toggleAlwaysOnTop() {
  try {
    await invoke("set_always_on_top", { alwaysOnTop: alwaysOnTopCheckboxEl.checked });
  } catch (error) {
    console.error("Always on top error:", error);
  }
}

function hideLoadingScreen() {
  if (loadingScreen) {
    loadingScreen.style.display = "none";
  }
}

async function toggleMenu() {
  const isOpen = controlPanel.classList.contains("open");

  if (isOpen) {
    // 메뉴 닫기: 375px로 축소, 웹뷰 위치 유지 (0, 0)
    controlPanel.classList.remove("open");
    try {
      await invoke("resize_window", { width: 375, height: 667 });
    } catch (error) {
      console.error("Failed to close menu:", error);
    }
  } else {
    // 메뉴 열기: 725px로 확장, 웹뷰 위치 유지 (0, 0)
    controlPanel.classList.add("open");
    try {
      await invoke("resize_window", { width: 725, height: 667 });
    } catch (error) {
      console.error("Failed to open menu:", error);
    }
  }
}

async function closeMenu() {
  controlPanel.classList.remove("open");
  try {
    await invoke("resize_window", { width: 375, height: 667 });
  } catch (error) {
    console.error("Failed to close menu:", error);
  }
}

async function scanBarcode() {
  const barcode = barcodeInputEl.value.trim();

  if (!barcode) {
    return;
  }

  try {
    // 웹뷰에서 scanBarcode 함수 실행
    await invoke("execute_js_in_webview", {
      jsCode: `scanBarcode('${barcode}')`
    });
    barcodeInputEl.value = "";
    barcodeInputEl.focus();
  } catch (error) {
    console.error("Barcode scan error:", error);
  }
}

window.addEventListener("DOMContentLoaded", () => {
  urlInputEl = document.querySelector("#url-input");
  devtoolsCheckboxEl = document.querySelector("#devtools-checkbox");
  alwaysOnTopCheckboxEl = document.querySelector("#always-on-top-checkbox");
  jsInputEl = document.querySelector("#js-input");
  loadingScreen = document.querySelector("#loading-screen");
  controlPanel = document.querySelector("#control-panel");
  barcodeInputEl = document.querySelector("#barcode-input");

  // 메뉴 토글 이벤트
  document.querySelector("#menu-toggle").addEventListener("click", toggleMenu);
  document.querySelector("#menu-close").addEventListener("click", closeMenu);

  // Always on top 체크박스 이벤트
  alwaysOnTopCheckboxEl.addEventListener("change", toggleAlwaysOnTop);

  // 폼 이벤트
  document.querySelector("#webview-form").addEventListener("submit", (e) => {
    e.preventDefault();
    loadUrl();
  });

  document.querySelector("#execute-js-btn").addEventListener("click", (e) => {
    e.preventDefault();
    executeJS();
  });

  // 바코드 폼 이벤트
  document.querySelector("#barcode-form").addEventListener("submit", (e) => {
    e.preventDefault();
    scanBarcode();
  });

  // 퀵 액션 이벤트
  document
    .querySelector("#reload-btn")
    .addEventListener("click", reloadWebView);
  document
    .querySelector("#default-url-btn")
    .addEventListener("click", loadDefaultUrl);

  // 키보드 단축키
  document.addEventListener("keydown", (e) => {
    // F12: DevTools 토글
    if (e.key === "F12") {
      e.preventDefault();
      devtoolsCheckboxEl.checked = !devtoolsCheckboxEl.checked;
      if (devtoolsCheckboxEl.checked) {
        toggleDevTools();
      }
    }

    // Ctrl/Cmd + K: 메뉴 토글
    if ((e.ctrlKey || e.metaKey) && e.key === "k") {
      e.preventDefault();
      toggleMenu();
    }

    // ESC: 메뉴 닫기
    if (e.key === "Escape") {
      e.preventDefault();
      closeMenu();
    }
  });

  // 로딩 화면을 1초 후에 숨김
  setTimeout(() => {
    hideLoadingScreen();
  }, 1000);
});
