# Kitchen Lights
Ikea Utrusta LED Worktop lighting does not support Philips Hue, so I've built a raspberry pi binary that press the button to control the lights. This program reads the brightness of a bulb and then uses gpio pins to a transistor to click the required number of times for the lights to be in the correct state.

## Installation
1. bash install-openssl-cross.bash
2. bash release\_build.bash
3. edit config.ron
4. deploy to pi
