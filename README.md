# Mobile WebView Preview App

Tauri 기반의 모바일 웹뷰 컨트롤러로 모바일 웹 애플리케이션의 테스트 및 디버깅을 위한 도구입니다.
이 도구는 순전히 개입없이 Claude Code를 통해서만 개발되었습니다.

## 주요 기능

- **모바일 기기 시뮬레이션**: 375x667 뷰포트 (iPhone SE 크기)
- **커스터마이징 가능한 기본 URL**: 선호하는 테스트 URL 설정 및 저장
- **바코드 스캐너**: 웹뷰에서 바코드 스캔 함수 입력 및 실행
- **스캔 히스토리**: 최근 바코드 스캔 8개 추적 (세션 한정)
- **개발자 도구**: F12로 DevTools 토글
- **Always on Top**: 다른 애플리케이션 위에 프리뷰 윈도우 고정
- **슬라이드 아웃 컨트롤 패널**: 설정 버튼(⚙️)을 통해 컨트롤 접근

## 키보드 단축키

- **F12**: DevTools 토글
- **Ctrl/Cmd + K**: 컨트롤 패널 토글
- **Escape**: 컨트롤 패널 닫기

## 개발 환경 설정

### 사전 요구사항

- [Rust](https://www.rust-lang.org/tools/install)

### 설치 방법

```bash
# 1. Rust 설치
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. 터미널 재시작 또는 환경변수 로드
source $HOME/.cargo/env

# 3. Tauri CLI 설치
cargo install tauri-cli

# 4. 개발 모드 실행
cargo tauri dev

# 5. 프로덕션 빌드
cargo tauri build
```

## 설정

### 기본 URL

- 컨트롤 패널 UI를 통해 설정
- localStorage에 저장
- 기본값: `https://alpha.wms.kakaostyle.com`

### 바코드 입력

- 컨트롤 패널을 통해 표시/숨김 토글
- 웹뷰에서 `scanBarcode(value)` 실행
- 자동으로 웹뷰 높이 조정

## 프로젝트 구조

```
mobile-webview-app/
├── src/               # 프론트엔드 리소스
│   ├── index.html     # 메인 UI
│   ├── main.js        # 애플리케이션 로직
│   └── styles.css     # 스타일링
└── src-tauri/         # Rust 백엔드
    ├── src/
    │   └── lib.rs     # Tauri 명령어
    ├── icons/         # 애플리케이션 아이콘
    └── Cargo.toml     # Rust 의존성
```

## 기술 세부사항

### 윈도우 설정

- **메인 윈도우**: 375x667 (패널 열림 시 725x667로 확장)
- **자식 웹뷰**: 375x617 (바코드 숨김 시 375x667)
- **컨트롤 패널**: 350px 너비 슬라이드 아웃

### 저장소

- **기본 URL**: localStorage (`defaultUrl`)
- **바코드 표시 여부**: localStorage (`showBarcodeInput`)
- **스캔 히스토리**: 세션 메모리 (종료 시 삭제)

## 권장 IDE 설정

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
