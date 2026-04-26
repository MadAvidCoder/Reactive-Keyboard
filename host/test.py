import serial
import time

ser = serial.Serial("COM11", 115200)
time.sleep(3)

ser.write(b"0,255,0,0\n")
ser.write(b"1,0,255,0\n")
ser.write(b"2,0,0,255\n")