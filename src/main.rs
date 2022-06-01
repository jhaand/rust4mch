#![allow(unused_imports)]
#![allow(clippy::single_component_path_imports)]

use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::{Condvar, Mutex};
use std::{cell::RefCell, env, sync::atomic::*, sync::Arc, thread, time::*};

use anyhow::bail;

use embedded_svc::mqtt::client::utils::ConnState;
use log::*;

use url;

use smol;

use embedded_hal::adc::OneShot;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::OutputPin;

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

use esp_idf_hal::adc;
use esp_idf_hal::delay;
use esp_idf_hal::gpio;
use esp_idf_hal::i2c;
use esp_idf_hal::prelude::*;
use esp_idf_hal::spi;

use esp_idf_sys::{self, c_types};
use esp_idf_sys::{esp, EspError};

use display_interface_spi::SPIInterfaceNoCS;

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

use ili9341::{self, Orientation};


fn main() -> Result<()> {
    esp_idf_sys::link_patches();

    test_print();

    #[cfg(not(esp_idf_version = "4.3"))]
    test_fs()?;
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    let spi = peripherals.spi3;
    let sclk = pins.gpio18;
    let mosi = pins.gpio23;
    let dc = pins.gpio33;
    let rst = pins.gpio21;
    let cs = pins.gpio32;

    let backlight = pins.gpio5;
    let miso = pins.gpio25;

    let config = <spi::config::Config as Default>::default().baudrate((26_000_000).into());

    //https://docs.espressif.com/projects/esp-idf/en/latest/esp32/api-reference/peripherals/spi_master.html
    // If using real HW wrover-kit, please, use spi::Master::<spi::SPI2, _, _, _, _>::new instead of spi::Master::<spi::SPI3, _, _, _, _>::new
    let di = SPIInterfaceNoCS::new(
        spi::Master::<spi::SPI3, _, _, _, _>::new(
            spi,
            spi::Pins {
                sclk,
                sdo: mosi,
                sdi: Some(miso),
                cs: Some(cs),
            },
            config,
        )?,
        dc.into_output()?,
    );

    let reset = rst.into_output()?;
    let backlight = backlight.into_output()?;

    let mut display = ili9341::Ili9341::new(
        di,
        reset,
        &mut delay::Ets,
        Orientation::Landscape,
        ili9341::DisplaySize240x320,
    )
    .map_err(|_| anyhow::anyhow!("Display"))?;

    draw_text(
        &mut display,
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

    info!("Displaying done");

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

fn test_tcp() -> Result<()> {
    info!("About to open a TCP connection to 1.1.1.1 port 80");

    let mut stream = TcpStream::connect("one.one.one.one:80")?;

    let err = stream.try_clone();
    if let Err(err) = err {
        info!(
            "Duplication of file descriptors does not work (yet) on the ESP-IDF, as expected: {}",
            err
        );
    }

    stream.write_all("GET / HTTP/1.0\n\n".as_bytes())?;

    let mut result = Vec::new();

    stream.read_to_end(&mut result)?;

    info!(
        "1.1.1.1 returned:\n=================\n{}\n=================\nSince it returned something, all is OK",
        std::str::from_utf8(&result)?);

    Ok(())
}

#[cfg(feature = "kaluga")]
fn kaluga_hello_world(
    backlight: gpio::Gpio6<gpio::Unknown>,
    dc: gpio::Gpio13<gpio::Unknown>,
    rst: gpio::Gpio16<gpio::Unknown>,
    spi: spi::SPI3,
    sclk: gpio::Gpio15<gpio::Unknown>,
    sdo: gpio::Gpio9<gpio::Unknown>,
    cs: gpio::Gpio11<gpio::Unknown>,
    ili9341: bool,
) -> Result<()> {
    info!(
        "About to initialize the Kaluga {} SPI LED driver",
        if ili9341 { "ILI9341" } else { "ST7789" }
    );

    let config = <spi::config::Config as Default>::default()
        .baudrate((if ili9341 { 40 } else { 80 }).MHz().into());

    let mut backlight = backlight.into_output()?;
    backlight.set_high()?;

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

    if ili9341 {
        let mut display = ili9341::Ili9341::new(
            di,
            reset,
            &mut delay::Ets,
            KalugaOrientation::Landscape,
            ili9341::DisplaySize240x320,
        )?;

        led_draw(&mut display).map_err(|e| anyhow::anyhow!("Display error: {:?}", e))
    } else {
        let mut display = st7789::ST7789::new(di, reset, 320, 240);

        display
            .init(&mut delay::Ets)
            .map_err(|e| anyhow::anyhow!("Display error: {:?}", e))?;
        display
            .set_orientation(st7789::Orientation::Landscape)
            .map_err(|e| anyhow::anyhow!("Display error: {:?}", e))?;

        led_draw(&mut display).map_err(|e| anyhow::anyhow!("Display error: {:?}", e))
    }
}

