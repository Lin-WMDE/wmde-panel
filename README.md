# WMDE Panel (WIP)

Fork of [cosmic-panel](https://github.com/pop-os/cosmic-panel). Ships the WMDE
panel/dock binary `wmde-panel` and its own config namespace `fun.wmde.Panel`.

### Building and Installing (Arch)

`makepkg -si`

The PKGBUILD builds via `just build-release` and installs the binary plus the
default schemas to `/usr/share/wmde/fun.wmde.Panel{,.Panel,.Dock}/v1/`.

### Configuring the panel / dock
See the provided configs for the panel and dock in `data/default_schema`.
The `fun.wmde.Panel` directory contains a key called entries, which is a list of profiles to be loaded.
Each profile then has its own directory, for example, `fun.wmde.Panel.Panel`.
You can make changes to the keys in this directory to alter the config.
After making changes to copies of the provided config in data `data/`, you may install each to `$HOME/.config/wmde/`
`find data/default_schema_copy -type f -exec install -Dm0644 {} {{$HOME/.config/wmde}}/{} \;`

### Usage
wmde-panel

### Installing Plugins and Applets
See the following for examples of applets and plugins which can be installed and used:
https://github.com/pop-os/cosmic-applets
