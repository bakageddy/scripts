#!/usr/bin/env python3

import subprocess


def main():
    result = subprocess.check_output("acpi").decode("utf-8")
    batteries = []
    for line in result.splitlines():
        for words in line.split(","):
            if ("%" in words.strip()):
                batteries.append(int(words.strip().removesuffix("%")))
    percentage = (sum(batteries)//len(batteries))

    if ("discharging" in result.lower()):
        status = "-"
    else:
        status = "+"
    print(f"{status}{percentage}")

    # if ("discharging" in result.lower()):
    #     status = ""
    # else:
    #     status = ""
    # print(f"{status} {percentage}")


if __name__ == '__main__':
    main()
