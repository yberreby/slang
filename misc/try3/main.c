#include <stdio.h>
static int COUNTER = 10;

int main() {

  COUNTER++;

  void* ptr = &COUNTER;


  printf("%p", ptr);

  return 0;
}
