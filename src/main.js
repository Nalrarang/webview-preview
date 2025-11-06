const { invoke } = window.__TAURI__.core;

// Constants
const DEFAULT_URL_KEY = "defaultUrl";
const DEFAULT_FALLBACK_URL = "https://alpha.wms.kakaostyle.com";
const SHOW_BARCODE_KEY = "showBarcodeInput";
const DEVICE_MODE_KEY = "deviceMode";
const MAX_HISTORY_ITEMS = 8;

// DOM Elements
let urlInputEl;
let devtoolsCheckboxEl;
let alwaysOnTopCheckboxEl;
let showBarcodeCheckboxEl;
let loadingScreen;
let controlPanel;
let barcodeInputEl;
let barcodeContainer;
let barcodeHistoryList;
let defaultUrlDisplay;
let mobileModeBtn;
let desktopModeBtn;
let modeInfoEl;

// State
let barcodeHistory = []; // Session-only storage
let isMobileMode = true; // Default to mobile mode

// ============================================================================
// URL Management
// ============================================================================

async function loadUrl() {
  const url = urlInputEl.value.trim();
  if (!url) return;

  try {
    await invoke("navigate_to_url", { url });
    hideLoadingScreen();
  } catch (error) {
    console.error("Navigation error:", error);
  }
}

function getDefaultUrl() {
  return localStorage.getItem(DEFAULT_URL_KEY) || DEFAULT_FALLBACK_URL;
}

function updateDefaultUrlDisplay() {
  const savedUrl = localStorage.getItem(DEFAULT_URL_KEY);
  if (savedUrl) {
    defaultUrlDisplay.textContent = savedUrl;
    defaultUrlDisplay.style.color = "#0f0f0f";
  } else {
    defaultUrlDisplay.textContent = `${DEFAULT_FALLBACK_URL} (fallback)`;
    defaultUrlDisplay.style.color = "#999";
  }
}

function setDefaultUrl() {
  const currentUrl = urlInputEl.value.trim();
  if (!currentUrl) return;

  localStorage.setItem(DEFAULT_URL_KEY, currentUrl);
  updateDefaultUrlDisplay();
  alert(`Default URL set to:\n${currentUrl}`);
}

function clearDefaultUrl() {
  localStorage.removeItem(DEFAULT_URL_KEY);
  urlInputEl.value = DEFAULT_FALLBACK_URL;
  updateDefaultUrlDisplay();
  alert("Default URL cleared. Reset to fallback URL.");
}

function loadDefaultUrl() {
  urlInputEl.value = getDefaultUrl();
  loadUrl();
}

// ============================================================================
// DevTools & Window Controls
// ============================================================================

