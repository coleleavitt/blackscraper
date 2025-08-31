// CSc 422
// First threads example

#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>

int glob1 = 0, glob2 = 0;

void *foo(void *dummyParam) {
  glob1 = 1;
  return NULL;
}

void *bar(void *dummyParam) {
  glob2 = 1;
  return NULL;
}

int main(int argc, char **argv) {

  int *dummy1, *dummy2;
  pthread_t t1, t2;

  dummy1 = (int *) malloc (sizeof(int));
  dummy2 = (int *) malloc (sizeof(int));

  /* dummy1 and dummy2 never used and so not initialized */
  if (pthread_create(&t1, NULL, foo, (void *) dummy1) < 0)
    perror("foo");  

  if (pthread_create(&t2, NULL, bar, (void *) dummy2) < 0)
    perror("bar");  

  pthread_join(t1, NULL);
  pthread_join(t2, NULL);
  
  printf("glob1 and glob2 are %d and %d\n", glob1, glob2);

  return 0;
}
