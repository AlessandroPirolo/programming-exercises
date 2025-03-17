#include <stdio.h>
#include <string.h>

void reverse(char s[])
{
  int c, i, j;

  for (i = 0, j = strlen(s) - 1; i < j; i++, j--) {
    c = s[i];
    s[i] = s[j];
    s[j] = c;
  }
}

void itoa(int n, char s[]) 
{
  int i, sign;

  if((sign = n) < 0)
    n = -n;
  i = 0;
  do {
    s[i++] = n % 10 + '0';
  } while ((n /= 10) > 0);
  if (sign < 0)
    s[i++] = '-';
  s[i] = '\0';
  reverse(s);
}

int main() 
{
  char s[1000];
  // can't handle largest negative number: inside itoa it is converted to positive, so 2147483648 which is greater than the largest positive number, i.e. 2147483647  
  itoa(-2147483648, s);
  printf("%s", s);

}
