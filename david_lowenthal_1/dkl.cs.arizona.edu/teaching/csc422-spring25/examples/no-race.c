// CSc 422
// Race condition example

#include <stdio.h>
#include <sys/types.h>
#include <pthread.h>
#include <stdlib.h>

int x = 0;
int iters;

void *race(void *dummyParam) {

  int i;

  for (i = 0; i < iters; i++)
    __sync_fetch_and_add(&x, 0x00000001);

  return NULL;
}

int main(int argc, char *argv[]) {
  int i, dummy;
  pthread_t *threads;
  int numThreads;

  iters = atoi(argv[1]);
  numThreads = atoi(argv[2]);

  // Create threads 
  for (i = 0; i < numThreads; i++) {
    pthread_create(&threads[i], NULL, race, (void *)(&dummy));
  }

  for (i = 0; i < numThreads; i++) {
    pthread_join(threads[i], NULL);
  }

  printf("x is %d; it should be %d\n", x, iters*numThreads);
}

