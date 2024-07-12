// use btleplug::api::{CentralEvent, Manager as _, ScanFilter};
use btleplug::api::{Central, CentralEvent, Manager as _, Peripheral as _, ScanFilter, WriteType};
use btleplug::platform::Manager;
use btleplug::platform::Peripheral;
use futures::stream::StreamExt;
use std::error::Error;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
// const UUID: &str = "12345678-1234-5678-1234-567812345678"; // Example UUID, replace with your actual UUID
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let manager = Manager::new().await?;
    let adapters = manager.adapters().await?;
    let central = adapters
        .into_iter()
        .nth(0)
        .expect("No Bluetooth adapters found");

    central.start_scan(ScanFilter::default()).await?;
    println!("掃描中...");

    // let server_name = "MyRustBluetoothServer";

    // let mut events = central.events().await?;
    // while let Some(event) = events.next().await {
    //     match event {
    //         CentralEvent::DeviceDiscovered(id) => {
    //             let peripheral = central.peripheral(&id).await?;
    //             println!("發現裝置: {:?}", peripheral.properties().await?);

    //             // 連接並發送數據
    //             // if let Err(e) = connect_and_send_data(peripheral).await {
    //             //     eprintln!("數據傳輸失敗: {:?}", e);
    //             // }
    //         }
    //         _ => {}
    //     }
    // }

    time::sleep(Duration::from_secs(5)).await;

    // List the devices found
    let devices = central.peripherals().await?;
    if devices.is_empty() {
        println!("No devices found.");
    } else {
        for device in devices {
            let properties = device.properties().await?.unwrap();
            let name = properties
                .local_name
                .unwrap_or_else(|| "Unknown".to_string());
            println!("Found device: {} ({})", name, device.address());
        }
    }

    // Stop scanning
    central.stop_scan().await?;

    Ok(())
}

// async fn connect_and_send_data(peripheral: Peripheral) -> Result<(), Box<dyn Error>> {
//     peripheral.connect().await?;
//     println!("連接成功");

//     let data = b"Hello, client!";
//     peripheral
//         .write(UUID, data, WriteType::WithoutResponse)
//         .await?;
//     println!("數據已發送");

//     peripheral.disconnect().await?;
//     println!("已斷開連接");

//     Ok(())
// }
