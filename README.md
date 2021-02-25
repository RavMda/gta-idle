![Version Badge](https://img.shields.io/badge/version-v1.0-brightgreen)
![License Badge](https://img.shields.io/badge/license-GPL--3.0-orange)

# GTA-IDLE

GTA-IDLE is a simple and dumb tool that will help you stay on the GTA Online server without being kicked for inactivity.

## How it works?

It simulates your mouse scroll when you're focused on GTA window.

Be aware that it does not affect the game memory in any way and therefore cannot be detected by anti-cheat.

**It won't do anything if the window is unfocused**

## Usage

Download any executable you prefer [here](https://github.com/RavMda/gta-idle/releases) and just run it.

![A Console](https://i.imgur.com/6YEG2k9.png)

## Build

#### Go
```
git clone https://github.com/RavMda/gta-idle.git
cd .\gta-idle\gta-idle-go\
go build -ldflags "-s -w" -o ./gta-idle-go.exe
```

## License
[GPL-3.0 License](https://choosealicense.com/licenses/gpl-3.0/)