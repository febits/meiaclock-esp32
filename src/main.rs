use anyhow::Result;

use meiaclock_esp32::api::SubathonAPI;
use meiaclock_esp32::wifi::start_wifi;

use std::thread;
use std::time;

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let wifi = start_wifi(env!("SSID"), env!("PASSWORD"))?;
    let wifi = wifi.wifi();

    log::info!("{:?}", wifi.sta_netif().get_ip_info()?);

    let mut api = SubathonAPI::new(env!("API_URL"))?;

    loop {
        let timer = api.get_time_left()?;
        log::info!(
            "{:02}:{:02}:{:02}",
            timer.hours,
            timer.minutes,
            timer.seconds
        );
        thread::sleep(time::Duration::from_millis(500));
    }
}
