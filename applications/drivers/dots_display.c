#include "dots_display.h"
#include "tock.h"

bool display_digit (char digit) {
    syscall_return_t res = command (0xa0001, 1, digit, 0);
    return res.type == TOCK_SYSCALL_SUCCESS_U32;
}