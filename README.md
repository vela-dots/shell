<h1 align=center>vela-shell</h1>

<div align=center>

![GitHub last commit](https://img.shields.io/github/last-commit/vela-dots/shell?style=for-the-badge&labelColor=101418&color=9ccbfb)
![GitHub Repo stars](https://img.shields.io/github/stars/vela-dots/shell?style=for-the-badge&labelColor=101418&color=b9c8da)
![GitHub repo size](https://img.shields.io/github/repo-size/vela-dots/shell?style=for-the-badge&labelColor=101418&color=d3bfe6)
[![Ko-Fi donate](https://img.shields.io/badge/donate-kofi?style=for-the-badge&logo=ko-fi&logoColor=ffffff&label=ko-fi&labelColor=101418&color=f16061&link=https%3A%2F%2Fko-fi.com%2Fsoramane)](https://ko-fi.com/soramane)

</div>

https://github.com/user-attachments/assets/0840f496-575c-4ca6-83a8-87bb01a85c5f

## Components

-   Widgets: [`Quickshell`](https://quickshell.outfoxxed.me)
-   Window manager: [`Hyprland`](https://hyprland.org)
-   Dots: [`vela`](https://github.com/vela-dots)

## Installation

> [!NOTE]
> This repo is for the desktop shell of the vela dots. If you want installation instructions
> for the entire dots, head to [the main repo](https://github.com/vela-dots/vela) instead.

### Arch linux

> [!NOTE]
> If you want to make your own changes/tweaks to the shell do NOT edit the files installed by the AUR
> package. Instead, follow the instructions in the [manual installation section](#manual-installation).

The shell is available from the AUR as `vela-shell`. You can install it with an AUR helper
like [`yay`](https://github.com/Jguer/yay) or manually downloading the PKGBUILD and running `makepkg -si`.

A package following the latest commit also exists as `vela-shell-git`. This is bleeding edge
and likely to be unstable/have bugs. Regular users are recommended to use the stable package
(`vela-shell`).

 

### Manual installation

Dependencies:

-   [`vela-cli`](https://github.com/vela-dots/cli)
-   [`quickshell-git`](https://quickshell.outfoxxed.me) - this has to be the git version, not the latest tagged version
-   [`ddcutil`](https://github.com/rockowitz/ddcutil)
-   [`brightnessctl`](https://github.com/Hummer12007/brightnessctl)
-   [`app2unit`](https://github.com/Vladimir-csp/app2unit)
-   [`cava`](https://github.com/karlstav/cava)
-   [`networkmanager`](https://networkmanager.dev)
-   [`lm-sensors`](https://github.com/lm-sensors/lm-sensors)
-   [`fish`](https://github.com/fish-shell/fish-shell)
-   [`aubio`](https://github.com/aubio/aubio)
-   [`libpipewire`](https://pipewire.org)
-   `glibc`
-   `qt6-declarative`
-   `gcc-libs`
-   [`material-symbols`](https://fonts.google.com/icons)
-   [`caskaydia-cove-nerd`](https://www.nerdfonts.com/font-downloads)
-   [`swappy`](https://github.com/jtheoof/swappy)
-   [`libqalculate`](https://github.com/Qalculate/libqalculate)
-   [`bash`](https://www.gnu.org/software/bash)
-   `qt6-base`
-   `qt6-declarative`

Build dependencies:

-   [`cmake`](https://cmake.org)
-   [`ninja`](https://github.com/ninja-build/ninja)

To install the shell manually, install all dependencies and clone this repo to `$XDG_CONFIG_HOME/quickshell/vela`.
Then simply build and install using `cmake`.

```sh
cd $XDG_CONFIG_HOME/quickshell
git clone https://github.com/vela-dots/shell.git vela

cd vela
cmake -B build -G Ninja -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/
cmake --build build
sudo cmake --install build
```

> [!TIP]
> You can customize the installation location via the `cmake` flags `INSTALL_LIBDIR`, `INSTALL_QMLDIR` and
> `INSTALL_QSCONFDIR` for the libraries (the beat detector), QML plugin and Quickshell config directories
> respectively. If changing the library directory, remember to set the `VELA_LIB_DIR` environment
> variable to the custom directory when launching the shell.
>
> e.g. installing to `~/.config/quickshell/vela` for easy local changes:
>
> ```sh
> mkdir -p ~/.config/quickshell/vela
> cmake -B build -G Ninja -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/ -DINSTALL_QSCONFDIR=~/.config/quickshell/vela
> cmake --build build
> sudo cmake --install build
> sudo chown -R $USER ~/.config/quickshell/vela
> ```

## Usage

The shell can be started via the `vela shell -d` command or `qs -c vela`.
If the entire vela dots are installed, the shell will be autostarted on login
via an `exec-once` in the hyprland config.

### Shortcuts/IPC

All keybinds are accessible via Hyprland [global shortcuts](https://wiki.hyprland.org/Configuring/Binds/#dbus-global-shortcuts).
If using the entire vela dots, the keybinds are already configured for you.
Otherwise, [this file](https://github.com/vela-dots/vela/blob/main/hypr/hyprland/keybinds.conf#L1-L39)
contains an example on how to use global shortcuts.

All IPC commands can be accessed via `vela shell ...`. For example

```sh
vela shell mpris getActive trackTitle
```

The list of IPC commands can be shown via `vela shell -s`:

```
$ vela shell -s
target drawers
  function toggle(drawer: string): void
  function list(): string
target notifs
  function clear(): void
target lock
  function lock(): void
  function unlock(): void
  function isLocked(): bool
target mpris
  function playPause(): void
  function getActive(prop: string): string
  function next(): void
  function stop(): void
  function play(): void
  function list(): string
  function pause(): void
  function previous(): void
target picker
  function openFreeze(): void
  function open(): void
target wallpaper
  function set(path: string): void
  function get(): string
  function list(): string
```

### PFP/Wallpapers

The profile picture for the dashboard is read from the file `~/.face`, so to set
it you can copy your image to there or set it via the dashboard.

The wallpapers for the wallpaper switcher are read from `~/Pictures/Wallpapers`
by default. To change it, change the wallpapers path in `~/.config/vela/shell.json`.

To set the wallpaper, you can use the command `vela wallpaper`. Use `vela wallpaper -h` for more info about
the command.

## Updating

If installed via the AUR package, simply update your system (e.g. using `yay`).

If installed manually, you can update by running `git pull` in `$XDG_CONFIG_HOME/quickshell/vela`.

```sh
cd $XDG_CONFIG_HOME/quickshell/vela
git pull
```

## Configuring

All configuration options should be put in `~/.config/vela/shell.json`. This file is _not_ created by
default, you must create it manually.

### Example configuration

> [!NOTE]
> The example configuration only includes recommended configuration options. For more advanced customization
> such as modifying the size of individual items or changing constants in the code, there are some other
> options which can be found in the source files in the `config` directory.

<details><summary>Example</summary>

```json
{
    "appearance": {
        "anim": {
            "durations": {
                "scale": 1
            }
        },
        "font": {
            "family": {
                "material": "Material Symbols Rounded",
                "mono": "CaskaydiaCove NF",
                "sans": "Rubik"
            },
            "size": {
                "scale": 1
            }
        },
        "padding": {
            "scale": 1
        },
        "rounding": {
            "scale": 1
        },
        "spacing": {
            "scale": 1
        },
        "transparency": {
            "enabled": false,
            "base": 0.85,
            "layers": 0.4
        }
    },
    "general": {
        "apps": {
            "terminal": ["kitty"],
            "audio": ["pavucontrol"]
        }
    },
    "background": {
        "desktopClock": {
            "enabled": false
        },
        "enabled": true,
        "visualiser": {
            "enabled": false,
            "autoHide": true,
            "rounding": 1,
            "spacing": 1
        }
    },
    "bar": {
        "clock": {
            "showIcon": true
        },
        "dragThreshold": 20,
        "entries": [
            {
                "id": "logo",
                "enabled": true
            },
            {
                "id": "workspaces",
                "enabled": true
            },
            {
                "id": "spacer",
                "enabled": true
            },
            {
                "id": "activeWindow",
                "enabled": true
            },
            {
                "id": "spacer",
                "enabled": true
            },
            {
                "id": "tray",
                "enabled": true
            },
            {
                "id": "clock",
                "enabled": true
            },
            {
                "id": "statusIcons",
                "enabled": true
            },
            {
                "id": "power",
                "enabled": true
            },
            {
                "id": "idleInhibitor",
                "enabled": false
            }
        ],
        "persistent": true,
        "scrollActions": {
            "brightness": true,
            "workspaces": true,
            "volume": true
        },
        "showOnHover": true,
        "status": {
            "showAudio": false,
            "showBattery": true,
            "showBluetooth": true,
            "showKbLayout": false,
            "showMicrophone": false,
            "showNetwork": true,
            "showLockStatus": true
        },
    "tray": {
            "background": false,
            "iconSubs": [],
            "recolor": false
        },
        "workspaces": {
            "activeIndicator": true,
            "activeLabel": "󰮯",
            "activeTrail": false,
            "label": "  ",
            "occupiedBg": false,
            "occupiedLabel": "󰮯",
            "perMonitorWorkspaces": true,
            "showWindows": true,
            "shown": 5
        }
    },
    "border": {
        "rounding": 25,
        "thickness": 10
    },
    "dashboard": {
        "enabled": true,
        "dragThreshold": 50,
        "mediaUpdateInterval": 500,
        "showOnHover": true
    },
    "launcher": {
        "actionPrefix": ">",
        "actions": [
            {
                "name": "Calculator",
                "icon": "calculate",
                "description": "Do simple math equations (powered by Qalc)",
                "command": ["autocomplete", "calc"],
                "enabled": true,
                "dangerous": false
            },
            {
                "name": "Scheme",
                "icon": "palette",
                "description": "Change the current color scheme",
                "command": ["autocomplete", "scheme"],
                "enabled": true,
                "dangerous": false
            },
            {
                "name": "Wallpaper",
                "icon": "image",
                "description": "Change the current wallpaper",
                "command": ["autocomplete", "wallpaper"],
                "enabled": true,
                "dangerous": false
            },
            {
                "name": "Variant",
                "icon": "colors",
                "description": "Change the current scheme variant",
                "command": ["autocomplete", "variant"],
                "enabled": true,
                "dangerous": false
            },
            {
                "name": "Transparency",
                "icon": "opacity",
                "description": "Change shell transparency",
                "command": ["autocomplete", "transparency"],
                "enabled": false,
                "dangerous": false
            },
            {
                "name": "Random",
                "icon": "casino",
                "description": "Switch to a random wallpaper",
                "command": ["vela", "wallpaper", "-r"],
                "enabled": true,
                "dangerous": false
            },
            {
                "name": "Light",
                "icon": "light_mode",
                "description": "Change the scheme to light mode",
                "command": ["setMode", "light"],
                "enabled": true,
                "dangerous": false
            },
            {
                "name": "Dark",
                "icon": "dark_mode",
                "description": "Change the scheme to dark mode",
                "command": ["setMode", "dark"],
                "enabled": true,
                "dangerous": false
            },
            {
                "name": "Shutdown",
                "icon": "power_settings_new",
                "description": "Shutdown the system",
                "command": ["systemctl", "poweroff"],
                "enabled": true,
                "dangerous": true
            },
            {
                "name": "Reboot",
                "icon": "cached",
                "description": "Reboot the system",
                "command": ["systemctl", "reboot"],
                "enabled": true,
                "dangerous": true
            },
            {
                "name": "Logout",
                "icon": "exit_to_app",
                "description": "Log out of the current session",
                "command": ["loginctl", "terminate-user", ""],
                "enabled": true,
                "dangerous": true
            },
            {
                "name": "Lock",
                "icon": "lock",
                "description": "Lock the current session",
                "command": ["loginctl", "lock-session"],
                "enabled": true,
                "dangerous": false
            },
            {
                "name": "Sleep",
                "icon": "bedtime",
                "description": "Suspend then hibernate",
                "command": ["systemctl", "suspend-then-hibernate"],
                "enabled": true,
                "dangerous": false
            }
        ],
        "dragThreshold": 50,
        "vimKeybinds": false,
        "enableDangerousActions": false,
        "maxShown": 8,
        "maxWallpapers": 9,
        "specialPrefix": "@",
        "useFuzzy": {
            "apps": false,
            "actions": false,
            "schemes": false,
            "variants": false,
            "wallpapers": false
        },
        "showOnHover": false
    },
    "lock": {
        "recolorLogo": false
    },
    "notifs": {
        "actionOnClick": false,
        "clearThreshold": 0.3,
        "defaultExpireTimeout": 5000,
        "expandThreshold": 20,
        "expire": false
    },
    "osd": {
        "enabled": true,
        "enableBrightness": true,
        "enableMicrophone": false,
        "hideDelay": 2000
    },
    "paths": {
        "mediaGif": "root:/assets/bongocat.gif",
        "sessionGif": "root:/assets/kurukuru.gif",
        "wallpaperDir": "~/Pictures/Wallpapers"
    },
    "services": {
        "audioIncrement": 0.1,
        "defaultPlayer": "Spotify",
        "gpuType": "",
        "playerAliases": [{ "from": "com.github.th_ch.youtube_music", "to": "YT Music" }],
        "weatherLocation": "",
        "useFahrenheit": false,
        "useTwelveHourClock": false,
        "smartScheme": true,
        "visualiserBars": 45
    },
    "session": {
        "dragThreshold": 30,
        "vimKeybinds": false,
        "commands": {
            "logout": ["loginctl", "terminate-user", ""],
            "shutdown": ["systemctl", "poweroff"],
            "hibernate": ["systemctl", "hibernate"],
            "reboot": ["systemctl", "reboot"]
        }
    }
}
```

</details>

 

## FAQ

### My screen is flickering, help pls!

Try disabling VRR in the hyprland config. You can do this by adding the following to `~/.config/vela/hypr-user.conf`:

```conf
misc {
    vrr = 0
}
```

### I want to make my own changes to the hyprland config!

You can add your custom hyprland configs to `~/.config/vela/hypr-user.conf`.

### I want to make my own changes to other stuff!

See the [manual installation](https://github.com/vela-dots/shell?tab=readme-ov-file#manual-installation) section
for the corresponding repo.

### I want to disable XXX feature!

Please read the [configuring](https://github.com/vela-dots/shell?tab=readme-ov-file#configuring) section in the readme.
If there is no corresponding option, make feature request.

### How do I make my color scheme change with my wallpaper?

Set a wallpaper via the launcher or `vela wallpaper` and set the scheme to the dynamic scheme via the launcher
or `vela scheme set`. e.g.

```sh
vela wallpaper -f <path/to/file>
vela scheme set -n dynamic
```

### My wallpapers aren't showing up in the launcher!

The launcher pulls wallpapers from `~/Pictures/Wallpapers` by default. You can change this in the config. Additionally,
the launcher only shows an odd number of wallpapers at one time. If you only have 2 wallpapers, consider getting more
(or just putting one).

## Credits

Thanks to the Hyprland discord community (especially the homies in #rice-discussion) for all the help and suggestions
for improving these dots!

A special thanks to [@outfoxxed](https://github.com/outfoxxed) for making Quickshell and the effort put into fixing issues
and implementing various feature requests.

Finally another thank you to all the configs I took inspiration from (only one for now):

-   [Axenide/Ax-Shell](https://github.com/)
