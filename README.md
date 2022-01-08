# NetPort

![license](https://img.shields.io/badge/license-Apache%202-green)
![version](https://img.shields.io/badge/version-0.4-blueviolet)
![win64](https://img.shields.io/badge/windows-release64-informational)
![linux64](https://img.shields.io/badge/linux-release64-orange)
![macos64](https://img.shields.io/badge/macOS-release64-magenta)

*A GUI address port checker written in Rust*

<p align="center">
<img src="https://i.ibb.co/JFLwnrp/netport4.png">
</p>

## Install 📦

*Windows, Linux and macOS binary files can be found in [Releases](https://github.com/shijimasoft/netport/releases) section*

<br>

<u> **Compile from Source Code** </u>

`cmake`, `git` and `g++` are required on all OS.

No external dependencies are required on *Windows* and *macOS*,

on *Linux* you should install:

**Debian**
`libx11-dev libxext-dev libxft-dev libxinerama-dev libxcursor-dev libxrender-dev libxfixes-dev libpango1.0-dev libgl1-mesa-dev libglu1-mesa-dev`

**Arch**
`libx11 libxext libxft libxinerama libxcursor libxrender libxfixes pango cairo libgl mesa --needed`

<br>

*Build binary with Rust compiler:*

`cargo build --release`

## Terminal usage ⚡️

NetPort GUI can be easily started with:

`netport.exe` on Windows

`./netport` on Linux/macOS

It can also be invoked with parameters:

`netport [address]` <u>Address</u> should have this format: **IP**:**PORT**  (e.g. `127.0.0.1:80`)

You can also specify the connection timeout:

`netport [address] -t [time]` <u>Time</u> is expressed in seconds (Default is `8` seconds)

