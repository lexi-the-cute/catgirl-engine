---
---

## Wasm Test

<link rel="icon" href="./assets/vanilla/texture/logo/logo.png">
<canvas id="catgirl-engine-canvas"></canvas>
<script type="module">
    import init, * as engine from "./pkg/main.js";

    window.addEventListener('load', async function () {
        await init();

        console.log("WASM Loaded");
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
