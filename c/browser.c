#include <string.h>
#include <emscripten.h>

void trace(char* message) {
    EM_ASM({
        console.trace(UTF8ToString($0, $1));
    }, message, strlen(message));
}

void debug(char* message) {
    EM_ASM({
        console.debug(UTF8ToString($0, $1));
    }, message, strlen(message));
}

void info(char* message) {
    EM_ASM({
        console.info(UTF8ToString($0, $1));
    }, message, strlen(message));
}

void warn(char* message) {
    EM_ASM({
        console.warn(UTF8ToString($0, $1));
    }, message, strlen(message));
}

void error(char* message) {
    EM_ASM({
        console.error(UTF8ToString($0, $1));
    }, message, strlen(message));
}