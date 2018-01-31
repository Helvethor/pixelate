# Pixelate

This utility pixelates an input image by segmenting the image into squares of `factor` width. The average value for each segment is applied to the whole segment.


## Why ?

I tried using `convert` and `magick` for this, but they were too slow for my taste. I use this program to draw a lockscreen image, the ~600ms latency added by those softwares was not reactive enough.

This quick and dirty implementation runs ~3x faster, no rigorous tests were made...
