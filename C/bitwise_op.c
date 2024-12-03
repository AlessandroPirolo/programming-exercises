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
  unsigned mask_x = ~mask_y;
  unsigned y = ~x;
  x = x & mask_x;
  y = y & mask_y;
  return x | y;
}

unsigned rightrot(int x, int n) 
{
  return x >> n;
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
}


