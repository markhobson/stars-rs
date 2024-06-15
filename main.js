async function init() {
    const {instance} = await WebAssembly.instantiateStreaming(
        fetch("./target/wasm32-unknown-unknown/release/rust_demo.wasm")
    );

    const answer = instance.exports.the_answer();
    console.log(answer);
}

init();
