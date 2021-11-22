#!/usr/bin/env python3

import os
import wand
import wand.image

our_dir = os.path.dirname(os.path.realpath(__file__))

sets = [
    "archer",
    "knight",
    "mage",
    "player",
]

anim_spec = [
    ("cast_up",     7),
    ("cast_left",   7),
    ("cast_down",   7),
    ("cast_right",  7),

    ("spear_up",    8),
    ("spear_left",  8),
    ("spear_down",  8),
    ("spear_right", 8),

    ("walk_up",     9),
    ("walk_left",   9),
    ("walk_down",   9),
    ("walk_right",  9),

    ("stab_up",     6),
    ("stab_left",   6),
    ("stab_down",   6),
    ("stab_right",  6),

    ("bow_up",     13),
    ("bow_left",   13),
    ("bow_down",   13),
    ("bow_right",  13),

    ("die",         6),
]

for set in sets:
    set_dir = f"{our_dir}/{set}"
    for y, (anim, anim_len) in enumerate(anim_spec):
        for x in range(0, anim_len):
            with wand.image.Image(filename=f"{set_dir}/grid.png") as img:
                img.crop(x * 64, y * 64, width=64, height=64)
                anim_dir = f"{set_dir}/{anim}"
                if not os.path.isdir(anim_dir):
                    os.makedirs(anim_dir)
                filename = f"{anim_dir}/{x:02}.png"
                img.save(filename = filename)
                print(f"Saved {set_dir}/({x}, {y}) to '{filename}'")
