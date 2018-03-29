#include <stdio.h>

#include <nrf_error.h>

#include <timer.h>
#include <tock.h>

#include "ble_http.h"

typedef enum {
  HTTP_MESSAGE = 1, // This buffer contains an HTTP message to send.
} msg_type_e;

typedef enum {
  MODE_HTTPS = 1,
  MODE_HTTP = 2,
} msg_mode_e;

static void ipc_callback(int pid, int len, int bufptr, __attribute__ ((unused)) void* ud) {
  uint8_t* buf = (uint8_t*) bufptr;

  if (len < 3) {
    printf("Error! IPC message too short.\n");
    ipc_notify_client(pid);
    return;
  }

  switch (buf[0]) {
    case HTTP_MESSAGE: {
      uint16_t msg_len = buf[1] | (((uint16_t) buf[2]) << 8);
      printf("recvd http message len: %i\n", msg_len);

      msg_mode_e mode = buf[3];
      bool https = false;
      if (mode == MODE_HTTPS) {
        https = true;
      }

      // Setup the BLE HTTP library.
      uint32_t err_code = blehttp_init();
      if (err_code != NRF_SUCCESS) {
        printf("error setting up BLE HTTP: %lx\n", err_code);
      }

      blehttp_send_http_message(buf+4, msg_len, https);
      break;
    }
  }
}


/*******************************************************************************
 * MAIN
 ******************************************************************************/

int main (void) {

  printf("[BLE] HTTP Gateway\n");

  // Listen for IPC requests to make a web request.
  ipc_register_svc(ipc_callback, NULL);
}
