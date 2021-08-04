/* vim: set sw=2 expandtab tw=80: */

#include <stdio.h>
#include "hello.h"

int main(void) {
  // printf ("Hello World!\r\n");
  // example_driver_action ();
  if (hello_is_present()) {
    printf ("The Hello driver is present\n");
    
    // print
    hello_print ();
    
    // up
    hello_up ();
    hello_print ();

    // down
    hello_down ();
    hello_print ();

    // set
    hello_set (0);
    hello_print ();

    // error
    hello_down ();
    hello_print ();
    
    // get
    hello_set (120);
    hello_print ();
    unsigned int n;
    if (hello_get(&n)) {
      printf ("n is %d\n", n);
    }
    else
    {
      printf ("Failed to read n from hello driver\n");
    }
  }
  else
  {
    printf ("The Hello driver is not present\n");
  }
  return 0;
}
