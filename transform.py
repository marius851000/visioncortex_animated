import os
import subprocess
os.makedirs("out/frames_trans", exist_ok=True)

SOURCE_DIR = "out/frames"
files = os.listdir(SOURCE_DIR)

for file_name in files:
    source_path = os.path.join(SOURCE_DIR, file_name)
    target_path = os.path.join("out/frames_trans", file_name + ".svg")
    if not os.path.isfile(target_path+".png"):
        subprocess.check_call(["cargo", "run", "--release", "--", "-i", source_path, "-o", target_path])
        subprocess.check_call(["inkscape", target_path, "-o", target_path + ".png", "-C"])
