---
---

Catgirl Engine Test

<style>
    canvas {
        background-color: black;
    }
</style>
<canvas id="catgirl-engine-canvas"></canvas>
<script type="module">
    import init from "./pkg/main.js";
    init().then(() => {
        console.log("WASM Loaded");
    });
</script>
