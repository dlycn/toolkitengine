import subprocess
import tempfile
import shlex
import os
from flask import Flask, request, jsonify

app = Flask(__name__)
path = os.path.abspath(__file__)
dir = os.path.dirname(path)
h5path = os.path.join(dir, "app.html")

# 允许的命令白名单（只允许这两个程序）
ALLOWED_COMMANDS = {
    'ffmpeg': 'ffmpeg',
    'basisu': 'basisu', # 或绝对路径
    'bcdec':'bcdec'}

# 根路由：返回前端 HTML 页面
@app.route('/')
def index():
    return open(h5path, 'r', encoding='utf-8').read()

@app.route('/exec', methods=['POST'])
def execute():
    data = request.get_json()
    tool = data.get('tool')
    args_str = data.get('args', '').strip()
    
    if tool not in ALLOWED_COMMANDS:
        return jsonify({'output': f'未知工具: {tool}'}), 400
    
    try:
        args = shlex.split(args_str)
    except Exception as e:
        return jsonify({'output': f'参数解析错误: {e}'}), 400
    
    cmd = [ALLOWED_COMMANDS[tool]] + args
    
    try:
        with tempfile.TemporaryDirectory() as tmpdir:
            result = subprocess.run(
                cmd,
                cwd=tmpdir,
                capture_output=True,
                timeout=30,
                encoding='utf-8',
                errors='replace'
            )
            output = result.stdout
            if result.stderr:
                output += '\n[stderr]\n' + result.stderr
            if result.returncode != 0:
                output = f'退出码 {result.returncode}\n{output}'
            return jsonify({'output': output or '(无输出)'})
    except subprocess.TimeoutExpired:
        return jsonify({'output': '命令执行超时（30秒）'}), 408
    except Exception as e:
        return jsonify({'output': f'执行异常: {str(e)}'}), 500

if __name__ == '__main__':
    app.run(host='127.0.0.1', port=3000, debug=True)