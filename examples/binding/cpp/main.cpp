#include <iostream>
#include <catgirl-engine.hpp>
using namespace std;

int main(int argc, const char * const* argv) {
  cout << "Starting Catgirl Engine From C++ Program\n";

  return ffi::start_engine(argc, argv);
}
