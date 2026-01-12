look into rusb and parsing with udev for the ID's

snippets:

```rust
use udev::{Enumerator, Context};

fn main() -> anyhow::Result<()> {
    let ctx = Context::new()?;
    let mut en = Enumerator::new(&ctx)?;
    en.match_subsystem("usb")?;

    for device in en.scan_devices()? {
        if let (Some(vid_s), Some(pid_s)) = (
            device.property_value("ID_VENDOR_ID"),
            device.property_value("ID_MODEL_ID"),
        ) {
            let vid = u16::from_str_radix(&vid_s.to_string_lossy(), 16)?;
            let pid = u16::from_str_radix(&pid_s.to_string_lossy(), 16)?;
            println!("{:04x}:{:04x}", vid, pid);
        }
    }
    Ok(())
}
```

rusb:

```rust
for device in rusb::devices()?.iter() {
    let desc = device.device_descriptor()?;
    println!("{:04x}:{:04x}", desc.vendor_id(), desc.product_id());
}
```

parsing only works for linux with udev, use rusb for windows
