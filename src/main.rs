use anyhow::Result;

use meiaclock_esp32::api::SubathonAPI;
use meiaclock_esp32::display::Display;
use meiaclock_esp32::wifi::start_wifi;

use std::thread;
use std::time::{Duration, Instant};

use esp_idf_svc::hal::gpio::AnyIOPin;
use esp_idf_svc::hal::peripherals::Peripherals;

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;

    let wifi = start_wifi(env!("SSID"), env!("PASSWORD"), peripherals.modem)?;
    log::info!("{:?}", wifi.wifi().sta_netif().get_ip_info()?);

    let mut api = SubathonAPI::new(env!("API_URL"))?;
    let mut display = Display::new(
        peripherals.i2c0,
        unsafe { AnyIOPin::new(env!("SDA_PIN").parse::<i32>()?) },
        unsafe { AnyIOPin::new(env!("SCL_PIN").parse::<i32>()?) },
    )?;

    display.init_display();
    display.draw_meianatal();

    thread::sleep(Duration::from_secs(2));

    loop {
        let start = Instant::now();
        let timer = match api.get_time_left() {
            Ok(t) => t,
            Err(e) => {
                log::error!("Failed to get timeLeft from API: {e}. Restarting the system...");
                unsafe {
                    esp_idf_svc::sys::esp_restart();
                }
            }
        };

        display.draw_timer(
            format!(
                "{:02}:{:02}:{:02}",
                timer.hours, timer.minutes, timer.seconds
            )
            .as_str(),
        );

        let elapsed = start.elapsed();

        if elapsed < Duration::from_millis(1000) {
            thread::sleep(Duration::from_millis(1000) - elapsed);
        }
    }
}
