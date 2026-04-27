# Music Reactive Keyboard
![Hackatime](https://hackatime-badge.hackclub.com/U081TBVQLCX/reactive-keyboard)
![License](https://img.shields.io/github/license/madavidcoder/reactive-keyboard)
![Created At](https://img.shields.io/github/created-at/madavidcoder/reactive-keyboard)
![Top Language](https://img.shields.io/github/languages/top/madavidcoder/reactive-keyboard)
![Commits](https://img.shields.io/github/commit-activity/t/madavidcoder/reactive-keyboard)

A custom firmware I built for my keyboard, which reacts to system audio, turning my desk into a light-show synced with whatever is playing on my computer!

### Check out the demo video on the [releases page](https://github.com/MadAvidCoder/reactive-keyboard/releases/latest)!

<img alt="image" src="https://github.com/user-attachments/assets/f9a52a44-5238-44ba-8631-6e12961b626f" />

## Features
- Runs it in a terminal window, and it will automatically connect to the custom keyboard firmware (no bloated GUIs!) 
- Captures whatever is playing on your laptop (*e.g. from Spotify*) via WASAPI loopback capture
- Real-time rendering of audio-reactive effects on the keyboard's RGB LEDs
- Supports simultaneous rendering and keyboard input *(so you can still type while the effects are running)*
- Simple serial communication protocol between host and keyboard, allowing for easily making new effects

## How it Works
**Host-Side (Audio Capture and Process)**
- In one thread, audio is captured via the WASAPI API and sent into a ringbuffer
- In a second thread, the audio is processed
- An FFT gets frequency data for mapping to LED colours
- Aubio extracts beat and onset information
- The processed audio data is used to generate LED effects
- The generated LED data is streamed to the keyboard, via USB serial *(not super robust, so frames can be dropped/corrupted)*

**Client-Side (Keyboard Firmware)**
- The keyboard constantly listens for incoming serial data
- When it receives LED data, it buffers it until it constructs a full frame
- If the frame was uncorrupted, it renders the LED data on the keyboard's RGB LEDs
- Simultaneously, it scans the keyboard matrix, and reports HID events back to the host, allowing for typing while effects run *(slight latency can occur)*

## Installation
> [!IMPORTANT]
> Currently, the software uses Windows-specific APIs for audio capture, so it can only be used on Windows. Support for other platforms may be added in the future.
### Client (Keyboard Firmware)
1. Download [CircuitPython](https://circuitpython.org/downloads) and follow the instructions to install it on your keyboard's microcontroller.
2. Download `code.py` from the [releases page](https://github.com/MadAvidCoder/reactive-keyboard/releases/latest) and place it in the root directory of the CircuitPython drive.
3. Install the required libraries for CircuitPython:
   - `adafruit_hid`
   - `neopixel`
   - `adafruit_pixelbuf`
   - `adafruit_blinka`
   
   You can use the [CircuitPython Library Bundle](https://circuitpython.org/libraries) to find and install these libraries.
4. Connect the keyboard to your computer and ensure that it is recognized as a USB device.

### Host (Audio Capture)
1. Download the executable for the host application from the [releases page](https://github.com/MadAvidCoder/reactive-keyboard/releases/latest).
2. Connect your keyboard to your computer and ensure it is recognized as a USB device.
3. Run the executable, and it should automatically detect the connected keyboard and start capturing audio from your system.
4. Start playing some music (eg. from Spotify) and the keyboard should automatically react!

## Usage
- Connect the keyboard to your computer and run the host application as described in the installation instructions.
- Play music from any source on your computer, and the keyboard's RGB LEDs will react to the audio in real-time.
- To stop the application, simply close the terminal window or press `Ctrl + C` in the terminal.

## Troubleshooting
- If the keyboard is not reacting to audio, ensure that it is properly connected and recognized by your computer, and confirm the COM port being used.
- Sometimes the COM port can change when you reconnect the keyboard, so make sure the host application is using the correct COM port.
- If the host application is not capturing audio, confirm you have the necessary audio drivers installed.
- If you encounter any issues, you can report it via [GitHub Issues](https://github.com/MadAvidCoder/reactive-keyboard/issues).

## Tech Stack
- **Client (Keyboard Firmware)**: CircuitPython, Adafruit libraries
- **Host (Audio Capture)**: Rust, WASAPI, Aubio, Serial Communication

## License
Reactive Keyboard is licensed under the [MIT License](LICENSE).

You are free to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of this software. You must include the copyright and license notice.

**There is no warranty.** Reactive Keyboard is provided “as is.”. Use and/or modify at your own risk.
