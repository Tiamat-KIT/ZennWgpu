const canvas = document.getElementById("canvas")
if(!(canvas instanceof HTMLCanvasElement)){
    throw new Error("canvasが見つかりませんでした")
}
if(!("gpu" in navigator)){
    throw new Error("WebGPUに未対応")
}
const adapter = await navigator.gpu.requestAdapter()
if(!adapter){
    throw new Error("WebGPUのアダプタを取得できませんでした")
}
const device = await adapter.requestDevice()
if(!device){
    throw new Error("WebGPUのデバイスを取得できませんでした")
}
const context = canvas.getContext("webgpu")
if(!context){
    throw new Error("WebGPUのコンテキストを取得できませんでした")
}

const format  = navigator.gpu.getPreferredCanvasFormat()
context.configure({
    device,
    format,
    // alphaMode: "opaque"
})

const module = device.createShaderModule({
    label: "triangle shader",
    code: `
    @vertex fn vs(
        @builtin(vertex_index) vertexIndex : u32
    ) -> @builtin(position) vec4f {
        let pos = array(
        vec2f( 0.0,  0.5), 
        vec2f(-0.5, -0.5),  
        vec2f( 0.5, -0.5) 
        );
        
        return vec4f(pos[vertexIndex], 0.0, 1.0);
    }

    @fragment fn fs() -> @location(0) vec4f {
        return vec4f(1.0, 1.0, 1.0, 1.0);
    }`
}) 

const pipeline = device.createRenderPipeline({
    layout: "auto",
    label: "triangle pipeline",
    vertex: {
        module,
        entryPoint: "vs"
    },
    fragment: {
        module,
        entryPoint: "fs",
        targets: [{format}]
    },
    primitive: {
        topology: "triangle-list"
    }
})

const renderPassDescriptor = {
    label: "canvas renderPass",
    colorAttachments: [{
        view: context.getCurrentTexture().createView(),
        clearValue: [0.3,0.3,0.3,1],
        loadOp: "clear",
        storeOp: "store"
    }]
}

function render() {
    // canvasのコンテキストから、カレントテクスチャを得る。
    // それをレンダーパスに設定して、描画対象として指定する。
    renderPassDescriptor.colorAttachments[0].view =
        context!.getCurrentTexture().createView();

    // コマンドエンコーダを生成する。コマンドのエンコードができる状態にする。
    const encoder = device.createCommandEncoder({ label: 'our encoder' });

    // レンダーパスのエンコーダを生成する。そこへコマンドを並べて、描画手順をエンコードする。
    const pass = encoder.beginRenderPass(renderPassDescriptor as GPURenderPassDescriptor);
    pass.setPipeline(pipeline);
    pass.draw(3);  // 頂点シェーダを３回呼び出す
    pass.end();

    device.queue.submit([ encoder.finish()]);
}
render();

export {}