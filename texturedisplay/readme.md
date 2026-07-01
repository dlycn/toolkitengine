
## code
```bash
# ======================== Basis Universal (basisu) ========================
# 压缩：PNG/TGA/JPG/QOI → KTX2（默认 ETC1S 模式）
basisu input.png -output_file output.ktx2
# 压缩：使用 UASTC 高质量模式（更大但质量更好）
basisu input.png -uastc -output_file output.ktx2
# 解包：KTX2/Basis → 多种格式（默认生成 KTX + PNG + DDS + ASTC 等一堆文件）
basisu input.ktx2 -unpack
# 解包：只输出 ETC1 格式的 PNG（不生成 KTX/DDS/ASTC 等）
basisu input.ktx2 -unpack -no_ktx -etc1_only
# 解包：只输出 KTX 容器文件（不生成 PNG）
basisu input.ktx2 -unpack -ktx_only -etc1_only
# 验证文件是否可被解码（不产生任何输出文件）
basisu input.ktx2 -validate
# 查看文件信息
basisu input.ktx2 -info
# ============================== bcdec (BCn/DDS) ==============================
# 解码 DDS 文件为 TGA（默认输出同名的 .tga 文件）
bcdec path/to/texture.dds
# 注意：bcdec 不支持指定输出格式，始终输出 TGA，且没有 PNG 选项
# 如需 PNG 可用 ffmpeg 二次转换：ffmpeg -i texture.tga texture.png
# ============================== astcenc (ASTC) ==============================
# 解码 ASTC 文件为 TGA
astcenc -d input.astc output.tga
# 编码 PNG 为 ASTC（4x4 块，质量中等）
astcenc -c input.png output.astc 4x4 -medium
# ============================== etcpack (ETC1/ETC2) ==============================
# 解码 ETC 文件为 TGA
etcpack input.ktx output.tga
# 编码 PNG 为 ETC1（PKM 容器）
etcpack input.png output.pkm -c etc1
# ================================ FFmpeg (通用) ================================
# TGA → PNG（最常用）
ffmpeg -i input.tga output.png
# PNG → TGA
ffmpeg -i input.png output.tga
# 批量转换当前目录所有 TGA 为 PNG
for f in *.tga; do ffmpeg -i "$f" "${f%.tga}.png"; done
# ================================ 关键参数解释 ================================
# basisu -unpack        : 解包模式
# basisu -no_ktx        : 不生成 .ktx 容器文件
# basisu -ktx_only      : 只生成 .ktx 容器，不生成 .png
# basisu -etc1_only     : 只转码为 ETC1 格式（跳过 BC/ASTC/PVRTC 等）
# basisu -validate      : 仅验证，不输出任何文件
# bcdec                 : 无额外参数，输出固定为 TGA
# astcenc -d            : decode 模式
# etcpack -c etc1       : 编码为 ETC1
# ffmpeg -i             : 指定输入文件
```