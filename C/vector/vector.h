#include <stdbool.h>

typedef struct Vector vector;

/* Create */
vector new_vector(int capacity); 

/* Capacity */
int size(vector v);
int capacity(vector v);
void resize(vector *v, int new_size, int val);
bool empty(vector v);
void reserve(vector *v, int new_capacity);
void shrink_to_fit(vector *v);

/* Element access */
int at(vector *v, int pos);
int front(vector *v);
int back(vector *v);
int* data(vector *v);

/* Modifiers */
void assign(vector* v, int size, int val);
void push_back(vector* v, int val);
void pop_back(vector* v);
void insert(vector* v, int pos, int size, int val);
void erase(vector *v, int pos);
void swap(vector* v1, vector* v2);
void clear(vector *v);
