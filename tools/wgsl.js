const testwgsl = 'test.wgsl';
const path = `../src/scripts/libs/${testwgsl}`;



async function main() {
  const adapter = await navigator.gpu?.requestAdapter();
  const device = await adapter?.requestDevice();
  if (!device) {
    fail('need a browser that supports WebGPU');
    return;
  }
  // Get a WebGPU context from the canvas and configure it
  const canvas = document.querySelector('canvas');
  const context = canvas.getContext('webgpu');
  const presentationFormat = navigator.gpu.getPreferredCanvasFormat();
  context.configure({
    device,
    format: presentationFormat,
  });

  const response = await fetch(path);
  const wgslCode = await response.text();
  console.log(wgslCode);

  const module = device.createShaderModule({
    label: 'our hardcoded red triangle shaders',
    code: /* wgsl */ wgslCode,
  });

  const pipeline = device.createRenderPipeline({
    label: 'our hardcoded red triangle pipeline',
    layout: 'auto',
    primitive: {
    topology: 'triangle-list',   // 图元拓扑类型：三角形列表
    cullMode: 'none',            // 背面剔除模式：不剔除
    frontFace: 'ccw',           // （可选）正面环绕顺序：逆时针（默认值）
  },
  
  vertex: {
    module,
    entryPoint: 'vert',  // 如果使用新的着色器入口点
  },
  fragment: {
    module,
    entryPoint: 'frag',  // 如果使用新的片元着色器入口点
    targets: [{ format: presentationFormat }],
  },
  });

  const renderPassDescriptor = {
    label: 'our basic canvas renderPass',
    colorAttachments: [
      {
        // view: <- to be filled out when we render
        clearValue: [0.3, 0.3, 0.3, 1],
        loadOp: 'clear',
        storeOp: 'store',
      },
    ],
  };

  function render() {
    // Get the current texture from the canvas context and
    // set it as the texture to render to.
    renderPassDescriptor.colorAttachments[0].view =
        context.getCurrentTexture().createView();

    const encoder = device.createCommandEncoder({ label: 'encoder' });
    const pass = encoder.beginRenderPass(renderPassDescriptor);
    pass.setPipeline(pipeline);
    pass.draw(6);  // call our vertex shader 3 times
    pass.end();

    const commandBuffer = encoder.finish();
    device.queue.submit([commandBuffer]);
  }

  const observer = new ResizeObserver(entries => {
    for (const entry of entries) {
      const canvas = entry.target;
      const width = entry.contentBoxSize[0].inlineSize;
      const height = entry.contentBoxSize[0].blockSize;
      canvas.width = Math.max(1, Math.min(width, device.limits.maxTextureDimension2D));
      canvas.height = Math.max(1, Math.min(height, device.limits.maxTextureDimension2D));
    }
    render();
  });
  observer.observe(canvas);
}

function fail(msg) {
  alert(msg);
}

main();
  