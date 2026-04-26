import serial
import time

ser = serial.Serial("COM11", 115200)

time.sleep(1)
ser.reset_input_buffer()
ser.reset_output_buffer()

ser.write(b"START\n")
for i in range(27):
    line = f"{i},{255},{0},{0}\n"
    ser.write(line.encode('utf-8'))
    time.sleep(0.002)

ser.write(b"END\n")