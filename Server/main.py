from posix import read
import qrcode
import os
import sys
import time
import data_spliter
from qrcode.main import QRCode

def gt(a,b):
    if a > b:
        return a
    if b > a:
        return b
    return a


def print_qr(data):
    qr = QRCode()
    qr.add_data(data)
    qr.print_ascii()

def multi_qr_maker(data):
    os.system('cls' if os.name == 'nt' else 'clear')
    bch = data_spliter.BigChunkus(data, 200)
    while True:
        for d in bch:
            os.system('cls' if os.name == 'nt' else 'clear')
            print_qr(d)
            print(f'{d.part}/{d.total}')
            time.sleep(gt(0.5 / len(bch), 0.05))



if __name__ == "__main__":
    if len(sys.argv) < 0:
        print("Provide file to transfer")
        exit(1)
    f = open(sys.argv[1], "r")
    try:
        multi_qr_maker(f.read())
    except KeyboardInterrupt:
        print("\rBye")
