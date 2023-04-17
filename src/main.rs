use std::time::Duration;
use std::process;
use config::{Config, File, FileFormat};
use serde::Deserialize;
use tinkerforge::{ip_connection::IpConnection, ambient_light_v2_bricklet::AmbientLightV2Bricklet };
use reqwest::Client;
use tokio::time::sleep;

#[derive(Debug, Deserialize, PartialEq, Clone)]
struct Settings {
    host: String,
    port: u16,
    uid: String,
    item: String,
    openhab_url: String,
    threshold: u32,
}

#[tokio::main]
async fn main() {
    // Load configuration
    let settings = match load_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading configuration: {}", e);
            process::exit(1);
        }
    };

    // Check if running in systemd or standalone
    let is_systemd = match std::env::var("INVOCATION_ID") {
        Ok(_) => true,
        Err(_) => false,
    };

    let http_client = Client::new();
    if is_systemd {
        // Initialize systemd
        systemd::daemon::notify(false, [(systemd::daemon::STATE_READY, "1")].iter()).expect("Failed to notify systemd of readiness.");

        loop {
            systemd::daemon::notify(false, [(systemd::daemon::STATE_STATUS, "Main loop running")].iter()).expect("Failed to notify systemd of status.");
            sleep(Duration::from_secs(1)).await;
            if let Err(e) = update_living_room_sensor(&settings, &http_client).await {
                eprintln!("Error updating sensor: {}", e);
            }
        }
    } else {
        // Running in standalone mode
        println!("Running in standalone mode.");
        match update_living_room_sensor(&settings, &http_client).await {
            Ok(_) => println!("Sensor updated successfully."),
            Err(e) => eprintln!("Error updating sensor: {}", e),
        }
    }
}

async fn update_living_room_sensor(settings: &Settings, http_client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let ipcon = IpConnection::new();
    let sensor = AmbientLightV2Bricklet::new(&settings.uid, &ipcon);
    ipcon.connect((&settings.host[..], settings.port)).recv()??;
    

    let illuminance = sensor.get_illuminance().recv()? as f32;
    ipcon.disconnect();

    let openhab_item_url = if settings.openhab_url.ends_with('/') {
        format!("{}{}/state", settings.openhab_url, settings.item)
    } else {
        format!("{}/{}/state", settings.openhab_url, settings.item)
    };
    let itemstate = http_client.get(&openhab_item_url).send().await?.text().await?;
    let current_illuminance = match itemstate.parse::<f32>() {
        Ok(v) => v,
        Err(_) => 0.0,
    };
         
     

    if current_illuminance == 0.0 || (illuminance - current_illuminance).abs() as u32 >= settings.threshold {
        let illuminance_str = illuminance.to_string();
        http_client.put(&openhab_item_url)
            .body(illuminance_str)
            .header("Content-Type", "text/plain; charset=UTF-8")
            .send().await?;
    }

    Ok(())
}

fn load_config() -> Result<Settings, config::ConfigError> {
    let config = Config::builder().add_source(File::new("/etc/habforge/config.toml", FileFormat::Toml)).build()?;    
    config.try_deserialize::<Settings>()
}
