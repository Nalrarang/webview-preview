const { invoke } = window.__TAURI__.core;

let urlInputEl;
let devtoolsCheckboxEl;
let jsInputEl;
let statusMsgEl;
let loadingScreen;

async function loadUrl() {
  const url = urlInputEl.value.trim();

  if (!url) {
    showStatus("Please enter a URL", "error");
    return;
  }

  try {
    await invoke("navigate_to_url", { url: url });
    showStatus("Navigated successfully!", "success");
    hideLoadingScreen();
  } catch (error) {
    showStatus(`Error: ${error}`, "error");
  }
}

async function executeJS() {
  const jsCode = jsInputEl.value.trim();

  if (!jsCode) {
    showStatus("Please enter JavaScript code", "error");
    return;
  }

  try {
    await invoke("execute_js_in_webview", { jsCode: jsCode });
    showStatus("JavaScript executed successfully!", "success");
  } catch (error) {
    showStatus(`Error: ${error}`, "error");
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
    showStatus("DevTools opened", "success");
  } catch (error) {
    showStatus(`Error: ${error}`, "error");
  }
}

function hideLoadingScreen() {
  if (loadingScreen) {
    loadingScreen.style.display = "none";
  }
}

function showStatus(message, type) {
  statusMsgEl.textContent = message;
  statusMsgEl.className = type;

  setTimeout(() => {
    statusMsgEl.textContent = "";
    statusMsgEl.className = "";
  }, 3000);
}

window.addEventListener("DOMContentLoaded", () => {
  urlInputEl = document.querySelector("#url-input");
  devtoolsCheckboxEl = document.querySelector("#devtools-checkbox");
  jsInputEl = document.querySelector("#js-input");
  statusMsgEl = document.querySelector("#status-msg");
  loadingScreen = document.querySelector("#loading-screen");

  // 폼 이벤트
  document.querySelector("#webview-form").addEventListener("submit", (e) => {
    e.preventDefault();
    loadUrl();
  });

  document.querySelector("#execute-js-btn").addEventListener("click", (e) => {
    e.preventDefault();
    executeJS();
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
      } else {
        showStatus("DevTools disabled", "success");
      }
    }
  });

  // 로딩 화면을 1초 후에 숨김
  setTimeout(() => {
    hideLoadingScreen();
    showStatus("WebView ready", "success");
  }, 1000);
});
