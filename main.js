async function init() {
    const {instance} = await WebAssembly.instantiateStreaming(
        fetch("./target/wasm32-unknown-unknown/release/stars_rs.wasm"),
        {
            "env": {
                "js_random": Math.random,
            }
        }
    );

    const width = 600;
    const height = 600;

    const canvas = document.getElementById("main");
    canvas.width = width;
    canvas.height = height;

    const buffer_address = instance.exports.BUFFER.value;
    const image = new ImageData(
        new Uint8ClampedArray(
            instance.exports.memory.buffer,
            buffer_address,
            4 * width * height,
        ),
        width,
    );

    const ctx = canvas.getContext("2d");

    const render = () => {
        instance.exports.go();
        ctx.putImageData(image, 0, 0);
        requestAnimationFrame(render);
    };

    render();
}

init();
