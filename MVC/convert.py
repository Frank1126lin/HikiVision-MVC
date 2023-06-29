from base64 import encode
import os


# convert the gbk encoding to utf8 encoding
path = "."
dst = "./dst/"
if not os.path.exists(dst):
    os.mkdir(dst)
for f in os.listdir(path):
    if os.path.isdir(f):
        continue
    print(f)
    if f.endswith(".cpp") or f.endswith(".h"):
        with open(os.path.join(path, f), 'r', encoding="gbk") as file:
            content = file.read()
        
        with open(os.path.join(dst, f), 'w', encoding="utf-8") as dest:
            dest.write(content)
