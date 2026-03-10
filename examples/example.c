#include "tossicat.h"
#include <stdio.h>

int main() {
    // 1. 단어에 토시 붙이기
    printf("=== postfix 예시 ===\n");

    char* r1 = tossicat_postfix("포션", "을");
    if (r1) {
        printf("%s 획득했습니다!\n", r1);
        tossicat_free(r1);
    }

    char* r2 = tossicat_postfix("마나", "을");
    if (r2) {
        printf("%s 획득했습니다!\n", r2);
        tossicat_free(r2);
    }

    char* r3 = tossicat_postfix("서울", "으로");
    if (r3) {
        printf("%s 출발합니다.\n", r3);
        tossicat_free(r3);
    }

    char* r4 = tossicat_postfix("부산", "으로");
    if (r4) {
        printf("%s 출발합니다.\n", r4);
        tossicat_free(r4);
    }

    // 2. 문장 단위로 변환하기
    printf("\n=== modify_sentence 예시 ===\n");

    char* s1 = tossicat_modify_sentence(
        "{철수, 은} {영희, 과} {밥, 를} 먹습니다."
    );
    if (s1) {
        printf("%s\n", s1);
        tossicat_free(s1);
    }

    char* s2 = tossicat_modify_sentence(
        "{전사, 이} {검, 으로} {드래곤, 을} 공격합니다."
    );
    if (s2) {
        printf("%s\n", s2);
        tossicat_free(s2);
    }

    // 3. 종성 확인
    printf("\n=== guess_final_letter 예시 ===\n");

    char* f1 = tossicat_guess_final_letter("포션");
    if (f1) {
        printf("포션의 종성: %s\n", f1);
        tossicat_free(f1);
    }

    char* f2 = tossicat_guess_final_letter("마나");
    if (f2) {
        printf("마나의 종성: %s\n", f2);
        tossicat_free(f2);
    }

    return 0;
}
