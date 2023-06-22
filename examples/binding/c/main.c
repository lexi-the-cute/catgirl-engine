#include <stdio.h>
#include <catgirl-engine.h>

int main(int argc, const char * const* argv) {
  printf("Starting Catgirl Engine From C Program\n");

  return ce_start(argc, argv);
}