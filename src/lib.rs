use bincode::deserialize;
use hidapi::HidApi;
use hidapi::HidDevice;
use serde::Deserialize;
use std::io::{Error, ErrorKind};
use std::result::Result;

const MIRA_VID: u16 = 0x0416;
const MIRA_PID: u16 = 0x5020;

const USB_REPORT_ID: u8 = 0x00;
const HID_TIMEOUT: i32 = 5000;

pub struct Mira {
    device: HidDevice,
}

enum OpCode {
    Refresh = 0x01,
    SetRefreshMode = 0x02,
    SetSpeed = 0x04,
    SetContrast = 0x05,
    SetColdLight = 0x06,
    SetWarmLight = 0x07,
    SetColourFilter = 0x11,
    Reset = 0x1f,
    GetStatus = 0x8f,
}

#[derive(Debug)]
pub enum DisplayMode {
    Speed,
    Text,
    Image,
    Video,
    Read,
}

#[derive(Debug)]
pub enum RefreshMode {
    Direct = 0x01,
    Grey = 0x02,
    A2 = 0x03,
}

pub struct Info {
    pub manufacturer: String,
    pub product: String,
    pub serial: String,
}

#[derive(Deserialize, Debug)]
pub struct Status {
    _a: u8,
    _b: u8,
    _d: u8,
    _e: u8,
    pub speed: u8,
    pub contrast: u8,
    pub cold_light: u8,
    pub warm_light: u8,
    _f: u8,
    pub refresh_mode: u8,
}

impl Mira {
    pub fn new() -> Result<Mira, Error> {
        match HidApi::new() {
            Ok(api) => {
                if let Ok(monitor) = api.open(MIRA_VID, MIRA_PID) {
                    Ok(Mira { device: monitor })
                } else {
                    Err(Error::new(
                        ErrorKind::Other,
                        "Could not connect to monitor",
                    ))
                }
            }
            Err(_) => Err(Error::new(
                ErrorKind::Other,
                "Could not instantiate HID API",
            )),
        }
    }

    pub fn get_info(&self) -> Info {
        Info {
            manufacturer: self
                .device
                .get_manufacturer_string()
                .unwrap()
                .unwrap(),
            product: self.device.get_product_string().unwrap().unwrap(),
            serial: self.device.get_serial_number_string().unwrap().unwrap(),
        }
    }

    fn write(&self, data: &[u8]) -> Result<(), Error> {
        let buf = [&[USB_REPORT_ID], data].concat();
        match self.device.write(&buf) {
            Ok(_) => Ok(()),
            Err(_) => {
                Err(Error::new(ErrorKind::Other, "failed to write to monitor"))
            }
        }
    }

    pub fn refresh(&self) -> Result<(), Error> {
        self.write(&[OpCode::Refresh as u8])
    }

    pub fn set_refresh_mode(&self, mode: RefreshMode) -> Result<(), Error> {
        self.write(&[OpCode::SetRefreshMode as u8, mode as u8])
    }

    pub fn set_speed(&self, speed: u8) -> Result<(), Error> {
        let adjusted_speed = 11 - speed.clamp(1, 7);
        self.write(&[OpCode::SetSpeed as u8, adjusted_speed])
    }

    pub fn set_cold_light(&self, brightness: u8) -> Result<(), Error> {
        self.write(&[OpCode::SetColdLight as u8, brightness.clamp(0, 254)])
    }

    pub fn set_warm_light(&self, brightness: u8) -> Result<(), Error> {
        self.write(&[OpCode::SetWarmLight as u8, brightness.clamp(0, 254)])
    }

    pub fn set_contrast(&self, contrast: u8) -> Result<(), Error> {
        self.write(&[OpCode::SetContrast as u8, contrast])
    }

    pub fn set_colour_filter(&self, white: u8, black: u8) -> Result<(), Error> {
        self.write(&[
            OpCode::SetColourFilter as u8,
            255 - white.clamp(0, 127),
            black.clamp(0, 127),
        ])
    }

    pub fn get_status(&self) -> Status {
        self.write(&[OpCode::GetStatus as u8]).unwrap();

        let mut buf = [0; 64];
        self.device.read_timeout(&mut buf[..], HID_TIMEOUT).unwrap();
        println!("{buf:?}");

        deserialize(&buf[..]).unwrap()
    }

    pub fn reset(&self) -> Result<(), Error> {
        self.write(&[OpCode::Reset as u8])
    }

    pub fn set_display_mode(&self, mode: DisplayMode) -> Result<(), Error> {
        let mut refresh_mode = RefreshMode::A2;
        let mut contrast = 7;
        let mut speed = 5;
        let mut filter = (0, 0);

        match mode {
            DisplayMode::Speed => {
                contrast = 8;
                speed = 7;
            }
            DisplayMode::Text => {
                speed = 6;
            }
            DisplayMode::Image => {
                refresh_mode = RefreshMode::Direct;
            }
            DisplayMode::Video => {
                speed = 6;
                filter = (10, 0);
            }
            DisplayMode::Read => {
                filter = (12, 10);
            }
        }

        self.set_refresh_mode(refresh_mode)?;
        self.set_contrast(contrast)?;
        self.set_speed(speed)?;
        self.set_colour_filter(filter.0, filter.1)?;
        Ok(())
    }
}
