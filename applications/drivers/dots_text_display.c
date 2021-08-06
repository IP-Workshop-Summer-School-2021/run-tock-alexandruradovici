#include "dots_text_display.h"
#include "tock.h"

#define DOTS_TEXT_DISPLAY_DRIVER_NUM 0xa0002

static void fn_done (__attribute__ ((unused)) int arg1, __attribute__ ((unused)) int arg2, __attribute__ ((unused)) int arg3, void * user_data) {
    bool *done = (bool*)user_data;
    *done = true;
}

bool display_text (const char *text) {
    // syscall_return_t res = command (0xa0001, 1, digit, 0);
    // return res.type == TOCK_SYSCALL_SUCCESS_U32;
    bool success = false;
    allow_ro_return_t ar = allow_readonly (DOTS_TEXT_DISPLAY_DRIVER_NUM, 0, text, strlen(text));
    if (ar.success) {
        bool done = false;
        subscribe_return_t sr = subscribe (DOTS_TEXT_DISPLAY_DRIVER_NUM, 0, fn_done, &done);
        if (sr.success) {
            syscall_return_t r = command (DOTS_TEXT_DISPLAY_DRIVER_NUM, 1, strlen(text), 1000);
            success = r.type == TOCK_SYSCALL_SUCCESS;
            yield_for (&done);
            // while (done != true) {
            //     yield ();
            // }
        }
    }
    allow_ro_return_t unallow = allow_readonly (DOTS_TEXT_DISPLAY_DRIVER_NUM, 0, NULL, 0);
    success = success & (unallow.ptr == text);
    return success;
}