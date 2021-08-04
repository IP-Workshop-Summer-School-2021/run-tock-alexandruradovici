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

bool hello_print (void) {
    syscall_return_t res = command (HELLO_DRIVER_NUM, 1, 0, 0);
    return res.type == TOCK_SYSCALL_SUCCESS;
}

bool hello_up (void) {
    syscall_return_t res = command (HELLO_DRIVER_NUM, 2, 0, 0);
    return res.type == TOCK_SYSCALL_SUCCESS;
}

bool hello_down (void) {
    syscall_return_t res = command (HELLO_DRIVER_NUM, 3, 0, 0);
    return res.type == TOCK_SYSCALL_SUCCESS;
}

bool hello_set (unsigned int n) {
    syscall_return_t res = command (HELLO_DRIVER_NUM, 4, n, 0);
    return res.type == TOCK_SYSCALL_SUCCESS;
}

bool hello_get (unsigned int *n) {
    syscall_return_t res = command (HELLO_DRIVER_NUM, 5, 0, 0);
    if (res.type == TOCK_SYSCALL_SUCCESS_U32) {
        *n = res.data[0];
        return true;
    }
    else
    {
        return false;
    }
}