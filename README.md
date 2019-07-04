# Rust-Visual-Keyboard
Create a virtual keyboard that based on Midi input will highlight which key(s) are being pressed. 


# Build
When running this program make sure to build it in release mode. Otherwise it will be VERY slow. This program uses SpriteBatch which runs very slowly in `debug` mode because it spends a lot of time on array bounds checking and un-optimized math; you need to build with optimizations enabled to really get the speed boost.

To run in release mode use the command below </br>
```cargo build --release```
