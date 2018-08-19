#include <stdio.h>

auto make_counter = [](int init_val) {
  return [=](int val) {
    return init_val + val;
  };
};

// auto a = [](){ return 13; };

int main() {
  auto c1 = make_counter(111);
  
  int ret = c1(222);
  printf("%d\n", ret);
  return 0;
}