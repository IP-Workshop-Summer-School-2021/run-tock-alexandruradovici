#include "dots_text_display.h"
#include "tock.h"

#define DOTS_TEXT_DISPLAY_DRIVER_NUM 0xa0002

bool display_set_speed (unsigned int ms) {
    syscall_return_t rs = command(DOTS_TEXT_DISPLAY_DRIVER_NUM, 1, ms, 0);
    return rs.type == TOCK_SYSCALL_SUCCESS;
}

bool display_get_speed (unsigned int *ms) {
    syscall_return_t rs = command(DOTS_TEXT_DISPLAY_DRIVER_NUM, 2, 0, 0);
    if (rs.type == TOCK_SYSCALL_SUCCESS_U32) {
        *ms = rs.data[0];
        return true;
    }
    else 
    {
        return false;
    }
}

bool display_is_present (void) {
    syscall_return_t rs = command(DOTS_TEXT_DISPLAY_DRIVER_NUM, 0, 0, 0);
    return rs.type == TOCK_SYSCALL_SUCCESS;
}
