//! # TossiCat C FFI
//!
//! tossicat-core의 C FFI 바인딩입니다.
//! 게임 엔진(Unreal, Unity, Godot 등)이나 C/C++ 프로젝트에서
//! 한국어 조사(토시) 변환 기능을 사용할 수 있게 해줍니다.
//!
//! ## 사용 예시 (C)
//!
//! ```c
//! #include "tossicat.h"
//!
//! const char* result = tossicat_postfix("포션", "을");
//! // result: "포션을"
//! printf("%s 획득했습니다!\n", result);
//! tossicat_free(result);
//! ```

#[cfg(feature = "source-crates-io")]
use tossicat_crates as tossicat;
#[cfg(feature = "source-github")]
use tossicat_git as tossicat;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

/// ## 입력된 토시를 단어에 맞게 변환해, 단어와 합쳐 반환하는 함수
///
/// C에서 사용하는 `postfix()` 함수입니다.
/// 반환된 문자열은 반드시 `tossicat_free()`로 해제해야 합니다.
///
/// ### 매개변수
/// - `word`: 토시를 붙일 단어 (UTF-8 인코딩 C 문자열)
/// - `tossi`: 붙일 토시 (UTF-8 인코딩 C 문자열)
///
/// ### 반환값
/// - 성공: 변환된 문자열 (단어 + 변환된 토시)
/// - 실패: NULL
///
/// ### 사용 예시 (C)
/// ```c
/// const char* result = tossicat_postfix("검", "을");
/// // result: "검을"
/// tossicat_free(result);
///
/// const char* result2 = tossicat_postfix("포션", "을");
/// // result2: "포션을"
/// tossicat_free(result2);
/// ```
#[no_mangle]
pub extern "C" fn tossicat_postfix(word: *const c_char, tossi: *const c_char) -> *mut c_char {
    if word.is_null() || tossi.is_null() {
        return ptr::null_mut();
    }

    let word_str = match unsafe { CStr::from_ptr(word) }.to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let tossi_str = match unsafe { CStr::from_ptr(tossi) }.to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    match tossicat::postfix(word_str, tossi_str) {
        Ok(result) => match CString::new(result) {
            Ok(c_result) => c_result.into_raw(),
            Err(_) => ptr::null_mut(),
        },
        Err(_) => ptr::null_mut(),
    }
}

/// ## 변경할 토시가 여러 개 들어 있는 문장을 적절한 토시로 변경하는 함수
///
/// C에서 사용하는 `modify_sentence()` 함수입니다.
/// 반환된 문자열은 반드시 `tossicat_free()`로 해제해야 합니다.
///
/// ### 매개변수
/// - `sentence`: 변환할 문장 (UTF-8 인코딩 C 문자열)
///   - 형식: `"{단어, 토시} 문장"`
///
/// ### 반환값
/// - 성공: 변환된 문장
/// - 실패: NULL
///
/// ### 사용 예시 (C)
/// ```c
/// const char* result = tossicat_modify_sentence(
///     "{철수, 은} {영희, 과} {밥, 를} 먹습니다."
/// );
/// // result: "철수는 영희와 밥을 먹습니다."
/// tossicat_free(result);
/// ```
#[no_mangle]
pub extern "C" fn tossicat_modify_sentence(sentence: *const c_char) -> *mut c_char {
    if sentence.is_null() {
        return ptr::null_mut();
    }

    let sentence_str = match unsafe { CStr::from_ptr(sentence) }.to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    match tossicat::modify_sentence(sentence_str) {
        Ok(result) => match CString::new(result) {
            Ok(c_result) => c_result.into_raw(),
            Err(_) => ptr::null_mut(),
        },
        Err(_) => ptr::null_mut(),
    }
}

/// ## 입력된 문자열에서 마지막 글자의 종성을 반환하는 함수
///
/// ### 매개변수
/// - `word`: 분석할 단어 (UTF-8 인코딩 C 문자열)
///
/// ### 반환값
/// - 종성이 있는 경우: 해당 종성의 UTF-8 인코딩 문자열
/// - 종성이 없는 경우: " " (공백)
/// - 한글이 아닌 경우: "N"
/// - 실패: NULL
///
/// 반환된 문자열은 반드시 `tossicat_free()`로 해제해야 합니다.
#[no_mangle]
pub extern "C" fn tossicat_guess_final_letter(word: *const c_char) -> *mut c_char {
    if word.is_null() {
        return ptr::null_mut();
    }

    let word_str = match unsafe { CStr::from_ptr(word) }.to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let result = tossicat::guess_final_letter(word_str);

    match CString::new(result.to_string()) {
        Ok(c_result) => c_result.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

/// ## tossicat 함수들이 반환한 문자열을 해제하는 함수
///
/// `tossicat_postfix()`, `tossicat_modify_sentence()`,
/// `tossicat_guess_final_letter()`가 반환한 문자열을
/// 이 함수로 반드시 해제해야 합니다.
///
/// ### 매개변수
/// - `s`: 해제할 문자열 포인터. NULL이면 무시합니다.
///
/// ### 사용 예시 (C)
/// ```c
/// const char* result = tossicat_postfix("검", "을");
/// printf("%s\n", result);
/// tossicat_free(result);  // 반드시 해제
/// ```
#[no_mangle]
pub extern "C" fn tossicat_free(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}
