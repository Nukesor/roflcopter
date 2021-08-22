colors = {
    "bright_red": "91",
    "bright_orange": "38;5;208",
    "bright_yellow": "93",
    "bright_green": "92",
    "bright_cyan": "96",
    "bright_blue": "38;5;27",
    "bright_purple": "38;5;99",
    "bright_magenta": "95",
    "bright_white": "38;5;231",
    "bright_pink": "38;5;213",
}


modes = {
    "reset": "0",
    "bold": "1",
    "blinking": "5",
}


def style(text: str, color: str, blinking: bool = False) -> str:
    """Apply ansi styling to a given text"""

    color = colors[color]
    attributes = [color]

    if blinking:
        attributes.append(modes["blinking"])

    joined_attributes = ";".join(attributes)
    styled = f"\x1b[{joined_attributes}m{text}\x1b[0m"

    return styled
