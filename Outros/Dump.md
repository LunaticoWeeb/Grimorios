# Dump

Anotações e textos desconexos em tema. Eles serão incorporados em outros textos ou serão descartados.

## Chat IA

### Bluetooth on NixOS

To connect a Bluetooth device on NixOS, you can follow these steps:
  
Enable Bluetooth support: Amend your system configuration as follows:

```nix
/etc/nixos/configuration.nix {
  hardware.bluetooth.enable = true; # enables support for Bluetooth hardware
  hardware.bluetooth.powerOnBoot = true; # powers up the default Bluetooth controller on boot
}
```

Then, evaluate your changes by running nixos-rebuild switch as root.

Pairing Bluetooth devices: If your desktop environment provides a Bluetooth management GUI, you can use it to pair devices. If not, you can enable the blueman service, which provides blueman-applet and blueman-manager with the snippet below:

```nix
services.blueman.enable = true;
```

Alternatively, Bluetooth devices can be paired from the command line using bluetoothctl:

```bash
$ bluetoothctl
[bluetooth] # power on
[bluetooth] # agent on
[bluetooth] # default-agent
[bluetooth] # scan on
...put device in pairing mode and wait for the hex-address to appear here...
[bluetooth] # pair [hex-address]
[bluetooth] # connect [hex-address]
```

Bluetooth devices automatically connect with bluetoothctl as well:

```bash
$ bluetoothctl
[bluetooth] # trust [hex-address]
```

Using Bluetooth headsets with PulseAudio: To allow Bluetooth audio devices to be used with PulseAudio, amend /etc/nixos/configuration.nix as follows:

```nix
{
  hardware.pulseaudio.enable = true;
  hardware.bluetooth.enable = true;
}
```

Then, evaluate your changes by running nixos-rebuild switch as root. You will need to restart PulseAudio; try systemctl --user daemon-reload; systemctl --user restart pulseaudio. You can verify that PulseAudio has loaded the Bluetooth module by running pactl list | grep -i 'Name.*module.*blue'; Bluetooth modules should be present in the list.
Please note that heavier desktop environments will usually provide a Bluetooth management GUI which you can use to pair devices. Almost any desktop environment will have some graphical interface for connecting to Bluetooth devices.

