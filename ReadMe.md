# Chinge Bot

Rhythm game automation bot that detects colored notes and presses keys.

## Functions

- `main()` - Initializes bot with monitor and lane configuration, starts detection loop
- `detect_and_play()` - Captures screen area and triggers key presses when notes detected
- `note_in_hitzone()` - Scans pixels near strike line for matching note colors
- `color_matches()` - Compares pixel color to target with threshold tolerance
- `get_compositor()` - Detects Wayland/X11 display environment


## Positions
yTop: 1225
yBottom: 1269

#### Lt
xLeft: 400
xRight: 488

#### Lb
xLeft: 623
xRight: 737

#### DPadUp
xLeft: 950
xRight: 1045

#### Y
xLeft: 1516
xRight: 1608 

#### Rb
xLeft: 1789
xRight: 1888

#### Rt
xLeft: 2070
xRight: 2161

