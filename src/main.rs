use gamesense::client::GameSenseClient;
use hidapi::HidApi;
use std::io;
use steelseries_sonar::Sonar;

/// Sends data to the connected HID device.
fn send_data_to_device(device: &mut hidapi::HidDevice, data: &[u8]) -> Result<(), String> {
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

/// Sets the keyboard lighting color via HID.
fn set_keyboard_lighting() -> io::Result<()> {
    // Raw data to send to the device, starting with 0x0 for proper alignment.
    // Main colour rgb(157, 0, 255) = 0x9d, 0x0, 0xff
    // Alternative colour rgb(247, 75, 0) = 0xF7, 0x4B, 0x00
    let raw_data_to_send = [
        0x0, 0x21, 0xff, 0x9d, 0x0, 0xff, 0x9d, 0x0, 0xff, 0x9d, 0x0, 0xff, 0x9d, 0x0, 0xff, 0x9d,
        0x0, 0xff, 0x9d, 0x0, 0xff, 0x9d, 0x0, 0xff, 0x9d, 0x0, 0xff, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    ];

    let api = HidApi::new().expect("Failed to initialize HID API.");

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

    let mut device_handle = selected_device_info
        .open_device(&api)
        .expect("Failed to open the device.");

    println!("Attempting to send data packets to the device...");

    send_data_to_device(&mut device_handle, &raw_data_to_send)
        .expect("Failed to send data to the device.");

    println!("All data packets sent successfully.");
    Ok(())
}

/// Example: Control SteelSeries Sonar audio settings.
/// Requires SteelSeries GG with Sonar to be running.
async fn control_sonar_audio() -> Result<(), steelseries_sonar::SonarError> {
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

/// Example: Initialize GameSense client for game event integration.
/// Requires SteelSeries GG Engine to be running.
fn init_gamesense_client() -> Result<GameSenseClient, Box<dyn std::error::Error>> {
    let client = GameSenseClient::new("ANTIBLOATLIGHT", "AntiBloatLight", "ScepticDope", None)?;
    println!("GameSense client initialized successfully.");
    Ok(client)
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // Primary functionality: Set keyboard lighting via HID
    set_keyboard_lighting()?;

    // Optional: Demonstrate Sonar integration (uncomment to use)
    // Note: Requires SteelSeries GG with Sonar to be running
    // if let Err(e) = control_sonar_audio().await {
    //     eprintln!("Sonar control failed: {}", e);
    // }

    // Optional: Demonstrate GameSense integration (uncomment to use)
    // Note: Requires SteelSeries GG Engine to be running
    // match init_gamesense_client() {
    //     Ok(_client) => println!("GameSense ready for game events"),
    //     Err(e) => eprintln!("GameSense init failed: {}", e),
    // }

    Ok(())
}
