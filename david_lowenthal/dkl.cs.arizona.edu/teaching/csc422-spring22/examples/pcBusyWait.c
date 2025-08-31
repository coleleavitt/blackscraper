/* 

CSc 422

a simple producer/consumer using busy-waiting threads

   gcc pcBusyWait.c -lpthread
   a.out numIters

*/
 
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>

void *Producer(void *);
void *Consumer(void *);

int produced = 0, consumed = 0;
int data;
int numIters;

/* main() -- read command line and create threads, then
             print result when the threads have quit */

int main(int argc, char *argv[]) {
  /* thread ids */
  pthread_t pid, cid;  

  numIters = atoi(argv[1]);
  printf("main started\n");
  pthread_create(&pid, NULL, Producer, NULL);
  pthread_create(&cid, NULL, Consumer, NULL);
  pthread_join(pid, NULL);
  pthread_join(cid, NULL);
  printf("main done\n");
}


void *Producer(void *arg) {
  printf("Producer created\n");
  while (produced < numIters) {
    while (produced > consumed)   /* wait for buffer to be empty */
      ;
    data = produced;
    produced++;
  }
  return NULL;
}


void *Consumer(void *arg) {
  int total = 0;
  printf("Consumer created\n");
  while (consumed < numIters) {
    while (produced == consumed)  /* wait for buffer to be full */
      ;
    total = total+data;
    consumed++;
  }
  printf("for %d iterations, the total is %d\n", numIters, total);
  return NULL;
}
