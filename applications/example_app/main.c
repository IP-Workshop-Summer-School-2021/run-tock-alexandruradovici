/* vim: set sw=2 expandtab tw=80: */

#include <stdio.h>
#include "dots_text_display.h"
#include "text_screen.h"
#include "timer.h"

int main(void) {
  // display_text ("987654321");
  text_screen_init (50);
  char *buffer = (char*)text_screen_buffer ();
  strcpy (buffer, "123456789");
  text_screen_write (9);
  if (display_is_present ()) {
    printf ("Dots Text Display is available\n");
    for (int i=500; i < 5000; i = i + 500) {
      display_set_speed (i);
      unsigned int speed;
      display_get_speed (&speed);
      printf ("Display speed: %d\n", speed);
      delay_ms (5000);
    }
  }
  return 0;
}
