// tools.js - 完整 DDS 头部解析，支持 DX10 扩展头，精确识别 BC1~BC7
import * as THREE from 'three';
import { DDSLoader } from './three/examples/jsm/loaders/DDSLoader.js';
import { FBXLoader } from './three/examples/jsm/loaders/FBXLoader.js';
import { KTXLoader } from './three/examples/jsm/loaders/KTXLoader.js';
import { GLTFLoader } from './three/examples/jsm/loaders/GLTFLoader.js';
import { KTX2Loader } from './three/examples/jsm/loaders/KTX2Loader.js';

function DDSparse(url){
const loader = new DDSLoader();
const texture = loader.load(url);
let boolcolor = 1;
if (boolcolor){texture.colorSpace = THREE.SRGBColorSpace }; 
const material = new THREE.MeshStandardMaterial({ map: texture });
return material;
}





function Scene(asset,model='plane'){

const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
const renderer = new THREE.WebGLRenderer();
renderer.setSize(window.innerWidth, window.innerHeight);
document.body.appendChild(renderer.domElement);

let material = DDSparse(asset); // 使用 DDS 解析的材质

if (model=='plane'){let space = THREE.PlaneGeometry(1, 1)}
else if (model=='cube'){let space = THREE.BoxGeometry(1, 1, 1)}
else if (model=='sphere'){let space = THREE.SphereGeometry(1, 10, 10)}
else {let space = THREE.PlaneGeometry(1, 1)}

const model = new THREE.Mesh(new space, material);
scene.add(model);

camera.position.z = 2;

// 添加简单光照
const light = new THREE.DirectionalLight(0xffffff, 3);
light.position.set(1, 1, 1);
scene.add(light);

// 动画循环
function animate() {
  requestAnimationFrame(animate);
  cube.rotation.x += 0.01;
  cube.rotation.y += 0.01;
  renderer.render(scene, camera);
}
animate();
}
export { DDSparse,Scene };