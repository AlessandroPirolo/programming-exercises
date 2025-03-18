#include "vector.h"
#include <malloc.h>
#include <stdbool.h>
#include <string.h>

typedef struct Vector {
  int capacity;
  int size;
  int **v;
} vector;

/* create */
vector new_vector(int capacity) {
  vector result;
  
  result.capacity = capacity;
  result.size = 0;
  result.v = (int **) malloc(sizeof(int*) * capacity);

  return result;
}

/* capacity */
int size(vector v) {
  return v.size;
}

int capacity(vector v) {
  return v.capacity;
}

bool empty(vector v) {
  return v.size==0;
}

void resize(vector *v, int new_size, int val) {
  int old_size = v->size;
  v->size = new_size;

  if (new_size > old_size) {
    if (new_size > v->capacity) {
      int** tmp = (int **) realloc(v->v, sizeof(int*) * new_size);
      if (tmp) {
        v->v = tmp;
        v->capacity = new_size;
      }

    }
  
    for (int i = old_size; i < new_size; i++) {
      **(v->v+i) = val; 
    }
  }

  if (new_size < old_size) {
    for (int i = new_size; i < old_size; i++) v->v[i] = NULL;
  }
}

void reserve(vector *v, int new_capacity) {
  int** tmp = (int **) realloc(v->v, sizeof(int*) * new_capacity);
  if (tmp) {
    v->v = tmp;
    v->capacity = new_capacity;
  }
}

void shrink_to_fit(vector *v) {
  reserve(v, v->size);
}

/* Element access */
int at(vector *v, int pos) {
  if (pos < v->size) {
    return *(v->v[pos]);
  } else {
    return NULL;
  }
}

int front(vector *v) {
  return at(v, 0);
}

int back(vector *v) {
  return at(v, v->size-1);
}

int* data(vector *v) {
  return v->v[0];
}

/* Modifiers */
void assign(vector* v, int size, int val) {
  reserve(v, size * 2);
  for (int i = 0; i < size; i++) {
    v->v[i] = &val;
  }
  for (int i = size; i < v->size; i++) {
    v->v[i] = NULL;
  }
  v->size = size;
}

void push_back(vector* v, int val) {
  if (v->size+1 > v->capacity) {
    reserve(v, v->capacity * 2);
  }
  v->size++;
  *(v->v[v->size-1]) = val;
}

void pop_back(vector* v) {
  v->v[v->size-1] = NULL;
  v->size--;
}

void insert(vector* v, int pos, int size, int val) {
  if (size > 0) {
    if (v->size + size > v->capacity) {
      reserve(v, v->capacity + size);
    }

    for (int i = v->size-1; i > pos; i--) {
      v->v[i+size] = v->v[i];
      v->v[i] = NULL;
    }

    for (int i = pos; i < size; i++) {
      int *ptr = &val;
      v->v[i] = ptr;
    }
    v->size += size;
  }
}

void erase(vector *v, int pos) {
  v->v[pos] = NULL;

  for (int i = pos; i < v->size; i++) {
    v->v[i] = v->v[i+1];
    v->v[i+1] = NULL;
  }

  v->size--;
}

void swap(vector* v1, vector* v2) {
  int v1_size = v1->size;
  int v2_size = v2->size;
  int max_size = v1_size > v2_size ? v1_size : v2_size;

  v1->capacity > v2->capacity ? reserve(v2, v1->capacity) : reserve(v1, v2->capacity);

  for (int i = 0; i < max_size; i++) {
    int* tmp = v1->v[i];
    v1->v[i] = v2->v[i];
    v2->v[i] = tmp;
  }

  v1->size = v2_size;
  v2->size = v1_size;
}

void clear(vector *v) {
  for (int i = 0; i < v->size; i++) {
    v->v[i] = NULL;
  }
  v->size = 0;
}
