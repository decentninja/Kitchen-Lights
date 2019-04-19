# Kithen Lights
Ikea Utrusta LED Worktop lighting does not support Philips Hue, so I've built a Lego Mindstorm to click the button to control the lights. This program reads the brightness of a light called "Magic Light" from the first bridge on the network and then asks a Lego Mindstorm to click the required number of times. 

## Installation
1. Install libusb
2. Name a light "Magic Light"
3. Connect a Mindstorm EV3 programmable block with a medium motor in port A.
4. Run `cargo run` play with the lights, notice the motor reacting.
5. Build the mechanism to push the button and modify the program accordinly.
