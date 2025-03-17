#include <stdio.h>

unsigned getbits(unsigned x, int p, int n)
{
  return (x >> (p+1-n)) & ~(~0 << n);
}

unsigned setbits(unsigned x, int p, int n, unsigned y)
{
  unsigned mask = ~(~0 << n) << (p + 1 - n); //used to isolate the bits we want. Eg: i want the two bits in the middle of 110011
  x = x & ~mask;
  return (getbits(y, p, n) << (p+1-n)) | x;
}

unsigned invert(unsigned x, int p, int n) 
{
  unsigned mask_y = ~(~0 << n) << (p + 1 - n);
  unsigned y = ~x;
  return (x & ~mask_y) | (y & mask_y);
}

/*
  * Rotate the integer x to the right by n positions
  * Eg: 3 = 000101 -> 100010 = 66
  *
*/
int rightrot(int x, int n) 
{
  while (n > 0) {
    if (x & 1) 
      x = (x >> 1) | ~(~0 >> 1);
    else
     x >>= 1;

    --n;
  }
  return x;
}

int main() 
{
  int x, p, n, y;
  x = 3;
  p = 2;
  n = 2;
  y = 4;
  printf("%d", getbits(x, p, n));
  printf("%d", setbits(x, p, n, y));
  printf("%d", invert(x, p, n));
  printf("%d", rightrot(x, n));
}


