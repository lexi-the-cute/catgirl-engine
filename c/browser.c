#include <string.h>
#include <emscripten/emscripten.h>
#include <emscripten/html5.h>

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

void create_webgl_context() {
    EmscriptenWebGLContextAttributes attr;
    attr.explicitSwapControl = 0;
    attr.renderViaOffscreenBackBuffer = 0;
    attr.proxyContextToMainThread = EMSCRIPTEN_WEBGL_CONTEXT_PROXY_DISALLOW;

    emscripten_webgl_init_context_attributes(&attr);
    EMSCRIPTEN_WEBGL_CONTEXT_HANDLE ctx = emscripten_webgl_create_context("#canvas", &attr);
    emscripten_webgl_make_context_current(ctx);
}