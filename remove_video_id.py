#! /usr/bin/env python3
# Removes the video ID present in the videos downloaded through yt-dlp
import os


def main():
    for filename in os.listdir(os.curdir):
        if not os.path.isdir(filename):
            result = filename.partition("[")
            tail = result[2].partition("]")[2]
            head = result[0].strip()
            new_file_name = head + tail
            os.rename(filename, new_file_name)


if __name__ == '__main__':
    main()
