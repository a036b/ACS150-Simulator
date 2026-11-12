# ACS150 Simulator (Rust)

A Windows desktop educational simulator prototype written in Rust using
`eframe`/`egui`.

## Build locally on Windows
Install the stable Rust toolchain from https://rustup.rs, then run:

```powershell
cargo build --release
```

The executable will be at:

```text
target\release\acs150-simulator.exe
```

## Build without installing Rust on your PC
Push this repository to GitHub, open **Actions**, and run
**Build ACS150 Simulator for Windows 10 x64**. Download the resulting
`ACS150_Simulator_Windows10_x64` artifact. Rust is used only on the
temporary cloud build runner.

## Included
- Simplified LCP screen and keypad navigation
- Start / Stop / Reset
- Frequency reference and output ramp simulation
- Forward / reverse selection
- Initial parameter list
- Demonstration fault buttons

## Safety and scope
This is a software-only educational prototype. It does not communicate with
or control a physical ABB ACS150. Parameter values and behaviors are
illustrative until verified against the exact drive manual and firmware.
