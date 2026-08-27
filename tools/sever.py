import http.server
import webbrowser
import threading
import logging
logging.basicConfig(level=logging.INFO,format="%(asctime)s - %(levelname)s - %(message)s")
from pathlib import Path
import os
if __name__ == "__main__":

    host = "127.0.0.1"
    port = 8000
    path = Path(os.getcwd()).parent
    index = "mc.html"
    os.chdir(path)
    threading.Timer(0.5,lambda:
        webbrowser.open(f"http://{host}:{port}/tools/{index}")).start()
    logging.info(f"server start at http://{host}:{port}/tools/{index}")
    http.server.HTTPServer(
        (host, port),
        http.server.SimpleHTTPRequestHandler).serve_forever()
