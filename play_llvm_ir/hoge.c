#include <stdio.h>

int square(int n) {
  return n * n;
}

void map(const int *source, int *result, size_t n, int (*func)(int)) {
  int i;

  for (i = 0; i < n; i++) {
    result[i] = func(source[i]);
  }
}

int main(int argc, char **argv) {
  int numbers[] = {1, 2, 3, 4, 5};
  int result[5] = {0};

  map(numbers, result, 5, (int (*)(int))square);

  int i = 0;
  for (i = 0; i < 5; i++) {
    printf("%d\n", result[i]);
  }
}