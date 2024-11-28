#include <stdio.h>
#include <catgirl-engine.h>

int main(int argc, const char * const* argv) {
  printf("Starting Catgirl Engine From C Program\n");

  // Testing Calling Functions
  // print_version();
  // print_dependencies();

  return ce_start(argc, argv);
}
