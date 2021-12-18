# NetPort

![license](https://img.shields.io/badge/license-Apache%202-green)
![win64](https://img.shields.io/badge/windows-release64-informational)
![macos64](https://img.shields.io/badge/macOS-release64-magenta)

*A GUI address port checker written in Rust*

<p align="center"><img src="https://i.ibb.co/SxdTX5j/netport.png"></p>

## Install 📦

*Windows and macOS binary files can be found in [Release](https://github.com/shijimasoft/netport/releases) section*

<br>

<u> **Compile from Source Code** </u>

No external dependencies are required on Windows and macOS.

on Linux you should install:

**Debian**
`libx11-dev libxext-dev libxft-dev libxinerama-dev libxcursor-dev libxrender-dev libxfixes-dev libpango1.0-dev libgl1-mesa-dev libglu1-mesa-dev`

**Arch**
`libx11 libxext libxft libxinerama libxcursor libxrender libxfixes pango cairo libgl mesa --needed`

<br>

*Build binary with Rust compiler:*

`cargo build --release`
