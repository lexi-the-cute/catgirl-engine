#include <emscripten/emscripten.h>
#include <emscripten/html5.h>

// This runs from the calling thread.
// https://emscripten.org/docs/api_reference/html5.h.html
EMSCRIPTEN_RESULT create_webgl_context() {
    EmscriptenWebGLContextAttributes attr;
    // attr.explicitSwapControl = 1;
    // attr.renderViaOffscreenBackBuffer = 1;
    attr.proxyContextToMainThread = EMSCRIPTEN_WEBGL_CONTEXT_PROXY_FALLBACK;

    emscripten_webgl_init_context_attributes(&attr);
    EMSCRIPTEN_WEBGL_CONTEXT_HANDLE ctx = emscripten_webgl_create_context("#canvas", &attr);

    return emscripten_webgl_make_context_current(ctx);
}