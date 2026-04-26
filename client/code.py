import board
import neopixel

pixel_pin = board.P0_08
length = 27
pixels = neopixel.NeoPixel(pixel_pin, length, brightness=0.5, auto_write=False, pixel_order=neopixel.GRB)

for i in range(length):
    pixels[i] = (0, 255, 0)
pixels.show()