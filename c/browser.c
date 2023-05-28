#include <string.h>
#include <emscripten.h>

void trace(char* message) {
    EM_ASM({
        if (ENVIRONMENT_IS_WEB || ENVIRONMENT_IS_NODE) {
            console.trace(UTF8ToString($0, $1));
        } else if (ENVIRONMENT_IS_WORKER) {
            let data = {};
            data['cmd'] = "callHandler";
            data['handler'] = "trace";
            data['args'] = UTF8ToString($0, $1);

            postMessage(data);
        }
    }, message, strlen(message));
}

void debug(char* message) {
    EM_ASM({
        if (ENVIRONMENT_IS_WEB || ENVIRONMENT_IS_NODE) {
            console.debug(UTF8ToString($0, $1));
        } else if (ENVIRONMENT_IS_WORKER) {
            let data = {};
            data['cmd'] = "callHandler";
            data['handler'] = "debug";
            data['args'] = UTF8ToString($0, $1);

            this.postMessage(data);
        }
    }, message, strlen(message));
}

void info(char* message) {
    EM_ASM({
        if (ENVIRONMENT_IS_WEB || ENVIRONMENT_IS_NODE) {
            console.info(UTF8ToString($0, $1));
        } else if (ENVIRONMENT_IS_WORKER) {
            let data = {};
            data['cmd'] = "callHandler";
            data['handler'] = "info";
            data['args'] = UTF8ToString($0, $1);

            postMessage(data);
        }
    }, message, strlen(message));
}

void warn(char* message) {
    EM_ASM({
        if (ENVIRONMENT_IS_WEB || ENVIRONMENT_IS_NODE) {
            console.warn(UTF8ToString($0, $1));
        } else if (ENVIRONMENT_IS_WORKER) {
            let data = {};
            data['cmd'] = "callHandler";
            data['handler'] = "warn";
            data['args'] = UTF8ToString($0, $1);

            postMessage(data);
        }
    }, message, strlen(message));
}

void error(char* message) {
    EM_ASM({
        if (ENVIRONMENT_IS_WEB || ENVIRONMENT_IS_NODE) {
            console.error(UTF8ToString($0, $1));
        } else if (ENVIRONMENT_IS_WORKER) {
            let data = {};
            data['cmd'] = "callHandler";
            data['handler'] = "error";
            data['args'] = UTF8ToString($0, $1);

            postMessage(data);
        }
    }, message, strlen(message));
}