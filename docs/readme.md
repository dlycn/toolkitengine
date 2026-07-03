## cil code
```bash
cargo tree >tree.txt --charset ascii
cargo check
cargo build --bin img2ktx2
cargo clean
cargo cache -a
cargo install wasm-pack

wasm-pack build --target web tools/texturedisplay
```

## bevy feature List
|Feature|Description|
|---|---|
|aac|AAC audio format support (through symphonia)|
|accesskit_unix|Enable AccessKit on Unix backends (currently only works with experimental screen readers and forks.)|
|android-game-activity|Android GameActivity support. Default, choose between this and .android-native-activity|
|android-native-activity|Android NativeActivity support. Legacy, should be avoided for most new Android games.|
|area_light_luts|Include Look Up Tables that are required for area lights.|
|asset_processor|Enables the built-in asset processor for processed assets.|
|async-io|Use async-io’s implementation of block_on instead of futures-lite’s implementation. This is preferred if your application uses async-io.|
|async_executor|Uses as a task execution backend.async-executor|
|basis-universal|Basis Universal compressed texture support|
|bevy_animation|Provides animation functionality|
|bevy_anti_alias|Provides various anti aliasing solutions|
|bevy_asset|Provides asset functionality|
|bevy_audio|Provides audio functionality|
|bevy_camera|Provides camera and visibility types, as well as culling primitives.|
|bevy_camera_controller|Provides a collection of prebuilt camera controllers|
|bevy_ci_testing|Enable systems that allow for automated testing on CI|
|bevy_clipboard|Clipboard resource and management. See for OS-integrated clipboard support.system_clipboard|
|bevy_color|Provides shared color types and operations|
|bevy_core_pipeline|Provides cameras and other basic render pipeline features|
|bevy_debug_stepping|Enable stepping-based debugging of Bevy systems|
|bevy_dev_tools|Provides a collection of developer tools|
|bevy_feathers|Feathers widget collection.|
|bevy_gilrs|Adds gamepad support|
|bevy_gizmos|Adds support for gizmos|
|bevy_gizmos_render|Adds support for rendering gizmos|
|bevy_gltf|glTF support|
|bevy_image|Load and access image data. Usually added by an image format|
|bevy_input_focus|Enable input focus subsystem|
|bevy_light|Provides light types such as point lights, directional lights, spotlights.|
|bevy_log|Enable integration with and tracinglog|
|bevy_material|Provides materials.|
|bevy_mesh|Provides a mesh format and some primitive meshing routines.|
|bevy_mikktspace|Provides vertex tangent generation for use with bevy_mesh.|
|bevy_pbr|Adds PBR rendering|
|bevy_picking|Provides picking functionality without any backend|
|bevy_post_process|Provides post process effects such as depth of field, bloom, chromatic aberration.|
|bevy_remote|Enable the Bevy Remote Protocol|
|bevy_render|Provides rendering functionality|
|bevy_scene|Provides scene functionality|
|bevy_settings|Load and save settings|
|bevy_shader|Provides shaders usable through asset handles.|
|bevy_solari|Provides raytraced lighting (experimental)|
|bevy_sprite|Provides sprite functionality|
|bevy_sprite_render|Provides sprite rendering functionality|
|bevy_state|Enable built in global state machines|
|bevy_text|Provides text functionality|
|bevy_ui|A custom ECS-driven UI framework|
|bevy_ui_debug|Provides a debug overlay for bevy UI|
|bevy_ui_render|Provides rendering functionality for bevy_ui|
|bevy_ui_widgets|Headless widget collection for Bevy UI.|
|bevy_window|Windowing layer|
|bevy_winit|winit window and input backend|
|bevy_world_serialization|Provides ECS serialization functionality|
|bluenoise_texture|Include spatio-temporal blue noise KTX2 file used by generated environment maps, Solari and atmosphere|
|bmp|BMP image format support|
|clipboard_image|Enables image copy/paste via the system clipboard. Not supported on WASM.|
|compressed_image_saver|Enables compressed KTX2 UASTC texture output on the asset processor|
|critical-section|critical-section provides the building blocks for synchronization primitives on all platforms, including .no_std|
|custom_cursor|Enable winit custom cursor support|
|dds|DDS compressed texture support|
|debug|Enable collecting debug information about systems and components to help with diagnostics|
|debug_glam_assert|Enable assertions in debug builds to check the validity of parameters passed to glam|
|default_font|Include a default font, containing only ASCII characters, at the cost of a 20kB binary size increase|
|detailed_trace|Enable detailed trace event logging. These trace events are expensive even when off, thus they require compile time opt-in|
|dfg_lut|Include a preintegrated BRDF Look Up Table for more accurate specular shading.|
|dlss|NVIDIA Deep Learning Super Sampling|
|dynamic_linking|Force dynamic linking, which improves iterative compile times|
|embedded_watcher|Enables watching in memory asset providers for Bevy Asset hot-reloading|
|experimental_pbr_pcss|Enable support for PCSS, at the risk of blowing past the global, per-shader sampler limit on older/lower-end GPUs|
|exr|EXR image format support|
|ff|Farbfeld image format support|
|file_watcher|Enables watching the filesystem for Bevy Asset hot-reloading|
|flac|FLAC audio format support (through claxon)|
|force_disable_dlss|Forcibly disable DLSS so that cargo build –all-features works without the DLSS SDK being installed. Not meant for users.|
|free_camera|Enables the free cam from bevy_camera_controller|
|gamepad|Gamepad support. Automatically enabled by .bevy_gilrs|
|gestures|Gestures support. Automatically enabled by .bevy_window|
|ghost_nodes|Experimental support for nodes that are ignored for UI layouting|
|gif|GIF image format support|
|glam_assert|Enable assertions to check the validity of parameters passed to glam|
|gltf_animation|Enable glTF animation loading|
|hdr|HDR image format support|
|hotpatching|Enable hotpatching of Bevy systems|
|http|Enables downloading assets from HTTP sources. Warning: there are security implications. Read the docs on WebAssetPlugin.|
|https|Enables downloading assets from HTTPS sources. Warning: there are security implications. Read the docs on WebAssetPlugin.|
|ico|ICO image format support|
|jpeg|JPEG image format support|
|keyboard|Keyboard support. Automatically enabled by .bevy_window|
|ktx2|KTX2 compressed texture support|
|libm|Uses the maths library instead of the one provided in and .libmstdcore|
|mesh_picking|Provides an implementation for picking meshes|
|meshlet|Enables the meshlet renderer for dense high-poly scenes (experimental)|
|meshlet_processor|Enables processing meshes into meshlet meshes for bevy_pbr|
|morph|Enables support for morph target weights in bevy_mesh|
|morph_animation|Enables bevy_mesh and bevy_animation morph weight support|
|mouse|Mouse support. Automatically enabled by .bevy_window|
|mp3|MP3 audio format support (through symphonia)|
|mp4|MP4 audio format support (through ). It also enables AAC support.symphonia|
|multi_threaded|Enables multithreaded parallelism in the engine. Disabling it forces all engine tasks to run on a single thread.|
|pan_camera|Enables the pan camera from bevy_camera_controller|
|pbr_anisotropy_texture|Enable support for anisotropy texture in the , at the risk of blowing past the global, per-shader texture limit on older/lower-end GPUsStandardMaterial|
|pbr_clustered_decals|Enable support for Clustered Decals|
|pbr_light_textures|Enable support for Light Textures|
|pbr_multi_layer_material_textures|Enable support for multi-layer material textures in the , at the risk of blowing past the global, per-shader texture limit on older/lower-end GPUsStandardMaterial|
|pbr_specular_textures|Enable support for specular textures in the , at the risk of blowing past the global, per-shader texture limit on older/lower-end GPUsStandardMaterial|
|pbr_transmission_textures|Enable support for transmission-related textures in the , at the risk of blowing past the global, per-shader texture limit on older/lower-end GPUsStandardMaterial|
|png|PNG image format support|
|pnm|PNM image format support, includes pam, pbm, pgm and ppm|
|qoi|QOI image format support|
|raw_vulkan_init|Forces the wgpu instance to be initialized using the raw Vulkan HAL, enabling additional configuration|
|reflect_auto_register|Enable automatic reflect registration|
|reflect_auto_register_static|Enable automatic reflect registration without inventory. See for more info.reflect::load_type_registrations|
|reflect_documentation|Enables bevy_reflect to access documentation comments of rust code at runtime|
|reflect_functions|Enable function reflection|
|schedule_data|Enable collecting schedule data from the app.|
|serialize|Enable serialization support through serde|
|shader_format_glsl|Enable support for shaders in GLSL|
|shader_format_spirv|Enable support for shaders in SPIR-V|
|shader_format_wesl|Enable support for shaders in WESL|
|smaa_luts|Include SMAA Look Up Tables KTX2 Files|
|spirv_shader_passthrough|Enable passthrough loading for SPIR-V shaders (Only supported on Vulkan, shader capabilities and extensions must agree with the platform implementation)|
|sprite_picking|Provides an implementation for picking sprites|
|statically-linked-dxc|Statically linked DXC shader compiler for DirectX 12|
|std|Allows access to the crate.std|
|symphonia-flac|FLAC audio format support (through symphonia)|
|symphonia-vorbis|OGG/VORBIS audio format support (through symphonia)|
|symphonia-wav|WAV audio format support (through symphonia)|
|sysinfo_plugin|Enables system information diagnostic plugin|
|system_clipboard|Enables system-level clipboard support.|
|system_font_discovery|Allows for discovery of preloaded system fonts|
|tga|TGA image format support|
|tiff|TIFF image format support|
|tonemapping_luts|Include tonemapping Look Up Tables KTX2 files. If everything is pink, you need to enable this feature or change the method for your or .TonemappingCamera2dCamera3d|
|touch|Touch support. Automatically enabled by .bevy_window|
|trace|Tracing support|
|trace_chrome|Tracing support, saving a file in Chrome Tracing format|
|trace_tracy|Tracing support, exposing a port for Tracy|
|trace_tracy_memory|Tracing support, with memory profiling, exposing a port for Tracy|
|track_location|Enables source location tracking for change detection and spawning/despawning, which can assist with debugging|
|type_label_buffers|Pre-populate buffer labels with buffer types for debugging.|
|ui_picking|Provides an implementation for picking UI|
|vorbis|OGG/VORBIS audio format support (through lewton)|
|wav|WAV audio format support (through hound)|
|wayland|Wayland display server support|
|web|Enables use of browser APIs. Note this is currently only applicable on architectures.wasm32|
|web_asset_cache|Enable caching downloaded assets on the filesystem. NOTE: this cache currently never invalidates entries!|
|webgl2|Enable some limitations to be able to use WebGL2. Please refer to the WebGL2 and WebGPU section of the examples README for more information on how to run Wasm builds with WebGPU.|
|webgpu|Enable support for WebGPU in Wasm. When enabled, this feature will override the feature and you won’t be able to run Wasm builds with WebGL2, only with WebGPU.webgl2|
|webp|WebP image format support|
|x11|X11 display server support|
|zlib|For KTX2 supercompression|
|zstd_c|For KTX2 Zstandard decompression using zstd. This is a faster backend, but uses unsafe C bindings. For the safe option, stick to the default backend with “zstd_rust”.|
|zstd_rust|For KTX2 Zstandard decompression using pure rust ruzstd. This is the safe default. For maximum performance, use “zstd_c”.|