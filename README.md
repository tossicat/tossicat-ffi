# TossiCat FFI

[tossicat-core](https://github.com/tossicat/tossicat-core)의 C FFI 바인딩입니다.
게임 엔진(Unreal, Unity, Godot 등)이나 C/C++ 프로젝트에서
한국어 조사(토시) 변환 기능을 사용할 수 있게 해줍니다.

## 빌드

```bash
cargo build --release
```

빌드하면 다음 파일들이 생성됩니다:

- `include/tossicat.h` — C 헤더 파일
- `target/release/libtossicat_ffi.so` (Linux)
- `target/release/libtossicat_ffi.dylib` (macOS)
- `target/release/tossicat_ffi.dll` (Windows)

## 사용법 (C)

```c
#include "tossicat.h"
#include <stdio.h>

int main() {
    // 단어에 토시 붙이기
    char* result = tossicat_postfix("포션", "을");
    if (result) {
        printf("%s 획득했습니다!\n", result);  // "포션을 획득했습니다!"
        tossicat_free(result);
    }

    // 문장 단위로 변환하기
    char* sentence = tossicat_modify_sentence(
        "{철수, 은} {영희, 과} {밥, 를} 먹습니다."
    );
    if (sentence) {
        printf("%s\n", sentence);  // "철수는 영희와 밥을 먹습니다."
        tossicat_free(sentence);
    }

    return 0;
}
```

### 컴파일 (Linux)

```bash
gcc -o example example.c -L target/release -ltossicat_ffi
```

### 컴파일 (macOS)

```bash
gcc -o example example.c -L target/release -ltossicat_ffi
```

## 이 프로젝트의 장점

- **의존성 소스 선택 가능**: Cargo features를 통해 crates.io 안정 버전과 GitHub dev 브랜치 중 선택할 수 있습니다.
  ```bash
  # crates.io 안정 버전 (기본값)
  cargo build --release

  # GitHub dev 브랜치 (최신 개발 버전)
  cargo build --release --no-default-features --features source-github
  ```
- **게임에서의 활용**: 한국어 게임에서 아이템명이나 캐릭터명에 따라 조사를 자동으로 붙일 수 있습니다.

```c
// 게임 아이템 획득 메시지
void show_item_message(const char* item_name) {
    // "{아이템명, 을} 획득했습니다!" 형식으로 문장 생성
    char template[256];
    snprintf(template, sizeof(template), "{%s, 을} 획득했습니다!", item_name);

    char* message = tossicat_modify_sentence(template);
    if (message) {
        show_ui_text(message);  // 게임 UI에 표시
        tossicat_free(message);
    }
}

// show_item_message("포션");   → "포션을 획득했습니다!"
// show_item_message("검");     → "검을 획득했습니다!"
// show_item_message("활");     → "활을 획득했습니다!"
// show_item_message("마나");   → "마나를 획득했습니다!"
```

## API 목록

| 함수 | 설명 |
|------|------|
| `tossicat_postfix(word, tossi)` | 단어에 토시를 붙여 반환 |
| `tossicat_modify_sentence(sentence)` | 문장 내 여러 토시를 일괄 변환 |
| `tossicat_guess_final_letter(word)` | 단어의 마지막 글자 종성 반환 |
| `tossicat_last_error()` | 마지막 에러 메시지 반환 (해제 불필요) |
| `tossicat_free(ptr)` | 반환된 문자열 메모리 해제 |

## 주의사항

- 모든 문자열은 **UTF-8 인코딩**이어야 합니다.
- `tossicat_postfix()`, `tossicat_modify_sentence()`, `tossicat_guess_final_letter()`가
  반환한 문자열은 **반드시 `tossicat_free()`로 해제**해야 합니다.
- NULL 포인터를 입력하면 NULL을 반환합니다.

## CI

[![CI](https://github.com/tossicat/tossicat-ffi/actions/workflows/ci.yml/badge.svg)](https://github.com/tossicat/tossicat-ffi/actions/workflows/ci.yml)

`main` 브랜치에 push하거나 PR을 올리면 GitHub Actions를 통해 자동으로 테스트와 빌드가 실행됩니다.

- **Linux**, **macOS**, **Windows** 3개 플랫폼에서 병렬 실행
- `cargo test` — 단위 테스트 자동 실행
- `cargo build --release` — 릴리스 빌드 및 산출물 artifact 업로드

결과는 [Actions 탭](https://github.com/tossicat/tossicat-ffi/actions)에서 확인할 수 있습니다.

## 의존성

- 기본: [tossicat](https://crates.io/crates/tossicat) 0.7 (crates.io)
- 선택: [tossicat-core](https://github.com/tossicat/tossicat-core) dev 브랜치 (`--features source-github`)

## 활용 방법

빌드된 라이브러리(`libtossicat_ffi.so`/`.dylib`/`.dll`)와 헤더 파일(`include/tossicat.h`)을 프로젝트에 복사하여 사용합니다.

### C/C++

헤더를 포함하고 라이브러리를 링크합니다.

```bash
gcc -o myapp myapp.c -I include -L target/release -ltossicat_ffi
```

실행 시 동적 라이브러리 경로를 지정합니다.

```bash
# Linux
LD_LIBRARY_PATH=target/release ./myapp

# macOS
DYLD_LIBRARY_PATH=target/release ./myapp
```

### Unreal Engine (C++)

1. 빌드된 라이브러리를 `Plugins/TossiCat/Binaries/` 에 복사합니다.
2. `tossicat.h`를 `Plugins/TossiCat/Source/` 에 복사합니다.
3. `.Build.cs`에서 라이브러리를 링크합니다.

```cpp
#include "tossicat.h"

FString GetItemMessage(const FString& ItemName) {
    FString Template = FString::Printf(TEXT("{%s, 을} 획득했습니다!"), *ItemName);
    char* Result = tossicat_modify_sentence(TCHAR_TO_UTF8(*Template));
    if (Result) {
        FString Message = UTF8_TO_TCHAR(Result);
        tossicat_free(Result);
        return Message;
    }
    return TEXT("");
}
```

### Unity (C#)

```csharp
using System.Runtime.InteropServices;

public static class TossiCat {
    [DllImport("tossicat_ffi")]
    private static extern IntPtr tossicat_postfix(string word, string tossi);

    [DllImport("tossicat_ffi")]
    private static extern IntPtr tossicat_modify_sentence(string sentence);

    [DllImport("tossicat_ffi")]
    private static extern void tossicat_free(IntPtr ptr);

    public static string Postfix(string word, string tossi) {
        IntPtr ptr = tossicat_postfix(word, tossi);
        if (ptr == IntPtr.Zero) return null;
        string result = Marshal.PtrToStringUTF8(ptr);
        tossicat_free(ptr);
        return result;
    }

    public static string ModifySentence(string sentence) {
        IntPtr ptr = tossicat_modify_sentence(sentence);
        if (ptr == IntPtr.Zero) return null;
        string result = Marshal.PtrToStringUTF8(ptr);
        tossicat_free(ptr);
        return result;
    }
}

// 사용 예시
// string msg = TossiCat.Postfix("포션", "을");  // "포션을"
```

### Godot (GDScript + GDExtension)

GDExtension C API를 통해 바인딩하거나, GDNative를 사용합니다.

```gdscript
# gdextension으로 래핑한 경우
var result = TossiCat.postfix("포션", "을")
print(result)  # "포션을"
```

## 라이선스

MIT
