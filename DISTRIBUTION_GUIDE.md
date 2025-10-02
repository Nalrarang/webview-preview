# 배포 가이드

## 🔒 보안 경고 해결 방법

macOS에서 "손상되어 열 수 없습니다" 경고가 나타나는 경우:

### 방법 1: 우클릭으로 열기 (권장)

1. `mobile-webview-preview.app` 파일을 **우클릭**
2. **"열기"** 선택
3. 경고창에서 **"열기"** 버튼 클릭
4. 이후부터는 정상적으로 더블클릭으로 실행 가능

### 방법 2: 터미널 명령어

```bash
# 앱을 Applications 폴더로 이동한 후
xattr -cr /Applications/mobile-webview-preview.app
```

### 방법 3: 시스템 설정

1. **시스템 설정** → **개인정보 보호 및 보안**
2. 하단의 "확인되지 않은 개발자" 메시지 확인
3. **"확인 없이 열기"** 클릭

## 📦 배포 방법

### 1. Google Drive / Dropbox
- `mobile-webview-preview.zip` 업로드
- 공유 링크 생성 및 전달

### 2. 이메일
- ZIP 파일 첨부 (3.7MB)

### 3. GitHub Releases
- Repository의 Releases 섹션에 업로드

## 🔑 키보드 단축키

- **F12**: DevTools 토글
- **Cmd+K**: 컨트롤 패널 토글
- **Cmd+N**: 새 창 생성
- **ESC**: 컨트롤 패널 닫기

## ❓ 자주 묻는 질문

### Q: 왜 보안 경고가 나타나나요?
A: 앱이 Apple Developer 계정으로 서명되지 않았기 때문입니다. 위의 방법으로 해결할 수 있습니다.

### Q: 여러 창을 어떻게 만드나요?
A: Cmd+N을 누르거나 컨트롤 패널의 "New Window" 버튼을 클릭하세요.

### Q: 기본 URL을 어떻게 설정하나요?
A: 컨트롤 패널에서 URL을 입력하고 "Set as Default URL" 버튼을 클릭하세요.

## 🛠️ 코드 서명 (개발자용)

Apple Developer 계정이 있는 경우:

```bash
# 1. 인증서 확인
security find-identity -v -p codesigning

# 2. 앱 서명
codesign --deep --force --verify --verbose --sign "Developer ID Application: Your Name" mobile-webview-preview.app

# 3. 공증 (선택사항)
xcrun notarytool submit mobile-webview-preview.zip --apple-id "your@email.com" --team-id "TEAMID" --password "app-specific-password"
```

서명된 앱은 보안 경고 없이 실행됩니다.
