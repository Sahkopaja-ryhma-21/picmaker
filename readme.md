# Picmaker

This application is made for our Electrical Engineering workshop (ELEC-A4010) course. It's an application, that converts SVG-files into commands understood by the Arduino-controller.

# Building

Releases have prebuild exe files and Arch packages. If you need to build this for yourself, you need cargo. Then go to the root of this project and run `cargo build --release`. The compiled binary is found under target. You can also run the project with `cargo run --release -- -f <FILENAME>`.

# Usage

## Files

You need an SVG file of paths for this to work. The paths can only be straight lines. No bezier curves! It is recommended for the svg canvas to be a square. You can use inkscape or a similar program to create the `.svg`-files. This git also includes some test pics.

## Arguments

The program has the following arguments:

`-f/--filename <FILENAME>` The svg file to be used.

`-b/--baud-rate <BAUD_RATE>` The baudrate for serial communication. Default 9600 is what the Arduino uses

`-h/--help` Help page

`-v/--version` Version

## Windows:

`picmaker.exe -f <FILENAME>`

## Linux

`./picmaker -f <FILENAME>`

or if you have the package installed and the binary in your path

`picmaker -f <FILENAME>`
