#include <stdio.h>

/* 
 * Vertical histograms of the 
 * lenghts of words in its 
 * input
 */

int max(int arr[], int len) {
  int max, i;
  max = 0;
  for(i = 0; i < len; ++i) {
    if (arr[i] > max) 
      max = arr[i];
  }
  return max;
}

int main() 
{
  int c, nc, i, j, maxn;
  int word_len[11]; /* we assume the longest word has 10 characters */

  for (i = 0; i < 11; ++i) 
    word_len[i] = 0;

  nc = 0;
  while ((c = getchar()) != EOF) {
    if (c == ' ' || c == '\n' || c == '\t') {
      ++word_len[nc];
      nc = 0;
    } 
    else 
      ++nc;
  }

  maxn = max(word_len, 11);


  for (; maxn > 0; --maxn) {
    printf(" ");
    for (i = 1; i < 11; ++i) { 
      if (word_len[i] >= maxn) {
        printf("*");
      }
      else {
        printf(" ");
      }
      printf(" ");
    }
    printf("\n");
  }

  printf(" ");
  for (i = 1; i < 11; ++i) 
    printf("%d ", i);
} 

