VIDEO_INPUT ?= ./tmp.mp4
TMP_DIR ?= tmp
VF ?=
# -vf ...
RENDER_DIR := out

all: out/frames_trans

out/frames: $(VIDEO_INPUT)
	rm -rf $(TMP_DIR)/frames
	mkdir -p $(TMP_DIR)/frames
	ffmpeg -i $(VIDEO_INPUT) $(VF) -vsync 0 $(TMP_DIR)/frames/%06d.png
	mkdir -p $(RENDER_DIR)/frames
	mv $(TMP_DIR)/frames/* $(RENDER_DIR)/frames/

.PHONY: out/frames_trans

out/frames_trans: out/frames
	cargo run --release -- batch out/frames out/frames_trans
	ffmpeg  -framerate 30 -pattern_type glob -i "out/frames_trans/*.png.svg.png" -i $(VIDEO_INPUT) -c:a copy -map 0:v:0 -map 1:a:0 final.mp4

clean:
	rm -rf out tmp
