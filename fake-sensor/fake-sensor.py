#!/usr/bin/env python
import socket
import argparse
import ipaddress
from random import random
from math import sin, pi
from time import sleep
from datetime import datetime

def main():
    # Create the parser
    parser = argparse.ArgumentParser(description='This is a simple fake temperaturesensor')
    parser.add_argument('-s', '--host',type=str , help='Hostname or ip address of the host', default="127.0.0.1")
    parser.add_argument('-p', '--port', type=int, help='Port of the host', default=8886)
    parser.add_argument('-t', '--shoots', type=int, help='Numbers of time to shooting to the server. Set to 0 for infinite shoots', default=0)
    parser.add_argument('-r','--shoots-rate', type=int, help='Frequency of shooting at minute', default=120)
    parser.add_argument('-i','--sensor-id', type=int, help='Id of the fake sensors', default=1)
    parser.add_argument('--base-temperature', type=float, help='Base temperature of the server', default=25)
    parser.add_argument('--temperature-noise', type=float, help='Temperature noise', default=1.0)
    parser.add_argument('--temperature-maximium', type=float, help='Temperature maximium', default=50)
    parser.add_argument('--temperature-minimium', type=float, help='Temperature minimium', default=-10)
    parser.add_argument('--temperature-frequency', type=float, help='Frequency of the temperature', default=5)
    parser.add_argument('--temperature-amplitudine', type=float, help='Amplitudine of the temperature', default=3)

    # Execute the parser
    res = parser.parse_args()
    
    # Setup the socket
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)

    # Initialize the variables
    temperature = res.base_temperature
    shootN = 0

    # Repeat forever or to res.shoots
    while (res.shoots == 0) | (shootN < res.shoots):
        sendData(sock, res.host, res.port, res.sensor_id, temperature)
        # Calculate the new pseudo-real temperature
        temperature =clamp(
            res.base_temperature + 
            sin((shootN % res.shoots_rate)/res.shoots_rate*res.temperature_frequency*2*pi)*res.temperature_amplitudine +
            (random()-0.5)*2*res.temperature_noise, res.temperature_minimium, res.temperature_maximium)
        
        shootN = shootN+1
        
        # Wait a little time
        sleep(1/res.shoots_rate*60)


def sendData(socket, host, port, sensorID, temperature):
    now = datetime.now()
    msg = "inizio;\nsensore {};\ndata {}-{}-{};\nora {}-{}-{};\ntemperatura rilevata {};\nfine".format(
        sensorID,
        now.day, now.month, now.year,
        now.hour, now.minute, now.second,
        temperature
    ).encode("ascii");
    print(msg)
    socket.sendto(msg, (host, port))


def clamp(n, minn, maxn):
    return max(min(maxn, n), minn)


main()