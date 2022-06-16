#!/usr/bin/env python3
from webusb import *
import argparse

parser = argparse.ArgumentParser(description='MCH2022 badge app sideloading tool')
parser.add_argument("name", help="AppFS filename")
parser.add_argument("application", help="Application binary")
parser.add_argument('--run', default=False, action='store_true')
args = parser.parse_args()

name = args.name
run = args.run

with open(args.application, "rb") as f:
    application = f.read()

dev = WebUSB()
print(f"Installing application \"{name}\" ({len(application)} bytes)...")

res = dev.appfsUpload(name, application)
if res:
    print("App installed")
else:
    print("Install failed")

if run:
    dev.appfsExecute(name)