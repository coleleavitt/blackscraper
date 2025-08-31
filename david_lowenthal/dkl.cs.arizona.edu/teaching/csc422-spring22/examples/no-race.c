// CSc 422
// Using an atomic instruction in C to avoid a race

#include <stdio.h>
#include <sys/types.h>
#include <pthread.h>
#include <stdlib.h>

int x = 0;
int iters;

void noRace(int myId) {

  int i;

  for (i = 0; i < iters; i++)
    __sync_fetch_and_add(&x, 0x00000001);
}

void *worker(void *arg)
{
  int id = *((int *) arg);
  noRace(id);
  return NULL;
}


int main(int argc, char *argv[]) {
  int i, j;
  int *p;
  pthread_t *threads;
  int numThreads;

  iters = atoi(argv[1]);
  numThreads = atoi(argv[2]);

  // Allocate thread handles
  threads = (pthread_t *) malloc(numThreads * sizeof(pthread_t));

  // Create threads 
  for (i = 0; i < numThreads; i++) {
    p = (int *) malloc(sizeof(int));  // yes, memory leak, don't worry for now
    *p = i;
    pthread_create(&threads[i], NULL, worker, (void *)(p));
  }

  for (i = 0; i < numThreads; i++) {
    pthread_join(threads[i], NULL);
  }

  printf("x is %d; it should be %d\n", x, iters*numThreads);
}

