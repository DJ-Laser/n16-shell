# n16-shell

### base16 themed layershell components for niri

[![built with garnix](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Fgarnix.io%2Fapi%2Fbadges%2Fdj-laser%2Fn16-shell)](https://garnix.io/repo/dj-laser/n16-shell)

![image](https://cloud-mxs6qgq8r-hack-club-bot.vercel.app/1screenshot_from_2025-01-07_15-42-31.png)

n16-shell is a collection of shell components for my nixos rice, focusing on the niri window manager and using the wayland layer_shell protocol to appear as a non-tiling window.

It uses [base16 theming](https://github.com/chriskempson/base16) for integration with [stylix](https://stylix.danth.me/) and my nixos configuration as a whole.

n16-shell is written in rust for better compatibility with niri and because rust is one of the most accesible systems programming languages currently. _(C is cringe and you can't change my mind)_

## Components

- ### App launcher

  - Loads desktop entries according to the XDG desktop standard
  - Loads icon themes according to the XDG icon standard (theme selection planned, current behavior defaults to `hicolor`)
  - Search for applications by name (category search planned)
  - Non-application actions (power management, calculator)
  - **(Planned)** Customizable application catagories (messaging, games, etc)

- ### (Partial) Colapsable bar

  - Barebones bar that shows the time
  - **(Planned)** Hidden during normal use
  - **(Planned)** Slides up from bottom on keybind activation
  - **(Future)** Shows niri workspaces and open applications

## Installation

Run `nix shell github:dj-laser/n16-shell` to try the program without permanantly installing.

Run `n16-daemon` to start the backend program.
This will launch the bar and listen for messages to control the bar and launcher

Run `n16` to control the backend by sending messages for example `n16 launcher open`.
Use `n16 <subcommand> help` to see the available options

For permanant instalation, add `github:dj-laser/n16-shell` as a flake input.

This flake exports a `packages.x86_64-linux.n16-shell`, or you can use the `overlays.default` to add `n16-shell` to `pkgs`.

Finally, configure `n16-daemon` to run on login.

This could be done a fwe ways, such as a systemd service or through the window manager.

Example using [`niri-flake`](https://https://github.com/sodiboo/niri-flake)

```nix
programs.niri = {
  settings.spawn-at-startup = [
    # Assuming n16-shell has been added to your packages
    {command = ["n16-daemon"];}

    # More explicit way to define it
    # {command = ["${pkgs.n16-shell}/bin/n16-daemon"];}
  ];
};
```

## Configuration

`n16-shell` currently looks for a config file at `$XDG_CONFIG_DIR/n16-shell/config.kdl` (kdl 1.0)

The only options at the moment are changing the theme colors. `base00` through `base0F`

```kdl
// In config.kdl

theme {
  base00 "#FFFFFF"
  base01 "#FFFFFF"
  base02 "#FFFFFF"

  // ...

  base0f "#FFFFFF"
}

```

## Technologies used

- **iced:** ui framework (https://crates.io/crates/iced)
- **iced_layershell:** iced integration with the wayland compositor (https://crates.io/crates/iced_layershell)
- **freedesktop-desktop-entry:** find and parse desktop entries (https://crates.io/crates/freedesktop-desktop-entry)
- **tini:** parser for XDG standard ini files, including icon themes (https://crates.io/crates/tini)
- **nix (❤️):** provides a reproducible dev environment and package build
