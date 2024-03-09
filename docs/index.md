---
---

## Wasm Test

<link rel="icon" href="./assets/vanilla/texture/logo/logo.png">
<canvas id="catgirl-engine-canvas"></canvas>
<script type="module">
    import init, * as engine from "./pkg/main.js";
    init().then(() => {
        console.log("WASM Loaded");
        // engine.print_version();
        // engine.print_dependencies();
    });
</script>
