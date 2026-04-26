import board
import neopixel
import usb_cdc

pixel_pin = board.P0_08
length = 27
pixels = neopixel.NeoPixel(pixel_pin, length, brightness=0.85, auto_write=False, pixel_order=neopixel.GRB)

for i in range(length):
    pixels[i] = (0, 0, 0)
pixels.show()

serial = usb_cdc.console

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