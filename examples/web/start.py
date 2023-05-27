#!/usr/bin/python3

from http.server import BaseHTTPRequestHandler, HTTPServer
import os

hostName = "localhost"
serverPort = 8080

script_dir: str = os.path.realpath(os.path.dirname(__file__))
files: dict = {
    "/": {
        "path": os.path.join(script_dir, "index.html"),
        "content-type": "text/html"
    },
    "/main.js": {
        "path": os.path.join(script_dir, "main.js"),
        "content-type": "text/javascript"
    },
    "/main.worker.js": {
        "path": os.path.join(script_dir, "main.worker.js"),
        "content-type": "text/javascript"
    },
    "/main.wasm": {
        "path": os.path.join(script_dir, "main.wasm"),
        "content-type": "application/wasm"
    }
}

class MyServer(BaseHTTPRequestHandler):
    def send_404(self):
        self.send_response(404)
        self.send_header("Content-type", "text/html")
        self.end_headers()

        self.wfile.write(bytes("<!DOCTYPE html><html><head><title>404 - File Not Found</title></head>", "utf-8"))
        self.wfile.write(bytes("<p>Request: %s</p>" % self.path, "utf-8"))
        self.wfile.write(bytes("<body>", "utf-8"))
        self.wfile.write(bytes("<h1>404 - File Not Found</h1>", "utf-8"))
        self.wfile.write(bytes("</body></html>", "utf-8"))

    def send_file(self, file_path: str, content_type: str):
        self.send_response(200)
        self.send_header("Content-type", content_type)

        # For Page Loading Wasm
        self.send_header("Cross-Origin-Opener-Policy", "same-origin")
        self.send_header("Cross-Origin-Embedder-Policy", "require-corp")

        self.end_headers()

        with open(file=file_path, mode="rb") as f:
            self.wfile.write(f.read())

    def get_file(self, url_path: str):
        if url_path in files:
            if os.path.exists(files[url_path]["path"]):
                self.send_file(file_path = files[url_path]["path"], content_type=files[url_path]["content-type"])
            else:
                self.send_404()
        else:
            self.send_404()

    def do_GET(self):
        self.get_file(url_path=self.path)

if __name__ == "__main__":        
    webServer = HTTPServer((hostName, serverPort), MyServer)
    print("Server started http://%s:%s" % (hostName, serverPort))
    print("-"*40)
    print("This is not a production server. It only exists for demonstration purposes. Use a production server instead")
    print("-"*40)

    try:
        webServer.serve_forever()
    except KeyboardInterrupt:
        pass

    webServer.server_close()
    print("Server stopped.")
