import codecs
import platform
from datetime import datetime
from urllib.request import urlopen

from bottle import response, route, run


@route("/")
def main():
    with urlopen("http://base") as f:
        value = f.read()
    hashcode = codecs.encode(value, "base64").decode("utf-8").strip()
    lines = [
        "[ Hello KubeCon NA 2019! ]",
        "[ Greetings from Python  ]",
        "[ Code: {} ]".format(hashcode),
        "",
        f"Host: {platform.node()}",
        f"Now:  {datetime.now()}",
    ]
    res = "\n".join(lines) + "\n"
    response.content_type = "text/plain; charset=UTF8"
    return res


run(host="0.0.0.0", port=8000, debug=True, reloader=True)
