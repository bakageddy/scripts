#!/usr/bin/env sh

yt-dlp $1 --embed-chapters --format 22 -I "1:25" -N 4
