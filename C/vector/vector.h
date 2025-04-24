#include <stdbool.h>

typedef struct Vector {
  int capacity;
  int size;
  void **v;
} vector;

/* Create */
vector new_vector(int capacity); 

/* Capacity */
int size(vector v);
int capacity(vector v);
void resize(vector *v, int new_size, void* val);
bool empty(vector v);
void reserve(vector *v, int new_capacity);
void shrink_to_fit(vector *v);

/* Element access */
void* at(vector *v, int pos);
void* front(vector *v);
void* back(vector *v);
void** data(vector *v);

/* Modifiers */
void assign(vector* v, int size, void* val);
void push_back(vector* v, void* val);
void pop_back(vector* v);
void insert(vector* v, int pos, int size, void* val);
void erase(vector *v, int pos);
void swap(vector* v1, vector* v2);
void clear(vector *v);

/* Others */
void print_vector(vector* v);
