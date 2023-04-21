import serial
import socket
import struct
import sys
import time

def connect_serial():
    try:
        arduino = serial.Serial(port='/dev/ttyACM0', 
                                baudrate=115200, 
                                parity=serial.PARITY_NONE, 
                                stopbits=serial.STOPBITS_ONE, 
                                bytesize=serial.EIGHTBITS, 
                                timeout=0.5, 
                                inter_byte_timeout=0.1
                                )
    except:
        print("ERROR: nothing connected to /dev/ttyACM0")
        sys.exit(2)
    return arduino


def write_read(x, arduino):
    arduino.write(bytes(x, 'utf-8'))
    #time.sleep(0.05)
    #data = ""
    #data = arduino.readline()
    #return data


def test():
    arduino = connect_serial()
    while True:
        num = input("Enter a number: ")
        value = write_read(num, arduino)
        print(value)

def formatter(msg, max_msg, max_arduino):
    return msg * max_arduino / max_msg

def run():
    # Connecto to Arduino Serial
    arduino = connect_serial()

    print("-")
    time.sleep(4)
    rpm = 0
    print("|")
    while rpm <= 180:
        write_read(">||" + str(formatter(rpm * 100, 10000, 180)) + "|||<", arduino)
        #value = write_read(">||" + str(formatter(rpm * 100, 10000, 180)) + "|||<", arduino)
        print(".")
        rpm += 1
        time.sleep(0.05)

run()
