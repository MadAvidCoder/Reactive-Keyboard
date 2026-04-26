import board
import neopixel
import usb_cdc
import usb_hid
import time

from digitalio import DigitalInOut, Direction, Pull
from adafruit_hid.keyboard import Keyboard
from adafruit_hid.keycode import Keycode

pixel_pin = board.P0_08
length = 27
pixels = neopixel.NeoPixel(pixel_pin, length, brightness=0.85, auto_write=False, pixel_order=neopixel.GRB)

for i in range(length):
    pixels[i] = (0, 0, 0)
pixels.show()

serial = usb_cdc.console

kbd = Keyboard(usb_hid.devices)

ROWS = 4
COLS = 6

keymap = [
    Keycode.A, Keycode.B, Keycode.C, Keycode.D, Keycode.E, Keycode.F,
    Keycode.G, Keycode.H, Keycode.I, Keycode.J, Keycode.K, Keycode.L,
    Keycode.M, Keycode.N, Keycode.O, Keycode.P, Keycode.Q, Keycode.R,
    Keycode.S, Keycode.T, Keycode.U, Keycode.V, Keycode.W, Keycode.X
]

row_pins = [
    DigitalInOut(board.P1_15),
    DigitalInOut(board.P1_13),
    DigitalInOut(board.P1_11),
    DigitalInOut(board.P0_10),
]

col_pins = [
    DigitalInOut(board.P0_11),
    DigitalInOut(board.P1_00),
    DigitalInOut(board.P0_24),
    DigitalInOut(board.P0_22),
    DigitalInOut(board.P0_20),
    DigitalInOut(board.P0_17),
]

for r in row_pins:
    r.direction = Direction.OUTPUT
    r.value = True

for c in col_pins:
    c.direction = Direction.INPUT
    c.pull = Pull.UP

pressed = [[False for _ in range(COLS)] for _ in range(ROWS)]

def index(r, c):
    return r * COLS + c

def parse(line):
    try:
        parts = line.strip().split(',')
        if len(parts) != 4:
            return None
        idx = int(parts[0])
        r = int(parts[1])
        g = int(parts[2])
        b = int(parts[3])
        return idx, (r, g, b)
    except:
        return None

print("Active.")

while True:
    for r in range(ROWS):
        for row in row_pins:
            row.value = True

        row_pins[r].value = False

        time.sleep(0.001)

        for c in range(COLS):
            is_pressed = not col_pins[c].value
            idx = index(r, c)

            if is_pressed and not pressed[r][c]:
                pressed[r][c] = True
                if idx < len(keymap):
                    kbd.press(keymap[idx])

            elif not is_pressed and pressed[r][c]:
                pressed[r][c] = False
                if idx < len(keymap):
                    kbd.release(keymap[idx])

    if serial.in_waiting:
        line = serial.readline().decode('utf-8').strip()

        if not (line.startswith("START;") and line.endswith(";END")):
            continue
        
        contents = line.split(';')[1:-1]
        buffer = [(0,0,0)] * length
        confirmed = [False] * length

        for item in contents:
            p = parse(item)
            if p:
                idx, color = p
                if 0 <= idx < length:
                    buffer[idx] = color
                    confirmed[idx] = True

        if all(confirmed):
            for idx, color in enumerate(buffer):
                pixels[idx] = color
            pixels.show()