#include <stdio.h>

#include <timer.h>
#include <ipc.h>
#include <tock.h>

#include "edimax-sp2101w.h"

int blegateway_svc_num = 0;
char buf[512] __attribute__((aligned(512)));

typedef enum {
  HTTP_MESSAGE = 1, // This buffer contains an HTTP message to send.
} msg_type_e;

typedef enum {
  MODE_HTTPS = 1,
  MODE_HTTP = 2,
} msg_mode_e;

static void callback(__attribute__ ((unused)) int pid,
                           __attribute__ ((unused)) int len,
                           __attribute__ ((unused)) int arg2, void* ud) {
  printf("yay\n");
}


/*******************************************************************************
 * MAIN
 ******************************************************************************/

int main (void) {
  uint16_t msg_len;

  blegateway_svc_num = ipc_discover("org.tockos.services.ble-http-gateway");
  if (blegateway_svc_num < 0) {
    printf("No BLE HTTP Gateway service\n");
    return -1;
  }

  printf("[Edimax] Control\n");

  ipc_register_client_cb(blegateway_svc_num, callback, NULL);
  ipc_share(blegateway_svc_num, buf, 512);

  msg_len = edimax_control_message(buf+4, 508, true, "192.168.86.152", "admin", "1234");

  buf[0] = HTTP_MESSAGE;
  buf[1] = msg_len & 0xFF;
  buf[2] = (msg_len >> 8) & 0xFF;
  buf[3] = MODE_HTTP;


  printf("Submit HTTP request\n");
  ipc_notify_svc(blegateway_svc_num);
}
