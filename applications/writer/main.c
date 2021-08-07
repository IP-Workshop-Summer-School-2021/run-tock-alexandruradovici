/* vim: set sw=2 expandtab tw=80: */

#include <ipc.h>
#include <stdio.h>
#include "timer.h"

int printer_service = -1;

char print_buf[64] __attribute__((aligned(64)));

int main(void) {
  int ret = ipc_discover("printer", &printer_service);
  if (ret != RETURNCODE_SUCCESS || printer_service < 0) {
    printf("No printer service\n");
    return -1;
  }
  printf ("share %d\n", ipc_share(printer_service, print_buf, 64));
  strcpy (print_buf, "12345");
  ipc_notify_service(printer_service);

  return 0;
}
