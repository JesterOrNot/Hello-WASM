from flask import Flask, request

app = Flask(__name__, static_url_path="")


@app.route("/")
def root():
    return open("index.html").read()


app.run(port=3000)
