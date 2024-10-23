use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::HashMap;
use std::net::UdpSocket;

#[derive(Debug, Deserialize, Serialize)]
struct DeviceInfo {
    mac_addr: String,
    ip_addr: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Devices {
    devices: HashMap<String, DeviceInfo>,
}

fn get_devices() -> HashMap<String, DeviceInfo> {
    // Read in the yaml file
    let file = std::fs::File::open("devices.yaml").expect("Failed to open devices.yaml");

    // Parse the YAML content
    let devices: Devices = serde_yaml::from_reader(file).expect("Failed to parse YAML");

    devices.devices
}

fn main() -> Result<(), serde_yaml::Error> {
    let devices = get_devices();
    println!("Powering on these devices:");
    for (name, device) in devices {
        println!("{} - {} - {}", name, device.mac_addr, device.ip_addr);
        {
            println!(
                "Attempting to wake device: MAC={}, IP={}",
                device.mac_addr, device.ip_addr
            );
            match send_wol_packet(&device.mac_addr, &device.ip_addr) {
                Ok(_) => println!("WoL packet sent successfully"),
                Err(e) => eprintln!("Failed to send WoL packet: {}", e),
            }
        }
    }

    Ok(())
}

fn send_wol_packet(mac_addr: &str, ip_addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Parse MAC address
    let mac_bytes = mac_addr
        .split(':')
        .map(|x| u8::from_str_radix(x, 16))
        .collect::<Result<Vec<u8>, _>>()?;

    if mac_bytes.len() != 6 {
        return Err("Invalid MAC address".into());
    }

    // Create magic packet
    let mut packet = vec![0xFF; 6];
    packet.extend(mac_bytes.iter().cycle().take(16 * 6));

    // Send packet
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;
    socket.send_to(&packet, format!("{}:9", ip_addr))?;
    socket.send_to(&packet, format!("{}:7", ip_addr))?;

    Ok(())
}
