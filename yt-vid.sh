#!/usr/bin/env sh

yt-dlp $1 --write-auto-subs --sub-langs "en" --embed-chapters --embed-subs
