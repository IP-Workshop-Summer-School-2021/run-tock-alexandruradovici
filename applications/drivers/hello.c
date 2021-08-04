#include "hello.h"
#include "tock.h"

static unsigned int HELLO_DRIVER_NUM = 0xa0000;

bool hello_is_present (void) {
    syscall_return_t res = command (HELLO_DRIVER_NUM, 0, 0, 0);
    // if (res.type == TOCK_SYSCALL_SUCCESS) {
    //     return true;
    // }
    // else
    // {
    //     return false;
    // }
    return res.type == TOCK_SYSCALL_SUCCESS;
}