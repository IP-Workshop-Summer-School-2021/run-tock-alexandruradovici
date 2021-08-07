/* vim: set sw=2 expandtab tw=80: */

#include <stdio.h>
#include <ipc.h>
#include "dots_text_display.h"
#include "text_screen.h"

#define LEN 50


static int min(int a, int b) {
  return (a<b)?a:b;
}

static void ipc_callback(int pid, int len, int buf, __attribute__ ((unused)) void* ud) {
  char* received_buf = (char*) buf;
  printf ("Got buffer %p from %d of length %d\n", (void*)buf, pid, len);
  char *buffer = (char*)text_screen_buffer ();
  len = strnlen (received_buf, len);
  int print_len = min(LEN, len);
  strncpy (buffer, received_buf, print_len);
  text_screen_write (print_len);
}

int main(void) {
  // display_text ("987654321");
  text_screen_init (LEN);
  char *buffer = (char*)text_screen_buffer ();
  strcpy (buffer, "");
  text_screen_write (0);

  ipc_register_service_callback(ipc_callback, NULL);
  
  return 0;
}