# Anki-tan's face

Anki-tan's `face` is the pack's app icon. `anki-tan/face.png` is that icon with the rounded tile, its
frame and the green grid behind her cut away. No pixel inside the mask
changed.

![Review on dark, grey, white and at 64 px](review.png)

| File | SHA-256 |
|---|---|
| `anki-tan/icon.png` (source, pack `icon.png`) | `388648cd15b2015ad34725591f31db3ae36c073170dbc098929df147042cdde1` |
| `anki-tan/face.png` | `0e01672d2fb48e4ab47eed5f2c3f01edb0ee2a1d3611e28c865b2d85744d3988` |

The mask is a polygon traced by hand on the 256 px icon, in pixels:

```
35,107 37,85 50,70 65,57 75,42 95,32 125,28 160,27 175,23 180,21 190,27
205,45 215,51 234,50 234,240 78,240 76,220 69,200 59,182 50,162 37,140
35,120
```

Made with ImageMagick 6.9.12:

```sh
convert -size 256x256 xc:black -fill white -draw "polygon <points>" mask.png
convert anki-tan/icon.png \( mask.png -blur 0x0.6 \) -alpha off \
  -compose CopyOpacity -composite anki-tan/face.png
```

The tile cut her hair and shoulders, so the right and bottom edges are
straight. The left star behind her head is left out. A drawn face would
replace this one.

Anki-tan by Shigeyuki (https://www.patreon.com/Shigeyuki). CC BY-SA 4.0
under [her LICENSE](../../../anki-tan/LICENSE).
