struct Uniforms {
    progress: f32, // 0.0 空，1.0 满
};

@group(0) @binding(0) var<uniform> u_params: Uniforms;

// 2. 声明纹理和采样器（WGSL 将纹理和采样器分开声明）
@group(0) @binding(1) var u_emptyTex: texture_2d<f32>;
@group(0) @binding(2) var u_fullTex: texture_2d<f32>;
@group(0) @binding(3) var u_sampler: sampler; // 公共采样器

// 3. 顶点着色器传过来的 UV 坐标
struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // 采样两张纹理
    let emptyColor = textureSample(u_emptyTex, u_sampler, in.uv);
    let fullColor = textureSample(u_fullTex, u_sampler, in.uv);
    
    let progress = u_params.progress;

    let edge = 0.02;
    let alpha = smoothstep(progress - edge, progress + edge, in.uv.x);
    return mix(emptyColor, fullColor, alpha);
}