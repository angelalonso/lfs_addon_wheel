import serial
import socket
import struct
import sys
import time

def connect_serial():
    try:
        arduino = serial.Serial(port='/dev/ttyACM0', baudrate=115200, timeout=.1)
    except:
        print("ERROR: nothing connected to /dev/ttyACM0")
        sys.exit(2)
    return arduino


def write_read(x, arduino):
    arduino.write(bytes(x, 'utf-8'))
    time.sleep(0.05)
    data = arduino.readline()
    return data


def test():
    arduino = connect_serial()
    while True:
        num = input("Enter a number: ")
        value = write_read(num, arduino)
        print(value)

def get_data():
    # Create UDP socket.
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)

    # Bind to LFS.
    sock.bind(('127.0.0.1', 30000))

    while True:
        # Receive data.
        data = sock.recv(256)

        if not data:
            break # Lost connection
      
        # Unpack the data.
        outgauge_pack = struct.unpack('I3sxH2B7f2I3f15sx15sx', data)
        time = outgauge_pack[0]
        car = outgauge_pack[1]
        flags = outgauge_pack[2]
        gear = outgauge_pack[3]
        speed = outgauge_pack[5]
        rpm = outgauge_pack[6]
        turbo = outgauge_pack[7]
        engtemp = outgauge_pack[8]
        fuel = outgauge_pack[9]
        oilpressure = outgauge_pack[10]
        oiltemp = outgauge_pack[11]
        dashlights = outgauge_pack[12]
        showlights = outgauge_pack[13]
        throttle = outgauge_pack[14]
        brake = outgauge_pack[15]
        clutch = outgauge_pack[16]
        display1 = outgauge_pack[17]
        display2 = outgauge_pack[18]

    # Release the socket.
    sock.close()

def formatter(msg, max_msg, max_arduino):
    return msg * max_arduino / max_msg

def run():
    # Connecto to Arduino Serial
    arduino = connect_serial()

    # Create UDP socket.
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    # Bind to LFS.
    sock.bind(('127.0.0.1', 8001))
    while True:
    # Receive data.
        data = sock.recv(256)
        if not data:
            break # Lost connection  
        # Unpack the data.
        outgauge_pack = struct.unpack('I3sxH2B7f2I3f15sx15sx', data)

        rpm = outgauge_pack[6]

        print(rpm)
        value = write_read(">||" + str(formatter(rpm, 10000, 180)) + "|||<", arduino)
        print(value)
        time.sleep(0.01)
    # Release the socket.
    sock.close()
    #i = 0
    #while i <= 180:
    #    value = write_read(">||" + str(i) + "|||<", arduino)
    #    print(value)
    #    i += 1

run()
