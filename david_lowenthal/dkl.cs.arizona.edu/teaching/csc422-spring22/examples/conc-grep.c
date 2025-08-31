// CSc 422
// Grep example with pthreads
// "Co nested within while" version

#include <stdio.h>
#include <sys/types.h>
#include <pthread.h>
#include <stdlib.h>
#include <unistd.h>
#include <string.h>

char *line[2];
char *pattern;
int next = 0;
char *retVal;
FILE *f;

void *readNext(void *dummy) {
  retVal = fgets(line[next], 80, f);
  return NULL;
}

void *search(void *dummy) {
  char *l = line[1-next];
  int i, j, numMatched;
  int lineLength = strlen(l);
  int patternLength = strlen(pattern);

  for (i = 0; i < lineLength; i++) {
    numMatched = 0;
    for (j = i; j < i+patternLength; j++) {
      if (l[j] != pattern[j-i]) 
        break;
      else {
        numMatched++;
        if (numMatched == patternLength)
          break;
      }
    }
    if (numMatched == patternLength)
      printf("%s", l);
  }
  return NULL;
}

int main(int argc, char *argv[]) {
  pthread_t t1, t2;
  int *dummy;
  char *p;

  line[0] = (char *) malloc (sizeof(char) * 80);
  line[1] = (char *) malloc (sizeof(char) * 80);
  pattern = (char *) malloc (sizeof(char) * 80);

  strcpy(pattern, argv[1]);
  f = fopen(argv[2], "r");

  retVal = fgets(line[next], 80, f);
  next++;
  while (retVal != NULL) {
    if (pthread_create(&t1, NULL, search, (void *) dummy) < 0)
      perror("search");  
    if (pthread_create(&t2, NULL, readNext, (void *) dummy) < 0)
      perror("readNext");  

    pthread_join(t1, NULL);
    pthread_join(t2, NULL);
  
    next = 1 - next;

  }
    
}



