//d:\Rust\toolkitengine\src\scripts\libs\test.wgsl
// 顶点输出结构体定义（添加重心坐标）
struct VertexOutput {
  @builtin(position) pos : vec4f,
  @location(0) color : vec3f,
  @location(1) barycentric : vec3f,
};

const DELTA : f32 = 0.02;
const LINE_WIDTH : f32 = 0.001+(1.0/ 3.0-0.001)*DELTA;

// 顶点着色器
@vertex
fn vert(@builtin(vertex_index) idx : u32) -> VertexOutput {
  // 顶点位置（NDC 归一化设备坐标）
  let positions = array<vec2f, 3>(
    vec2f( -0.8,  0.6),    // 顶点0
    vec2f( 0.8,  -0.6),    // 顶点1
    vec2f(-0.8,  -0.6),    // 顶点2
  );
  
  // 顶点颜色
  let colors = array<vec3f, 3>(
    vec3f(1.0, 0.0, 0.0),  // 红
    vec3f(0.0, 1.0, 0.0),  // 绿
    vec3f(0.0, 0.0, 1.0),  // 蓝
  );

  // 重心坐标：每个三角形的三个顶点分别对应 (1,0,0), (0,1,0), (0,0,1)
  let barycentrics = array<vec3f, 3>(
    vec3f(1.001, 0.001, 0.001),  // 三角形1 - 顶点A
    vec3f(0.001, 1.001, 0.001),  // 三角形1 - 顶点B
    vec3f(0.001, 0.001, 1.001),  // 三角形1 - 顶点C
  );
  
  var output : VertexOutput;
  output.pos = vec4f(positions[idx], 0.0, 1.0);
  output.color = colors[idx];
  output.barycentric = barycentrics[idx];
  return output;
}

// 片元着色器 - 宽线框核心逻辑
@fragment
fn frag(
  @location(0) color : vec3f,
  @location(1) barycentric : vec3f
) -> @location(0) vec4f {
  // 计算到三条边的最小距离
  // 重心坐标的每个分量表示到对边的距离
  let d = min(barycentric.x, min(barycentric.y, barycentric.z));
  
  // 使用 fwidth 自动适应屏幕分辨率和几何大小
  // line_width 控制线的相对粗细
  let edge_width = LINE_WIDTH;
  
  // smoothstep 实现抗锯齿边缘过渡
  let alpha = smoothstep(edge_width, 0.001, d);

  // 丢弃完全透明的像素（性能优化）
  if (alpha == 0.0) {
    discard;
  }
  
  return vec4f(color, alpha);
}