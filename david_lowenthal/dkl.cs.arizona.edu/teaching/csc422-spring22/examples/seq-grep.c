// CSc 422
// Sequential Grep Example 

#include <stdio.h>
#include <sys/types.h>
#include <pthread.h>
#include <stdlib.h>
#include <unistd.h>
#include <string.h>

char *line;
char *pattern;

void search() {
  int i, j, numMatched;
  int lineLength = strlen(line);
  int patternLength = strlen(pattern);

  for (i = 0; i < lineLength; i++) {
    numMatched = 0;
    for (j = i; j < i+patternLength; j++) {
      if (line[j] != pattern[j-i]) 
        break;
      else {
        numMatched++;
        if (numMatched == patternLength)
          break;
      }
    }
    if (numMatched == patternLength)
      printf("%s", line);
  }

  return;
}

int main(int argc, char *argv[]) {
  FILE *f;
  int count;

  line = (char *) malloc (sizeof(char) * 80);
  pattern = (char *) malloc (sizeof(char) * 80);

  strcpy(pattern, argv[1]);
  f = fopen(argv[2], "r");

  count = 0;
  while (fgets(line, 80, f) != NULL) {
    search();
    count++;
  }

  printf("Number of lines: %d\n", count);
    
}



