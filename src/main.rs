use hidapi::HidApi;
use std::io::{self};
use steelseries_sonar::{Sonar, SonarError};

#[tokio::main]
async fn main() -> Result<(), SonarError> {
    // Create a new Sonar client
    let sonar = Sonar::new().await?;
    // Set master volume to 50%
    sonar.set_volume("master", 0.5, None).await?;
    // Mute the game channel
    sonar.mute_channel("game", true, None).await?;
    // Get current volume data
    let volume_data = sonar.get_volume_data().await?;
    println!("Current volume data: {}", volume_data);
    Ok(())
}

/// Sends data to the connected HID device with a specified timeout.
fn send_data_to_device(device: &mut hidapi::HidDevice, data: &[u8]) -> Result<(), String> {
    // Attempt to write the data to the HID device.
    match device.write(data) {
        Ok(_) => {
            println!("Data packet successfully sent: {:?}", data);
            Ok(())
        }
        Err(error) => {
            eprintln!("Error while sending data packet: {:?}", error);
            Err(format!("Failed to send data: {}", error))
        }
    }
}

/// Main entry point for the application, handling the device setup and data transmission.
fn main() -> io::Result<()> {
    // Raw data that needs to be sent to the device, starting with an extra 0x0 to ensure proper alignment.
    // Main colour rgb(157, 0, 255) = 0x9d, 0x0, 0xff
    // Alternative colour rgb(247, 75, 0) = 0xF7, 0x4B, 0x00
    let raw_data_to_send = [
        0x0, 0x21, 0xff, 0x9d, 0x0, 0xff, 0x9d, 0x0, 0xff, 0x9d, 0x0, 0xff, 0x9d, 0x0, 0xff, 0x9d,
        0x0, 0xff, 0x9d, 0x0, 0xff, 0x9d, 0x0, 0xff, 0x9d, 0x0, 0xff, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    ];

    // Initialize the HID API.
    let api = HidApi::new().expect("Failed to initialize HID API.");

    // Search for the connected device by Vendor ID, Product ID, and Interface Number.
    let selected_device_info = api
        .device_list()
        .find(|device| {
            device.vendor_id() == 0x1038
                && device.product_id() == 0x1622
                && device.interface_number() == 0x01
        })
        .expect(
            "No device found with Vendor ID 0x1038 and Product ID 0x1622 and Interface Number 0x01.",
        );

    println!(
        "Selected device: Vendor ID: 0x{:X}, Product ID: 0x{:X}",
        selected_device_info.vendor_id(),
        selected_device_info.product_id()
    );

    // Open the selected device using the HID API.
    let mut device_handle = selected_device_info
        .open_device(&api)
        .expect("Failed to open the device.");

    println!("Attempting to send data packets to the device...");

    // Send the raw data packet to the device.
    send_data_to_device(&mut device_handle, &raw_data_to_send)
        .expect("Failed to send data to the device.");

    // Confirm successful data transmission.
    println!("All data packets sent successfully.");

    Ok(())
}
