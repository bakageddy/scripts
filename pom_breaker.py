#! /usr/bin/env python3

# Make the transcript from POM lectures into 4096 words
# Made this script to get around ChatGPT's token limit per prompt...
import sys


def main():
    if (len(sys.argv) < 2):
        print("Usage: ./pom_breaker.py file.txt")
        exit(1)

    data = open(sys.argv[1]).read()
    data_len = len(data)
    times = data_len // 4096
    try:
        with open("out.txt", "w") as file:
            ptr = 0
            for i in range(times):
                file.write(data[ptr: ptr + 4096] + '\n')
                ptr += 4096
            file.write(data[ptr:] + '\n')
    except Exception:
        print("Hehe")


if __name__ == '__main__':
    main()
