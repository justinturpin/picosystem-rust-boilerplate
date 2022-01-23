#!/usr/bin/env python3

"""
Convert all PNG's in assets/* to .16bpp images.
"""

from pathlib import Path

import struct

from PIL import Image


def convert_to_16bpp(path: Path):
    """
    Convert an image to a 16bpp format
    """

    im = Image.open(path)

    buffer = b""

    for y in range(im.height):
        for x in range(im.width):
            rgb = im.getpixel((x, y))

            if im.mode == "RGB":
                r, g, b, a = rgb[0], rgb[1], rgb[2], 255
            elif im.mode == "RGBA":
                r, g, b, a = rgb[0], rgb[1], rgb[2], rgb[3]

            # Assume the pixel format is ARGB4444

            a = 15 * a // 255
            r = 15 * r // 255
            g = 15 * g // 255
            b = 15 * b // 255

            color = (a << 12) | (r << 8) | (g << 4) | b

            buffer += struct.pack(">H", color)

    dest = Path(path.parent, path.stem + ".16bpp")
    dest.write_bytes(buffer)



for path in Path(".").glob("assets/*.png"):
    print(f"Converting {path} to 16bpp")

    convert_to_16bpp(path)
