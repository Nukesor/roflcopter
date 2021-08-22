#!/bin/env python
import time
import math
import os

terminal_width = os.get_terminal_size()
thingy = "ROFLCOPTER"
length = len(thingy)


def main():
    offset = 0
    for _ in range(0, terminal_width.lines):
        print_line(offset)
        offset += 1
        offset = offset % length

    while True:
        print_line(offset)
        offset += 1
        offset = offset % length
        sleep_time = math.sin(time.time())

        if sle
        time.sleep(math.abs(sleep_time))


def print_line(offset):
    """Print the thingy"""
    first = thingy[0:offset]
    second = thingy[offset:length]
    string = f"{second}{first}"
    full_string = string * int(terminal_width.columns / length)
    print(full_string)


main()
