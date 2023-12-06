# miractl

_miractl_ is a utility to manage Boox Mira and Mira Pro e-ink displays.

## Installation

### Generic

Currently there are no pre-built binaries available. You’ll need a [Rust
toolchain](https://rustup.rs/) installed, then from the repository root you can
run:

```sh
$ cargo build --release
```

Then just copy `target/release/miractl` to somewhere in your path.  On Linux you
will also need appropriate udev rules; see below.

### Nix

_miractl_ is available as a Nix flake.  You can install it on NixOS with
something like the follwing in `/etc/nixos/configuration.nix`:

``` nix
let
  miractl = (builtins.getFlake "github:clarkema/miractl").defaultPackage.${builtins.currentSystem};
in
{
  ...
  environment.systemPackages = [
    ...
    miractl
    ...
  ];

  services.udev.packages = [ miractl ];
  ...
}
```

If you want to install it as a user via `home-manager`, or using `nixpkgs` on a
non-NixOS system, you’ll have to take care of appropriate udev rules yourself.
See `dist/nix/10-miractl.rules` for the rules used by the flake, or below for
generic rules for other systems.

## Usage

Refresh the display:

```bash
$ miractl refresh
```

Set display parameters:

```bash
$ miractl set --warm 25 --cold 15
$ miractl set --filter 10:96 --contrast 5
```

Select a display mode:

```bash
$ miractl mode read
$ miractl mode speed
```

Detect whether a display is connected:

```bash
$ miractl detect
$ echo $?
0 # or 1 if no display
```

The `detect` subcommand is intended for use in scripts that need to change
their behaviour depending on whether there is an e-ink display attached or not.
It just exits successfully if it could find and connect to a display.

### Parameters

<dl>
  <dt>warm</td>
  <dd>
    set the warm frontlight, from 0 to 254
  </dd>

  <dt>cold</td>
  <dd>
    set the cold frontlight, from 0 to 254
  </dd>

  <dt>speed</td>
  <dd>
    set the refresh speed, from 1 to 7
  </dd>

  <dt>refresh-mode</td>
  <dd>
    set the refresh mode
  </dd>

  <dt>contrast</td>
  <dd>
    set the contrast, from 0 to 15
  </dd>

  <dt>filter</td>
  <dd>
    set the white / black filter

Both must be set at the same time, in the format `WHITE:BLACK` where `WHITE`
and `BLACK` are integers between 0 and 127.
  </dd>
</dl>

## Colour filter and contrast

The Mira displays both support a contrast setting and have an adjustable colour
filter.  The interaction can be a little confusing.

The second slider down on the screen UI and the setting labelled “dark color
enhancement” in the official app both adjust the _contrast_ of the display,
between 0—15.

The third slider down on the screen UI and the setting labelled “light color
filter” in the official app adjust the white filter.

The white and black filters work in a manner analogous to setting the white and
black points of a photograph.  Anything lighter than the white filter point is
considered to be white; anything darker than the black filter point is
considered to just be black.  This leads to losing some detail in the lightest
and darkest parts of the image, but it can be a significant benefit in general
web browsing use.  Webpages often have backgrounds that are not 100% white;
without a white filter they would appear dithered on the display—the effect is
unnecessarily distracting.  The white filter allows you to get rid of the
dithering and have the background display as though it were pure white,
resulting in a higher-contrast image that works better on the e-ink display.

The black filter works in the same way, and can be useful with dark text that
isn’t actually black.

It’s not possible to set the black filter directly from the display UI or the
Boox app at the time of writing; instead, it forms part of the presets that
make up the official default display modes.

## Display modes

Display modes are essentially built-in pre-defined combinations of the
lower-level parameters.  There are five: `speed`, `text`, `image`, `video`, and
`read`.

## Caveats and limitations

I only have one Boox Mira for testing.  I believe this should work with the Pro
as well, but if you have multiple Mira screens your mileage may vary.  PRs /
testers welcome.

The auto dithering setting is not yet supported.

## udev rules

1. Create `/etc/udev/rules.d/58-hid.rules`

2. Copy the following rules into the file to support `hidraw` and `libusb`.

```
SUBSYSTEM=="input", GROUP="input", MODE="0666"
SUBSYSTEM=="usb", ATTRS{idVendor}=="0416", ATTRS{idProduct}=="5020", MODE:="666", GROUP="plugdev"
KERNEL=="hidraw*", ATTRS{idVendor}=="0416", ATTRS{idProduct}=="5020", MODE="0666", GROUP="plugdev"
```

3. Reload udev rules

```bash
udevadm control --reload-rules && udevadm trigger
```

## Credits

The core of the USB interface was based on knowledge from
https://github.com/ipodnerd3019/mira-js.

## Alternatives

 - [mira-js](https://github.com/ipodnerd3019/mira-js)

   The original implementation in JavaScript.


 - [miractl](https://git.sr.ht/~elithper/miractl)

   A port of mira-js implemented as a standalone Python script.
