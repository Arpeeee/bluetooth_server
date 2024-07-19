use btleplug::api::{Central, Manager as _, Peripheral, PeripheralProperties, ScanFilter};
use btleplug::platform::Manager;
use std::error::Error;
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let manager = Manager::new().await?;
    let adapters = manager.adapters().await?;
    let central = adapters.into_iter().next().unwrap();

    // Start scanning for devices
    central.start_scan(ScanFilter::default()).await?;
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Set up your service and characteristic UUIDs
    let service_uuid = uuid::Uuid::parse_str("YOUR_SERVICE_UUID_HERE")?;
    let characteristic_uuid = uuid::Uuid::parse_str("YOUR_CHARACTERISTIC_UUID_HERE")?;

    // Create a peripheral
    let local_name = "RustBLEServer";
    let mut peripheral = central.peripheral(local_name).await?;

    // Add a service and characteristic
    peripheral.add_service(service_uuid).await?;
    peripheral
        .add_characteristic(service_uuid, characteristic_uuid, vec![])
        .await?;

    // Start advertising
    peripheral.start_advertising().await?;

    println!("BLE Server started. Advertising as '{}'", local_name);

    // Keep the program running
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}
