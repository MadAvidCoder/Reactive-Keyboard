import board
import neopixel
import usb_cdc

pixel_pin = board.P0_08
length = 27
pixels = neopixel.NeoPixel(pixel_pin, length, brightness=0.5, auto_write=False, pixel_order=neopixel.GRB)

serial = usb_cdc.console

in_buffer = [(0,0,0)] * length
confirmed = [False] * length

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

        if line == "START":
            in_buffer = [(0, 0, 0)] * length
            confirmed = [False] * length
            continue
        
        elif line == "END":
            if all(confirmed):
                if len(in_buffer) == length:
                    for i, color in enumerate(in_buffer):
                        pixels[i] = color
                    pixels.show()
            continue

        p = parse(line)

        if p:
            idx, color = p
            if 0 <= idx < length:
                confirmed[idx] = True
                in_buffer[idx] = color