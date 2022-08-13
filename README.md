# tmux-resurrect-profiles

[![.github/workflows/release.yml](https://github.com/jakejdavis/tmux-resurrect-profiles/actions/workflows/release.yml/badge.svg)](https://github.com/jakejdavis/tmux-resurrect-profiles/actions/workflows/release.yml)

A CLI for switching between multiple [tmux-resurrect](https://github.com/tmux-plugins/tmux-resurrect) profiles by saving existing tmux-resurrect profiles and modifying the `~/.tmux/resurrect/last` symlink. 

## Installation 

Prebuilt binaries for macOS and Linux are available from the [releases](https://github.com/jakejdavis/tmux-resurrect-profiles/releases/) page

## Usage

```
USAGE:
    tmux-resurrect-profiles <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    create    Create a profile from existing tmux-resurrect saves
    help      Print this message or the help of the given subcommand(s)
    launch    Select a profile and launch tmux
    select    Select a profile
```
