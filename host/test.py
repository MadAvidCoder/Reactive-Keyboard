import serial
import time

ser = serial.Serial("COM11", 115200)

time.sleep(1)
ser.reset_input_buffer()
ser.reset_output_buffer()

out_buffer = [(i, (0,0,0)) for i in range(27)]
 
for i in range(27):
    out_buffer[i] = (i, (0, 255, 255))

ser.write(b"START;" + b";".join(f"{idx},{r},{g},{b}".encode('utf-8') for idx, (r, g, b) in out_buffer) + b";END\n")