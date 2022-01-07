extern crate rpassword;

use image::Luma;
use qrcode::QrCode;
use rpassword::read_password;
use std::error::Error;
use std::io::Write;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Enter your WIFI SSID:");
    let mut ssid = String::new();
    std::io::stdin().read_line(&mut ssid).unwrap();

    println!("\nEnter your WIFI password:");
    std::io::stdout().flush().unwrap();
    let password = read_password().unwrap();

    let code = format!(
        "WIFI:T:WPA;S:{ssid};P:{password};;",
        ssid = ssid.trim(),
        password = password.trim()
    );
    let qrcode = QrCode::new(code.as_bytes()).unwrap();
    let qrimage = qrcode.render::<Luma<u8>>().build();
    let qrname = "/tmp/qrcode.png";
    qrimage.save(qrname).unwrap();

    Command::new("open")
        .arg(qrname)
        .spawn()
        .expect("open command failed to start");

    Ok(())
}
