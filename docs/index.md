---
---

## Wasm Test

<link rel="manifest" href="./manifest.json" />
<link rel="icon" type="image/png" href="./resources/assets/vanilla/texture/logo/logo-1024x1024-color.png" />
<link rel="icon" type="image/svg+xml" href="./resources/assets/vanilla/texture/logo/logo.svg" />
<link rel="license" href="./pkg/LICENSE" />


<canvas id="catgirl-engine-canvas"></canvas>
<!-- <script type="text/javascript">console.clear();</script> -->
<script type="text/javascript">
    if ('serviceWorker' in navigator) {
        if (window.performance && performance.mark) {
            performance.mark("Service-Worker: Register");
        }

        navigator.serviceWorker.register("./service-worker.js");
    }
</script>
<script type="module">
    if (window.performance && performance.mark) {
        performance.mark("Wasm: Download");
    }

    import init, * as engine from "./pkg/main.js";

    window.addEventListener('load', async function () {
        if (window.performance && performance.mark) {
            performance.mark("Wasm: Initialize");
        }

        try {
            await init();
        } catch(error) {
            // console.error("Wasm Error: " + error);
            return;
        }

        if (window.performance && performance.mark) {
            performance.mark("Wasm: Loaded");
        }

        console.log("WASM Loaded");
        console.debug("%cYou can gain access to the engine's functions by calling %cgetEngine()", "color: orange; font-weight: bold; font-size: 200%", "color: purple; font-weight: bold; font-size: 200%");
        // engine.print_version();
        // engine.print_dependencies();
    });

    /**
     * Allows retrieving engine
     * @returns {engine} Exported engine functions
     */
    export function getEngine() {
        return engine;
    }

    // Allows retrieving engine from console
    //   as window.getEngine()
    globalThis.getEngine = getEngine
</script>
