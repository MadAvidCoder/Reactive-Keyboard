import serial
import time

ser = serial.Serial("COM11", 115200)

ser.write(b"0,255,0,0\n")
ser.write(b"1,255,255,0\n")
ser.write(b"2,0,255,0\n")
ser.write(b"3,0,255,255\n")
ser.write(b"4,0,0,255\n")