async function createNewWindow() {
  try {
    await invoke("create_new_window");
  } catch (error) {
    console.error("Failed to create new window:", error);
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

// ============================================================================
// Menu Controls
// ============================================================================

async function toggleMenu() {
  const isOpen = controlPanel.classList.contains("open");
  const newWidth = isOpen ? 375 : 725;

  if (isOpen) {
    controlPanel.classList.remove("open");
  } else {
    controlPanel.classList.add("open");
  }

  try {
    await invoke("resize_window", { width: newWidth, height: 667 });
  } catch (error) {
    console.error("Failed to toggle menu:", error);
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

// ============================================================================
// Device Mode Management
// ============================================================================

async function setDeviceMode(mobile) {
  const currentUrl = urlInputEl.value.trim() || "about:blank";

  try {
    // 웹뷰를 재생성하여 User-Agent 적용
    await invoke("set_user_agent", {
      isMobile: mobile,
      currentUrl: currentUrl
    });

    isMobileMode = mobile;
    localStorage.setItem(DEVICE_MODE_KEY, String(mobile));
    updateModeUI();
  } catch (error) {
    console.error("Failed to set device mode:", error);
    alert(`Failed to switch mode: ${error}`);
  }
}

function updateModeUI() {
  if (isMobileMode) {
    mobileModeBtn.classList.add("active");
    desktopModeBtn.classList.remove("active");
    modeInfoEl.textContent = "Current: Mobile (iPhone)";
  } else {
    mobileModeBtn.classList.remove("active");
    desktopModeBtn.classList.add("active");
    modeInfoEl.textContent = "Current: Desktop (macOS Chrome)";
  }
}

function loadDeviceMode() {
  const savedMode = localStorage.getItem(DEVICE_MODE_KEY);
  isMobileMode = savedMode === null ? true : savedMode === "true";
  updateModeUI();
}

// ============================================================================
// Barcode Input Management
// ============================================================================

async function toggleBarcodeInput() {
  const isVisible = showBarcodeCheckboxEl.checked;
  const webviewHeight = isVisible ? 617 : 667;

  localStorage.setItem(SHOW_BARCODE_KEY, String(isVisible));
  barcodeContainer.style.display = isVisible ? "flex" : "none";

  try {
    await invoke("resize_webview", { width: 375, height: webviewHeight });
  } catch (error) {
    console.error("Failed to resize webview:", error);
  }
}

async function loadBarcodeVisibility() {
  const savedState = localStorage.getItem(SHOW_BARCODE_KEY);
  const isVisible = savedState === null ? true : savedState === "true";
  const webviewHeight = isVisible ? 617 : 667;

  showBarcodeCheckboxEl.checked = isVisible;
  barcodeContainer.style.display = isVisible ? "flex" : "none";

  try {
    await invoke("resize_webview", { width: 375, height: webviewHeight });
  } catch (error) {
    console.error("Failed to resize webview on load:", error);
  }
}

// ============================================================================
// Barcode History
// ============================================================================

function addBarcodeToHistory(barcode) {
  const now = new Date();
  const timeString = now.toLocaleTimeString('en-US', {
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
    hour12: false
  });

  barcodeHistory.unshift({ barcode, time: timeString, timestamp: now.getTime() });

  if (barcodeHistory.length > MAX_HISTORY_ITEMS) {
    barcodeHistory = barcodeHistory.slice(0, MAX_HISTORY_ITEMS);
  }

  updateHistoryDisplay();
}

function updateHistoryDisplay() {
  if (barcodeHistory.length === 0) {
    barcodeHistoryList.innerHTML = '<p class="empty-history">No scans yet</p>';
    return;
  }

  barcodeHistoryList.innerHTML = barcodeHistory.map((item, index) => `
    <div class="history-item" data-index="${index}">
      <span class="history-item-barcode">${item.barcode}</span>
      <span class="history-item-time">${item.time}</span>
    </div>
  `).join('');

  // 히스토리 아이템 클릭 이벤트 추가
  const historyItems = barcodeHistoryList.querySelectorAll('.history-item');
  historyItems.forEach(item => {
    item.addEventListener('click', () => {
      const index = parseInt(item.getAttribute('data-index'));
      const historyItem = barcodeHistory[index];
      if (historyItem) {
        barcodeInputEl.value = historyItem.barcode;
        barcodeInputEl.focus();
      }
    });
  });
}

async function scanBarcode() {
  const barcode = barcodeInputEl.value.trim();
  if (!barcode) return;

  try {
    await invoke("execute_js_in_webview", { jsCode: `scanBarcode('${barcode}')` });
    addBarcodeToHistory(barcode);
    barcodeInputEl.value = "";
    barcodeInputEl.focus();
  } catch (error) {
    console.error("Barcode scan error:", error);
  }
}

// ============================================================================
// Initialization
// ============================================================================

window.addEventListener("DOMContentLoaded", () => {
  // Initialize DOM elements
  urlInputEl = document.querySelector("#url-input");
  devtoolsCheckboxEl = document.querySelector("#devtools-checkbox");
  alwaysOnTopCheckboxEl = document.querySelector("#always-on-top-checkbox");
  showBarcodeCheckboxEl = document.querySelector("#show-barcode-checkbox");
  loadingScreen = document.querySelector("#loading-screen");
  controlPanel = document.querySelector("#control-panel");
  barcodeInputEl = document.querySelector("#barcode-input");
  barcodeContainer = document.querySelector("#barcode-container");
  barcodeHistoryList = document.querySelector("#barcode-history-list");
  defaultUrlDisplay = document.querySelector("#default-url-display");
  mobileModeBtn = document.querySelector("#mobile-mode-btn");
  desktopModeBtn = document.querySelector("#desktop-mode-btn");
  modeInfoEl = document.querySelector("#mode-info");

  // Load saved state
  urlInputEl.value = getDefaultUrl();
  updateDefaultUrlDisplay();
  loadBarcodeVisibility();
  loadDeviceMode();

  // Menu controls
  document.querySelector("#menu-toggle").addEventListener("click", toggleMenu);
  document.querySelector("#menu-close").addEventListener("click", closeMenu);

  // Settings controls
  alwaysOnTopCheckboxEl.addEventListener("change", toggleAlwaysOnTop);
  showBarcodeCheckboxEl.addEventListener("change", toggleBarcodeInput);
  document.querySelector("#set-default-url-btn").addEventListener("click", setDefaultUrl);
  document.querySelector("#clear-default-url-btn").addEventListener("click", clearDefaultUrl);

  // Device mode controls
  mobileModeBtn.addEventListener("click", () => setDeviceMode(true));
  desktopModeBtn.addEventListener("click", () => setDeviceMode(false));

  // Form submissions
  document.querySelector("#webview-form").addEventListener("submit", (e) => {
    e.preventDefault();
    loadUrl();
  });

  document.querySelector("#barcode-form").addEventListener("submit", (e) => {
    e.preventDefault();
    scanBarcode();
  });

  // Quick actions
  document.querySelector("#new-window-btn").addEventListener("click", createNewWindow);
  document.querySelector("#reload-btn").addEventListener("click", loadUrl);
  document.querySelector("#default-url-btn").addEventListener("click", loadDefaultUrl);

  // Keyboard shortcuts
  document.addEventListener("keydown", (e) => {
    if ((e.ctrlKey || e.metaKey) && e.key === "k") {
      e.preventDefault();
      toggleMenu();
    }

    if ((e.ctrlKey || e.metaKey) && e.key === "n") {
      e.preventDefault();
      createNewWindow();
    }

    if (e.key === "Escape") {
      e.preventDefault();
      closeMenu();
    }
  });

  // Initial navigation and loading
  loadUrl();
  setTimeout(hideLoadingScreen, 1000);
});
