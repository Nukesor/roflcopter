#!/bin/env python
import time
import math
import os
import random


class App:
    def __init__(self, word):
        """Initialize the app and the initial screen."""
        self.word = list(word)
        self.terminal_width = os.get_terminal_size()
        self.sleep_factor = random.randrange(1, 5) / 10

        # Initialize the screen
        self.offset = 0
        for _ in range(0, self.terminal_width.lines):
            self.print_line()
            self.increment_offset()

    def increment_offset(self):
        self.offset += 1
        self.offset = self.offset % len(self.word)

    def calculate_sleep(self, negative: bool):
        """Calculate the current sleep time and whether a new random."""
        sleep_time = math.sin(time.time())

        # Whenever we switch from positive to negative in the sinus curve,
        # we assign a new random sleep_factor is assigned.
        if sleep_time > 0 and negative:
            negative = False
            self.sleep_factor = random.randrange(1, 5) / 10
        elif sleep_time < 0 and not negative:
            negative = True
            self.sleep_factor = random.randrange(1, 5) / 10

        sleep_time = abs(sleep_time) * self.sleep_factor

        return sleep_time

    def print_line(self):
        """Print a new line filled with the word"""
        # Shift the word by the current offset
        first = self.word[0 : self.offset]
        second = self.word[self.offset : len(self.word)]

        compiled_first = "".join(first)
        compiled_second = "".join(second)
        string = f"{compiled_second}{compiled_first} "

        # Fill the line with as many full words as possible
        full_string = string * int(self.terminal_width.columns / (len(self.word) + 1))

        # Fill the remaining space with a partial word
        remaining_length = self.terminal_width.columns - len(full_string)
        full_string += "".join(string[0:remaining_length])

        # PRINT IT
        print(full_string)

    def run(self):
        """The main update loop of this program."""
        negative = True
        while True:
            self.print_line()
            self.increment_offset()
            sleep_time = self.calculate_sleep(negative)
            time.sleep(sleep_time)


def main():
    app = App("ROFLCOPTER")
    app.run()


main()
