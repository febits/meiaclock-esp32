use anyhow::Result;
use miniserde::{json, Deserialize};

use embedded_svc::http::client::Client as HttpClient;
use esp_idf_svc::http::client::{Configuration as HttpConfiguration, EspHttpConnection, Method};

const BUFFSIZE: usize = 1024;

#[derive(Deserialize)]
struct ResponseJSON {
    timeLeft: u64,
}

pub struct SubathonAPI {
    api_url: &'static str,
    client: HttpClient<EspHttpConnection>,
    buf: [u8; BUFFSIZE],
}

pub struct SubathonTimer {
    pub hours: u64,
    pub minutes: u64,
    pub seconds: u64,
}

impl From<u64> for SubathonTimer {
    fn from(value: u64) -> Self {
        let milliseconds = value;
        let hours = milliseconds / (3600 * 1000);
        let minutes = (milliseconds % (3600 * 1000)) / (60 * 1000);
        let seconds = (milliseconds % (60 * 1000)) / 1000;

        Self {
            hours,
            minutes,
            seconds,
        }
    }
}

impl From<SubathonTimer> for u64 {
    fn from(value: SubathonTimer) -> Self {
        (value.hours * (3600 * 1000)) + (value.minutes * (60 * 1000)) + (value.seconds * 1000)
    }
}

impl SubathonAPI {
    pub fn new(api_url: &'static str) -> Result<Self> {
        let config = &HttpConfiguration {
            crt_bundle_attach: Some(esp_idf_svc::sys::esp_crt_bundle_attach),
            ..Default::default()
        };
        let client = HttpClient::wrap(EspHttpConnection::new(config)?);

        Ok(Self {
            api_url,
            client,
            buf: [0u8; BUFFSIZE],
        })
    }

    pub fn get_time_left(&mut self) -> Result<SubathonTimer> {
        let mut response = self
            .client
            .request(
                Method::Get,
                self.api_url,
                &[("User-Agent", "meiaclock-esp/0.1")],
            )?
            .submit()?;

        let bytes_read = response.read(&mut self.buf)?;
        let response_body = std::str::from_utf8(&self.buf[0..bytes_read])?;

        let rjson: ResponseJSON = json::from_str(response_body)?;
        let timer = SubathonTimer::from(rjson.timeLeft);

        Ok(timer)
    }
}
