#include "dots_text_display.h"
#include "tock.h"

#define DOTS_TEXT_DISPLAY_DRIVER_NUM 0xa0002

static void fn_done (int arg1, int arg2, int arg3, void * user_data) {

}

bool display_text (const char *digit) {
    // syscall_return_t res = command (0xa0001, 1, digit, 0);
    // return res.type == TOCK_SYSCALL_SUCCESS_U32;
    subscribe_return_t sr = subscribe (DOTS_TEXT_DISPLAY_DRIVER_NUM, 0, fn_done, 0);
    return sr.success;
}