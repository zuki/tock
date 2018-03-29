#include <stdio.h>

#include <timer.h>
#include <ipc.h>
#include <tock.h>

#include "edimax-sp2101w.h"

int blegateway_svc_num = 0;
char buf[512] __attribute__((aligned(512)));

// void sensing_timer_callback (int a, int b, int c, void* ud);
// void start_sensing (void);

// char _post[512];
// int _post_len = 0;

// char AIO_KEY[]   =  "3e83a80b0ed94a338a3b3d26998b0dbe";
// char USERNAME[]  = "bradjc";
// char FEED_NAME[] = "hail-test";


// void sensing_timer_callback (__attribute__ ((unused)) int a,
//                              __attribute__ ((unused)) int b,
//                              __attribute__ ((unused)) int c,
//                              __attribute__ ((unused)) void* ud) {
//   char value[200];
//   int light;

//   light = isl29035_read_light_intensity();
//   snprintf(value, 200, "%i", light);

//   _post_len = create_post_to_feed(_post, 512, USERNAME, FEED_NAME, AIO_KEY, value);

//   blehttp_send_http_message(_post, _post_len);
// }

typedef enum {
  HTTP_MESSAGE = 1, // This buffer contains an HTTP message to send.
} msg_type_e;

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
  printf("svc: %i\n", blegateway_svc_num);

  printf("[Edimax] Control\n");

  ipc_register_client_cb(blegateway_svc_num, callback, NULL);
  ipc_share(blegateway_svc_num, buf, 512);

  msg_len = edimax_control_message(buf+3, 509, true, "192.168.86.152", "admin", "1234");
  printf("%s\n", buf+3);
  printf("%i\n", msg_len);

  buf[0] = HTTP_MESSAGE;
  buf[1] = msg_len & 0xFF;
  buf[2] = (msg_len >> 8) & 0xFF;


  printf("notify the ble gateway service\n");
  ipc_notify_svc(blegateway_svc_num);
}
