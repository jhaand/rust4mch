/* A small hello world example that works via the serial port. */ 

use esp_idf_sys;

fn main() {
    esp_idf_sys::link_patches();
    println!("Hello MCH from Rust");
}
