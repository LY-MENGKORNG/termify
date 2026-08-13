# Termify

A modern, keyboard-first Spotify client for the terminal.

```
    ████████╗███████╗██████╗ ███╗   ███╗██╗███████╗██╗   ██╗
    ╚══██╔══╝██╔════╝██╔══██╗████╗ ████║██║██╔════╝╚██╗ ██╔╝
       ██║   █████╗  ██████╔╝██╔████╔██║██║█████╗   ╚████╔╝
       ██║   ██╔══╝  ██╔══██╗██║╚██╔╝██║██║██╔══╝    ╚██╔╝
       ██║   ███████╗██║  ██║██║ ╚═╝ ██║██║██║        ██║
       ╚═╝   ╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝╚═╝╚═╝        ╚═╝
```

termify **plays audio itself**. It embeds
[librespot](https://github.com/librespot-org/librespot) and registers as a Spotify
Connect device, so there is no need to leave the desktop app open. It is also a
remote control: press `d` and send playback to your phone, a speaker, or anything
else on your account.

```
┌─────────┬────────────────────────────────────────────────┐
│ ▶  Now Playing                                           │
│ ⌂  Home        Whenever You Need Somebody · 1987         │
│ ⌕  Search                                                │
│ ♪  Library     Never Gonna Give You Up                   │
│ ≡  Queue       Rick Astley                               │
│                                                          │
│                                                          │
│                0:31 elapsed · 3:02 left                  │
│                playing on Studio (computer)              │
│ rick                                                     │
├──────────────────────────────────────────────────────────┤
│ space play/pause · n next · d devices · ? help           │
│                                                          │
│ ▶  Never Gonna Give You Up — Rick Astley           70%   │
│ 0:31 ━━━━━━━━━━━━╸──────────────────────────────── 3:33  │
└──────────────────────────────────────────────────────────┘
```

## Requirements

- Rust 1.86 or newer
- **Spotify Premium.** Neither the Web API nor librespot will play on a free account.
- A Spotify app registered in the [developer dashboard](https://developer.spotify.com/dashboard)

## Getting started

```sh
cargo run
```


