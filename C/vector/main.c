#include "vector.h"

int main() 
{
  vector my_vector = new_vector(10);
  int new_val = 10;

  insert(&my_vector, 0, 1, &new_val);
}
