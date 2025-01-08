# n16-shell

### base16 themed layershell components for niri

![image](https://cloud-mxs6qgq8r-hack-club-bot.vercel.app/1screenshot_from_2025-01-07_15-42-31.png)

n16-shell is a collection of shell components for my nixos rice, focusing on the niri window manager and using the wayland layer_shell protocol to appear as a non-tiling window.

It uses [base16 theming](https://github.com/chriskempson/base16) for integration with [stylix](https://stylix.danth.me/) and my nixos configuration as a whole.

n16-shell is written in rust for better compatibility with niri and because rust is one of the most accesible systems programming languages currently. _(C is cringe and you can't change my mind)_

## Components

- ### App launcher

  - Loads desktop entries according to the XDG desktop standard
  - Loads icon themes according to the XDG icon standard (theme selection planned, current behavior defaults to `hicolor`)
  - Search for applications by name (category search planned)
  - **(Planned)** Customizable application catagories (messaging, games, etc)
  - **(Planned)** Non-application actions (power management, calculator, nix-shell)

- ### (Future) Colapsable bar
  - **(Future)** Hidden during normal use
  - **(Future)** Slides up from bottom on keybind activation
  - **(Future)** Shows niri workspaces and open applications

##

- **iced:** ui framework (https://crates.io/crates/iced)
- **iced_layershell:** iced integration with the wayland compositor (https://crates.io/crates/iced_layershell)
- **freedesktop-desktop-entry:** find and parse desktop entries (https://crates.io/crates/freedesktop-desktop-entry)
- **tini:** parser for XDG standard ini files, including icon themes (https://crates.io/crates/tini)
