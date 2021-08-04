/* vim: set sw=2 expandtab tw=80: */

#include <stdio.h>
#include "hello.h"

int main(void) {
  // printf ("Hello World!\r\n");
  // example_driver_action ();
  if (hello_is_present()) {
    printf ("The Hello driver is present\n");
  }
  else
  {
    printf ("The Hello driver is not present\n");
  }
  return 0;
}
