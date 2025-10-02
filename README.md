# Mobile WebView Preview App

A Tauri-based mobile webview controller for testing and debugging mobile web applications.

## Features

- **Mobile Device Simulation**: 375x667 viewport (iPhone SE size)
- **Customizable Default URL**: Set and persist your preferred testing URL
- **Barcode Scanner**: Input and execute barcode scan functions in the webview
- **Scan History**: Track up to 8 recent barcode scans (session-only)
- **Developer Tools**: Toggle DevTools with F12
- **Always on Top**: Keep the preview window above other applications
- **Slide-out Control Panel**: Access controls via settings button (⚙️)

## Keyboard Shortcuts

- **F12**: Toggle DevTools
- **Ctrl/Cmd + K**: Toggle control panel
- **Escape**: Close control panel

## Development

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/)
- [pnpm](https://pnpm.io/)

### Setup

```bash
# Install dependencies
pnpm install

# Run in development mode
pnpm tauri dev

# Build for production
pnpm tauri build
```

## Configuration

### Default URL

- Set via control panel UI
- Stored in localStorage
- Falls back to `https://alpha.wms.kakaostyle.com`

### Barcode Input

- Toggle visibility via control panel
- Executes `scanBarcode(value)` in webview
- Automatically adjusts webview height

## Project Structure

```
mobile-webview-app/
├── src/               # Frontend assets
│   ├── index.html     # Main UI
│   ├── main.js        # Application logic
│   └── styles.css     # Styling
└── src-tauri/         # Rust backend
    ├── src/
    │   └── lib.rs     # Tauri commands
    ├── icons/         # Application icons
    └── Cargo.toml     # Rust dependencies
```

## Technical Details

### Window Configuration

- **Main Window**: 375x667 (expandable to 725x667 when panel open)
- **Child Webview**: 375x617 (or 375x667 when barcode hidden)
- **Control Panel**: 350px width slide-out

### Storage

- **Default URL**: localStorage (`defaultUrl`)
- **Barcode Visibility**: localStorage (`showBarcodeInput`)
- **Scan History**: Session memory (cleared on close)

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
