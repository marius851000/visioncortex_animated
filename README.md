# visioncortex animated
This rust tool apply the vectorisation algoritm of video cortex on every frame of a video.

features :
- it work
- it have multithreading support
- execution can be stopped, and then continued (but this may be imperfect)

method :
- extract every frame of a video to pngs (this will take a lot of disk space)
- apply the visioncortex algo on each frame (with multithreading provided by rayon)
- reassemble them in a single video, transfering audio from the original video

## how to use
create a file named tmp.mp4, and copie it in this folder. then, run make (you need inkscape and cargo installed for the tool to run. Only tested on linux. Should work with no problem with macos and other unix like, but not sure for windows).
