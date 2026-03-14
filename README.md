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

- 토시를 추가하면 개발 중인 [tossicat-core](https://github.com/tossicat/tossicat-core)을 직접 사용할 수 있습니다.
- 게임에서의 활용: 한국어 게임에서 아이템명이나 캐릭터명에 따라 조사를 자동으로 붙일 수 있습니다.

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
| `tossicat_free(ptr)` | 반환된 문자열 메모리 해제 |

## 주의사항

- 모든 문자열은 **UTF-8 인코딩**이어야 합니다.
- `tossicat_postfix()`, `tossicat_modify_sentence()`, `tossicat_guess_final_letter()`가
  반환한 문자열은 **반드시 `tossicat_free()`로 해제**해야 합니다.
- NULL 포인터를 입력하면 NULL을 반환합니다.

## 의존성

- 개발 중: [tossicat-core](https://github.com/tossicat/tossicat-core) (GitHub dev 브랜치)
- 배포 후: [tossicat](https://crates.io/crates/tossicat) 0.8+

## 라이선스

MIT
