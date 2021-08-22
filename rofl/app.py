import time
import math
import os
import random

from rofl.colors import style, tty_colors


class App:
    def __init__(self, word: str):
        """Initialize the app and the initial screen."""
        self.modes = {
            "bold": {"counter": 0, "interval": 2},
            # "italic": {"counter": 0, "interval": 3},
            # "inverse": {"counter": 0, "interval": 15},
            # "blinking": {"counter": 0, "interval": 3},
        }
        self.bold_interval = 0

        self.original_word = word
        self.style_word()

        self.terminal_width = os.get_terminal_size()
        self.sleep_factor = random.randrange(1, 5) / 10

        # Initialize the screen
        self.offset = 0
        for _ in range(0, self.terminal_width.lines):
            self.print_line()
            self.increment_offset()

    def style_word(self):
        word = []
        modes = []

        # Apply all modes in their specified intervals
        for key, info in self.modes.items():
            info["counter"] += 1
            info["counter"] = info["counter"] % info["interval"]
            if info["counter"] == 0:
                modes.append(key)

        # Give every character their own color.
        # Each character is prefixed with an ASCII escape sequence.
        # The mode is resetted after each character as well.
        color_keys = list(tty_colors.keys())
        for index, char in enumerate(list(self.original_word)):
            index = index % len(color_keys)
            color = color_keys[index]
            word.append(style(char, color, modes))

        return word

    def increment_offset(self):
        self.offset += 1
        self.offset = self.offset % len(self.original_word)

    def calculate_sleep(self, negative: bool) -> float:
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
        word = self.style_word()

        # Shift the word by the current offset
        start = word[0 : self.offset]
        end = word[self.offset : len(word)]
        shifted_chars = end + start

        rotated_string = "".join(shifted_chars) + " "

        # Fill the line with as many full words as possible
        full_repetitions = int(self.terminal_width.columns / (len(word) + 1))
        full_string = rotated_string * full_repetitions
        total_length = (len(word) + 1) * full_repetitions

        # Fill the remaining space with a partial word
        remaining_length = self.terminal_width.columns - total_length
        full_string += "".join(shifted_chars[0:remaining_length])

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
