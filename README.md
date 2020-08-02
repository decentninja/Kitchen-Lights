# Kitchen Lights
Ikea Utrusta LED Worktop lighting does not support Philips Hue, so I've built a raspberry pi binary that press the button to control the lights. This program reads the brightness of a bulb and then uses gpio pins to a transistor to click the required number of times for the lights to be in the correct state.

## Build
cargo install cross
cross build --target arm-unknown-linux-musleabi --release

## Upload
scp kitchen-lights.service config.ron user target/arm-unknown-linux-musleabi/release/kitchen-lights pi@floodgates.local:/home/pi/Kitchen-Lights

## Installation
cp kitchen-lights.service /etc/systemd/system/kitchen-lights.service
sudo systemctl enable kitchen-lights.service

## Debug
journalctl -u kitchen-lights
