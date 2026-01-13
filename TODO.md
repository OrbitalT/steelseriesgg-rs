- https://docs.rs/hidapi/2.6.4/hidapi
- https://crates.io/crates/evdevil
- https://crates.io/crates/usbd-human-interface-device

Try to integrate these to see if that works:

```bash
cargo add nix lazy_static log
```

```bash
sudo groupdel uinput 2>/dev/null
sudo groupadd --system uinput
sudo usermod -aG input $USER
sudo usermod -aG uinput $USER
groups
sudo modprobe uinput
```

```bash
sudo tee /etc/udev/rules.d/99-input.rules >/dev/null <<EOF
KERNEL=="uinput", MODE="0660", GROUP="uinput", OPTIONS+="static_node=uinput"
EOF
```

```bash
sudo udevadm control --reload-rules && sudo udevadm trigger
ls -l /dev/uinput
```
