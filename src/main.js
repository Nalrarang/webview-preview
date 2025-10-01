const { invoke } = window.__TAURI__.core;

let urlInputEl;
let devtoolsCheckboxEl;
let jsInputEl;
let statusMsgEl;

async function openWebView() {
  const url = urlInputEl.value.trim();
  const enableDevtools = devtoolsCheckboxEl.checked;
  
  if (!url) {
    showStatus("Please enter a URL", "error");
    return;
  }

  try {
    await invoke("open_webview", { 
      url: url, 
      enableDevtools: enableDevtools 
    });
    showStatus("WebView opened successfully!", "success");
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
    await invoke("execute_js", { jsCode: jsCode });
    showStatus("JavaScript executed successfully!", "success");
  } catch (error) {
    showStatus(`Error: ${error}`, "error");
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
  
  document.querySelector("#webview-form").addEventListener("submit", (e) => {
    e.preventDefault();
    openWebView();
  });
  
  document.querySelector("#execute-js-btn").addEventListener("click", (e) => {
    e.preventDefault();
    executeJS();
  });
});
