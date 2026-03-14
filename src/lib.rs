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
//! char* result = tossicat_postfix("포션", "을");
//! if (result) {
//!     printf("%s 획득했습니다!\n", result);
//!     tossicat_free(result);
//! } else {
//!     const char* err = tossicat_last_error();
//!     printf("에러: %s\n", err);
//! }
//! ```

#[cfg(feature = "source-crates-io")]
use tossicat_crates as tossicat;
#[cfg(feature = "source-github")]
use tossicat_git as tossicat;

use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

thread_local! {
    static LAST_ERROR: RefCell<CString> = RefCell::new(CString::default());
}

/// 에러 메시지를 thread-local 변수에 저장
fn set_last_error(msg: &str) {
    LAST_ERROR.with(|e| {
        *e.borrow_mut() = CString::new(msg).unwrap_or_default();
    });
}

/// 에러 상태를 초기화
fn clear_last_error() {
    LAST_ERROR.with(|e| {
        *e.borrow_mut() = CString::default();
    });
}

/// ## 마지막 에러 메시지를 반환하는 함수
///
/// 에러가 없으면 빈 문자열을 반환합니다.
/// 반환된 포인터는 다음 tossicat 함수 호출 전까지만 유효합니다.
/// `tossicat_free()`로 해제하지 마세요.
///
/// ### 사용 예시 (C)
/// ```c
/// char* result = tossicat_postfix(word, tossi);
/// if (!result) {
///     const char* err = tossicat_last_error();
///     printf("에러: %s\n", err);
/// }
/// ```
#[no_mangle]
pub extern "C" fn tossicat_last_error() -> *const c_char {
    LAST_ERROR.with(|e| e.borrow().as_ptr())
}

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
/// - 실패: NULL (`tossicat_last_error()`로 원인 조회)
///
/// ### 사용 예시 (C)
/// ```c
/// char* result = tossicat_postfix("검", "을");
/// // result: "검을"
/// tossicat_free(result);
///
/// char* result2 = tossicat_postfix("포션", "을");
/// // result2: "포션을"
/// tossicat_free(result2);
/// ```
#[no_mangle]
pub extern "C" fn tossicat_postfix(word: *const c_char, tossi: *const c_char) -> *mut c_char {
    clear_last_error();

    if word.is_null() || tossi.is_null() {
        set_last_error("NULL 포인터가 전달되었습니다");
        return ptr::null_mut();
    }

    let word_str = match unsafe { CStr::from_ptr(word) }.to_str() {
        Ok(s) => s,
        Err(_) => {
            set_last_error("word: 잘못된 UTF-8 인코딩입니다");
            return ptr::null_mut();
        }
    };

    let tossi_str = match unsafe { CStr::from_ptr(tossi) }.to_str() {
        Ok(s) => s,
        Err(_) => {
            set_last_error("tossi: 잘못된 UTF-8 인코딩입니다");
            return ptr::null_mut();
        }
    };

    match tossicat::postfix(word_str, tossi_str) {
        Ok(result) => match CString::new(result) {
            Ok(c_result) => c_result.into_raw(),
            Err(e) => {
                set_last_error(&format!("결과 문자열 변환 실패: {}", e));
                ptr::null_mut()
            }
        },
        Err(e) => {
            set_last_error(&format!("postfix 변환 실패: {}", e));
            ptr::null_mut()
        }
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
/// - 실패: NULL (`tossicat_last_error()`로 원인 조회)
///
/// ### 사용 예시 (C)
/// ```c
/// char* result = tossicat_modify_sentence(
///     "{철수, 은} {영희, 과} {밥, 를} 먹습니다."
/// );
/// // result: "철수는 영희와 밥을 먹습니다."
/// tossicat_free(result);
/// ```
#[no_mangle]
pub extern "C" fn tossicat_modify_sentence(sentence: *const c_char) -> *mut c_char {
    clear_last_error();

    if sentence.is_null() {
        set_last_error("NULL 포인터가 전달되었습니다");
        return ptr::null_mut();
    }

    let sentence_str = match unsafe { CStr::from_ptr(sentence) }.to_str() {
        Ok(s) => s,
        Err(_) => {
            set_last_error("sentence: 잘못된 UTF-8 인코딩입니다");
            return ptr::null_mut();
        }
    };

    match tossicat::modify_sentence(sentence_str) {
        Ok(result) => match CString::new(result) {
            Ok(c_result) => c_result.into_raw(),
            Err(e) => {
                set_last_error(&format!("결과 문자열 변환 실패: {}", e));
                ptr::null_mut()
            }
        },
        Err(e) => {
            set_last_error(&format!("modify_sentence 변환 실패: {}", e));
            ptr::null_mut()
        }
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
/// - 실패: NULL (`tossicat_last_error()`로 원인 조회)
///
/// 반환된 문자열은 반드시 `tossicat_free()`로 해제해야 합니다.
#[no_mangle]
pub extern "C" fn tossicat_guess_final_letter(word: *const c_char) -> *mut c_char {
    clear_last_error();

    if word.is_null() {
        set_last_error("NULL 포인터가 전달되었습니다");
        return ptr::null_mut();
    }

    let word_str = match unsafe { CStr::from_ptr(word) }.to_str() {
        Ok(s) => s,
        Err(_) => {
            set_last_error("word: 잘못된 UTF-8 인코딩입니다");
            return ptr::null_mut();
        }
    };

    let result = tossicat::guess_final_letter(word_str);

    match CString::new(result.to_string()) {
        Ok(c_result) => c_result.into_raw(),
        Err(e) => {
            set_last_error(&format!("결과 문자열 변환 실패: {}", e));
            ptr::null_mut()
        }
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
/// char* result = tossicat_postfix("검", "을");
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    /// CStr 포인터에서 &str로 변환하는 헬퍼
    unsafe fn ptr_to_str(ptr: *const c_char) -> &'static str {
        unsafe { CStr::from_ptr(ptr) }.to_str().unwrap()
    }

    /// 결과 포인터를 String으로 변환하고 메모리를 해제하는 헬퍼
    unsafe fn take_result(ptr: *mut c_char) -> String {
        assert!(!ptr.is_null());
        let s = unsafe { CStr::from_ptr(ptr) }
            .to_str()
            .unwrap()
            .to_owned();
        tossicat_free(ptr);
        s
    }

    // === postfix 테스트 ===

    #[test]
    fn postfix_받침있는_단어() {
        let word = CString::new("포션").unwrap();
        let tossi = CString::new("을").unwrap();
        let result = tossicat_postfix(word.as_ptr(), tossi.as_ptr());
        assert_eq!(unsafe { take_result(result) }, "포션을");
    }

    #[test]
    fn postfix_받침없는_단어() {
        let word = CString::new("마나").unwrap();
        let tossi = CString::new("을").unwrap();
        let result = tossicat_postfix(word.as_ptr(), tossi.as_ptr());
        assert_eq!(unsafe { take_result(result) }, "마나를");
    }

    #[test]
    fn postfix_null_word() {
        let tossi = CString::new("을").unwrap();
        let result = tossicat_postfix(ptr::null(), tossi.as_ptr());
        assert!(result.is_null());
        assert!(unsafe { ptr_to_str(tossicat_last_error()) }.contains("NULL"));
    }

    #[test]
    fn postfix_null_tossi() {
        let word = CString::new("검").unwrap();
        let result = tossicat_postfix(word.as_ptr(), ptr::null());
        assert!(result.is_null());
        assert!(unsafe { ptr_to_str(tossicat_last_error()) }.contains("NULL"));
    }

    #[test]
    fn postfix_빈_문자열() {
        let word = CString::new("").unwrap();
        let tossi = CString::new("을").unwrap();
        let result = tossicat_postfix(word.as_ptr(), tossi.as_ptr());
        // 빈 문자열이라도 크래시 없이 처리되어야 함
        if !result.is_null() {
            tossicat_free(result);
        }
    }

    // === modify_sentence 테스트 ===

    #[test]
    fn modify_sentence_기본() {
        let sentence = CString::new("{철수, 은} {영희, 과} {밥, 를} 먹습니다.").unwrap();
        let result = tossicat_modify_sentence(sentence.as_ptr());
        assert_eq!(unsafe { take_result(result) }, "철수는 영희와 밥을 먹습니다.");
    }

    #[test]
    fn modify_sentence_null() {
        let result = tossicat_modify_sentence(ptr::null());
        assert!(result.is_null());
        assert!(unsafe { ptr_to_str(tossicat_last_error()) }.contains("NULL"));
    }

    #[test]
    fn modify_sentence_토시_없는_문장() {
        let sentence = CString::new("일반 문장입니다.").unwrap();
        let result = tossicat_modify_sentence(sentence.as_ptr());
        if !result.is_null() {
            assert_eq!(unsafe { take_result(result) }, "일반 문장입니다.");
        }
    }

    // === guess_final_letter 테스트 ===

    #[test]
    fn guess_final_letter_종성있음() {
        let word = CString::new("포션").unwrap();
        let result = tossicat_guess_final_letter(word.as_ptr());
        assert!(!result.is_null());
        tossicat_free(result);
    }

    #[test]
    fn guess_final_letter_종성없음() {
        let word = CString::new("마나").unwrap();
        let result = tossicat_guess_final_letter(word.as_ptr());
        assert!(!result.is_null());
        tossicat_free(result);
    }

    #[test]
    fn guess_final_letter_null() {
        let result = tossicat_guess_final_letter(ptr::null());
        assert!(result.is_null());
        assert!(unsafe { ptr_to_str(tossicat_last_error()) }.contains("NULL"));
    }

    // === tossicat_free 테스트 ===

    #[test]
    fn free_null_안전() {
        tossicat_free(ptr::null_mut()); // 크래시 없어야 함
    }

    // === tossicat_last_error 테스트 ===

    #[test]
    fn last_error_성공시_초기화() {
        // 먼저 에러를 발생시킴
        tossicat_postfix(ptr::null(), ptr::null());
        assert!(!unsafe { ptr_to_str(tossicat_last_error()) }.is_empty());

        // 성공 호출 후 에러가 초기화되는지 확인
        let word = CString::new("검").unwrap();
        let tossi = CString::new("을").unwrap();
        let result = tossicat_postfix(word.as_ptr(), tossi.as_ptr());
        if !result.is_null() {
            assert!(unsafe { ptr_to_str(tossicat_last_error()) }.is_empty());
            tossicat_free(result);
        }
    }
}
