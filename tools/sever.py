import http.server
import webbrowser
import threading
from pathlib import Path
import os

if __name__ == "__main__":

    host = "127.0.0.1"
    port = 8000
    path = Path(os.getcwd()).parent
    #index = "wgsl.html"
    index = fr"tools\mc.html"
    os.chdir(path)
    threading.Timer(0.5,lambda:
        webbrowser.open(f"http://{host}:{port}/{index}")).start()
    http.server.HTTPServer(
        (host, port),
        http.server.SimpleHTTPRequestHandler).serve_forever()
