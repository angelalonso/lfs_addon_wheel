use lfs_addon_wheel::aux::formatter;
use lfs_addon_wheel::outgauge::get_data;
use load_dotenv::load_dotenv;
use std::io;
use std::thread;
use std::time::Duration;

// TODO: use logger
// TODO: ask ChatGPT to write tests
// TODO: check it runs as root, check android IDE Serial monitor is on

fn run() -> std::io::Result<()> {
    load_dotenv!();
    let og_addr = env!("OUTGAUGE_UDP_ADDR");
    let portpath = env!("ARDUINO_USB_DEV");
    let baudrate = env!("ARDUINO_BAUDRATE").parse::<u32>().unwrap();
    let arduino_read_delay = env!("ARDUINO_READ_DELAY").parse::<u64>().unwrap();
    let max_vibr = env!("ARDUINO_MAX_VIBR").parse::<i32>().unwrap();
    let mut _rpm: f32 = 0.0;
    let mut _maxrpm: i32 = 0;
    loop {
        let handle = thread::spawn(move || {
            // Getting data from Live For Speed
            //TODO:  why clippy complains?
            (_rpm, _maxrpm) = get_data(og_addr).unwrap_or((0.0, 0));
            //println!("DEBUG: got {} and {} from LFS", rpm, maxrpm);

            // Sending/receiving data from Arduino
            match serialport::new(portpath, baudrate)
                .timeout(Duration::from_millis(arduino_read_delay))
                .open()
            {
                Ok(mut sp) => {
                    //println!("DEBUG: sending {_rpm} to Arduino");
                    let form_rpm = formatter(_rpm, _maxrpm, max_vibr);
                    match sp.write(format!(">||{}|{}||<", form_rpm, form_rpm).as_bytes()) {
                        Ok(_) => {
                            //println!("DEBUG: -> wrote {}", format!(">||{}|||<", form_rpm));
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                        Err(e) => println!("ERROR: {:?}", e),
                    }

                    Some(sp)
                }
                Err(e) => {
                    if e.kind() == serialport::ErrorKind::Io(std::io::ErrorKind::NotFound) {
                        println!("ERROR: Serial not connected");
                    } else {
                        println!("ERROR: {:?}", e);
                    };
                    None
                }
            };
        });
        handle.join().unwrap();
    }
}

#[allow(dead_code)]
fn run_outsim() -> std::io::Result<()> {
    Ok(())
}

fn main() -> std::io::Result<()> {
    // Not needed for now (see below)
    //use std::io::{stdin, stdout, Write};
    //let mut s = String::new();
    println!("   ----------------------------------------");
    println!("   - Rumble controller for Live for Speed -");
    println!("   ----------------------------------------");
    println!();
    // This is here for when we want to pause start:
    //println!("Press ENTER to continue");
    //let _ = stdout().flush();
    //stdin().read_line(&mut s).expect("Please press ENTER");
    println!("Waiting for LFS to start...");
    //match run_outsim() {
    match run() {
        Ok(_) => println!("All Good"),
        Err(e) => println!("ERROR: {:?}", e),
    };

    Ok(())
}
