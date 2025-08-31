// CSc 422
// example: finding the max

#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/time.h>   //setitimer

int *a;
int which, numThreads, arraySize;
int max = 0, pos = 0;
pthread_mutex_t L;


// non-thread-safe version (shown only to provide lower bound on execution time)
void max0(int myId) {
  int startpos = myId * arraySize/numThreads;
  int endpos = (myId+1) * (arraySize/numThreads) - 1;
  int i;

  for (i = startpos; i <= endpos; i++) {
    if (a[i] > max) {
      pos = i;
      max = a[i];
    }
  }
}

// correct but poorly performing parallel version
void max1(int myId) {
  int startpos = myId * arraySize/numThreads;
  int endpos = (myId+1) * (arraySize/numThreads) - 1;
  int i;

  for (i = startpos; i <= endpos; i++) {
    pthread_mutex_lock(&L);  // implementing <...> from the concurrency slide deck
    if (a[i] > max) {
      pos = i;
      max = a[i];
    }
    pthread_mutex_unlock(&L);
  }
}

// correct, complicated, but efficient parallel version
void max2(int myId) {
  int startpos = myId * arraySize/numThreads;
  int endpos = (myId+1) * (arraySize/numThreads) - 1;
  int i;

  for (i = startpos; i <= endpos; i++) {
    if (a[i] > max) {
      pthread_mutex_lock(&L);  // implementing <...> from the concurrency slide deck
      if (a[i] > max) {   // note the rechecking of condition
	pos = i;
	max = a[i];
      }
      pthread_mutex_unlock(&L);
    }
  }

}

void *worker(void *arg)
{
  int id = *((int *) arg);
  if (which == 0)
    max0(id);
  else if (which == 1)
    max1(id);
  else
    max2(id);
  return NULL;
}


int main(int argc, char **argv) {
  int i;
  int *params;
  pthread_t *threads;
  struct timeval start, stop;
  double elapsed;

  arraySize = atoi(argv[1]);
  numThreads = atoi(argv[2]);
  which = atoi(argv[3]);

  a = (int *) malloc (sizeof(int) * arraySize);

  for (i = 0; i < arraySize; i++)
    a[i] = rand() % 1000000;

  pthread_mutex_init(&L, NULL);

  gettimeofday(&start, NULL);
  // Allocate thread handles
  threads = (pthread_t *) malloc(numThreads * sizeof(pthread_t));
  params = (int *) malloc(numThreads * sizeof(int));

  // Create threads
  for (i = 0; i < numThreads; i++) {
    params[i] = i;
    pthread_create(&threads[i], NULL, worker, (void *)(&params[i]));
  }

  for (i = 0; i < numThreads; i++) {
    pthread_join(threads[i], NULL);
  }
  
  printf("max is %d; found at position %d\n", max, pos);

  gettimeofday(&stop, NULL);
  elapsed = ((stop.tv_sec - start.tv_sec) * 1000000+(stop.tv_usec-start.tv_usec))/1000000.0;
  printf("time taken is %f seconds\n", elapsed);


}
