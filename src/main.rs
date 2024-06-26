mod wifi_creds;

use esp_idf_sys as _;

/*

use core::fmt::Debug;
use embedded_svc::eth;
use embedded_svc::eth::{Eth, TransitionalState};
use embedded_svc::httpd::registry::*;
use embedded_svc::httpd::*;
use embedded_svc::io;
use embedded_svc::ipv4;
use embedded_svc::ping::Ping;
use embedded_svc::sys_time::SystemTime;
use embedded_svc::timer::TimerService;
use embedded_svc::timer::*;
use embedded_svc::wifi::*;

use esp_idf_svc::eth::*;
use esp_idf_svc::eventloop::*;
use esp_idf_svc::eventloop::*;
use esp_idf_svc::httpd as idf;
use esp_idf_svc::httpd::ServerRegistry;
use esp_idf_svc::netif::*;
use esp_idf_svc::nvs::*;
use esp_idf_svc::ping;
use esp_idf_svc::sntp;
use esp_idf_svc::sysloop::*;
use esp_idf_svc::systime::EspSystemTime;
use esp_idf_svc::timer::*;
use esp_idf_svc::wifi::*;
*/

//use esp_idf_hal::adc;
use esp_idf_hal::delay;
use esp_idf_hal::gpio;
//use esp_idf_hal::i2c;
use esp_idf_hal::prelude::*;
use esp_idf_hal::spi;

// use esp_idf_sys::{self, c_types};
// use esp_idf_sys::{esp, EspError};

use display_interface_spi::SPIInterfaceNoCS;
use embedded_hal::digital::v2::OutputPin;

use embedded_graphics::mono_font::{ascii::FONT_10X20, MonoTextStyle};
use embedded_graphics::pixelcolor::*;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::*;
use embedded_graphics::text::*;
use embedded_text::{
    alignment::HorizontalAlignment,
    style::{HeightMode, TextBoxStyleBuilder},
    TextBox,
};

use ili9341;

#[allow(dead_code)]
const SSID: &str = wifi_creds::SSID;
#[allow(dead_code)]
const PASS: &str = wifi_creds::PASS;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    test_print();

    #[allow(unused)]
    let peripherals = Peripherals::take().unwrap();
    #[allow(unused)]
    let pins = peripherals.pins;

    let mut display_control = pins.gpio26.into_output()?;
    display_control.set_low()?;

    mch_hello_world(
        pins.gpio33,
        pins.gpio25,
        peripherals.spi3,
        pins.gpio18,
        pins.gpio23,
        pins.gpio32,
    )?;
    Ok(())
}

fn test_print() {
    // Start simple
    println!("Hello from Rust!");

    // Check collections
    let mut children = vec![];

    children.push("foo");
    children.push("bar");
    println!("More complex print {:?}", children);
}

/*
fn test_fs() -> Result<()> {
    assert_eq!(fs::canonicalize(PathBuf::from("."))?, PathBuf::from("/"));
    assert_eq!(
        fs::canonicalize(
            PathBuf::from("/")
                .join("foo")
                .join("bar")
                .join(".")
                .join("..")
                .join("baz")
        )?,
        PathBuf::from("/foo/baz")
    );

    Ok(())
}
*/

fn mch_hello_world(
    dc: gpio::Gpio33<gpio::Unknown>,
    rst: gpio::Gpio25<gpio::Unknown>,
    spi: spi::SPI3,
    sclk: gpio::Gpio18<gpio::Unknown>,
    sdo: gpio::Gpio23<gpio::Unknown>,
    cs: gpio::Gpio32<gpio::Unknown>,
) -> anyhow::Result<()> {
    let config = <spi::config::Config as Default>::default().baudrate((40).MHz().into());

    let di = SPIInterfaceNoCS::new(
        spi::Master::<spi::SPI3, _, _, _, _>::new(
            spi,
            spi::Pins {
                sclk,
                sdo,
                sdi: Option::<gpio::Gpio21<gpio::Unknown>>::None,
                cs: Some(cs),
            },
            config,
        )?,
        dc.into_output()?,
    );

    let reset = rst.into_output()?;

    let display = ili9341::Ili9341::new(
        di,
        reset,
        &mut delay::Ets,
        KalugaOrientation::LandscapeVericallyFlipped,
        ili9341::DisplaySize240x320,
    );

    _ = draw_text(
        &mut display.unwrap(),
        &"".to_string(),
        &"Hello MCH2022 from Rust!".to_string(),
    );
    Ok(())
}

#[allow(dead_code)]
fn draw_text<D>(display: &mut D, text: &String, time: &String) -> Result<(), D::Error>
where
    D: DrawTarget + Dimensions,
    D::Color: From<Rgb565>,
{
    //let rect = Rectangle::new(display.bounding_box().top_left, display.bounding_box().size);

    display.clear(Rgb565::BLACK.into())?;
    //display.fill_solid(&rect, Rgb565::GREEN.into());

    Rectangle::new(Point::zero(), Size::new(300, 20)).into_styled(
        TextBoxStyleBuilder::new()
            .height_mode(HeightMode::FitToText)
            .alignment(HorizontalAlignment::Justified)
            .paragraph_spacing(3)
            .build(),
    );
    //.draw(display)?;

    Text::with_alignment(
        &time,
        Point::new(0, 15),
        MonoTextStyle::new(
            &embedded_graphics::mono_font::iso_8859_2::FONT_10X20,
            Rgb565::WHITE.into(),
        ),
        Alignment::Left,
    )
    .draw(display)?;
    
    Rectangle::new(Point::zero(), Size::new(300, 300)).into_styled(
        TextBoxStyleBuilder::new()
            //.height_mode(HeightMode::FitToText)
            .alignment(HorizontalAlignment::Justified)
            .paragraph_spacing(3)
            .build(),
    );
    //.draw(display)?;

    Text::with_alignment(
        &text,
        Point::new(0, 30),
        MonoTextStyle::new(
            &embedded_graphics::mono_font::iso_8859_2::FONT_10X20,
            Rgb565::WHITE.into(),
        ),
        Alignment::Left,
    )
    .draw(display)?;

    println!("Displaying done");

    Ok(())
}

// MCH2022 badge needs customized screen orientation commands
// (not a surprise; quite a few ILI9341 boards need these as evidenced in the TFT_eSPI & lvgl ESP32 C drivers)
pub enum KalugaOrientation {
    Portrait,
    PortraitFlipped,
    Landscape,
    LandscapeFlipped,
    LandscapeVericallyFlipped,
}


impl ili9341::Mode for KalugaOrientation {
    fn mode(&self) -> u8 {
        match self {
            Self::Portrait => 0,
            Self::LandscapeVericallyFlipped => 0x20,
            Self::Landscape => 0x20 | 0x40,
            Self::PortraitFlipped => 0x80 | 0x40,
            Self::LandscapeFlipped => 0x80 | 0x20,
        }
    }

    fn is_landscape(&self) -> bool {
        matches!(self, Self::Landscape | Self::LandscapeFlipped)
    }
}
