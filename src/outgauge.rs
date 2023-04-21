use crate::aux::mytrim;
use std::collections::HashMap;
use std::net::UdpSocket;

pub fn get_data(outgauge_addr: &str) -> std::io::Result<(f32, i32)> {
    let mut car_maxrpm = HashMap::new();
    car_maxrpm.insert(String::from("BF1"), 19912); // 3452
    car_maxrpm.insert(String::from("FO8"), 9476); // 1852
    car_maxrpm.insert(String::from("FBM"), 9179); // 1879
    car_maxrpm.insert(String::from("FOX"), 7481); // 1867
    car_maxrpm.insert(String::from("FXO"), 7482); // 955
    car_maxrpm.insert(String::from("FXR"), 7492); //1408
    car_maxrpm.insert(String::from("FZ5"), 7971); //951
    car_maxrpm.insert(String::from("FZR"), 8474); //1425
    car_maxrpm.insert(String::from("LX4"), 8974); //934
    car_maxrpm.insert(String::from("LX6"), 8975); //934
    car_maxrpm.insert(String::from("MRT"), 12921); //966
    car_maxrpm.insert(String::from("RAC"), 6985); //964
    car_maxrpm.insert(String::from("RB4"), 7480); //958
    car_maxrpm.insert(String::from("UF1"), 6983); //964
    car_maxrpm.insert(String::from("UFR"), 8978); //1430
    car_maxrpm.insert(String::from("XFG"), 7978); //950
    car_maxrpm.insert(String::from("XFR"), 7979); //1424
    car_maxrpm.insert(String::from("XRG"), 6980); //951
    car_maxrpm.insert(String::from("XRR"), 7492); //1408
    car_maxrpm.insert(String::from("XRT"), 7480); //958

    let mut rpm: f32 = 0.0;
    let mut maxrpm: i32 = 0;
    let mut buf_outgauge = [0; 256];
    let sock_outgauge = UdpSocket::bind(outgauge_addr)?;
    let conn = match sock_outgauge.recv_from(&mut buf_outgauge) {
        Ok(a) => Some(a),
        Err(e) => {
            println!("ERROR ON RECV: {}", e);
            None
        }
    };
    let amt_g = match conn {
        Some(u) => u.0,
        None => 0,
    };
    let data_g = &buf_outgauge[..amt_g];
    if data_g.len() > 20 {
        let car_raw = [data_g[3], data_g[4], data_g[5], data_g[6]];
        // TODO: this does not work well with MODS
        let car_str = std::str::from_utf8(&car_raw).unwrap_or("AAA");
        let car = mytrim(car_str.trim().to_string());
        rpm = f32::from_le_bytes([data_g[16], data_g[17], data_g[18], data_g[19]]);
        maxrpm = car_maxrpm.get(&car).copied().unwrap_or(15000);
    }
    Ok((rpm, maxrpm))
}

// REFERENCE
//
// Unpack the data for Outsim.
//let outsim_pack: [u8; 256] = unsafe { mem::transmute(buf_outsim) };
//let _time_s = outsim_pack[0];
//let _angvel = [outsim_pack[1], outsim_pack[2], outsim_pack[3]];
//let _header = outsim_pack[4] as i32;
//let _pitch = outsim_pack[5];
//let _roll = outsim_pack[6];
//let _accel = [outsim_pack[7], outsim_pack[8], outsim_pack[9]];
//let _vel = [outsim_pack[10], outsim_pack[11], outsim_pack[12]];
//let _pos = [outsim_pack[13], outsim_pack[14], outsim_pack[15]];
//let outgauge_pack: [u8; 256] = unsafe { mem::transmute(buf_outgauge) };
//let _time_g = outgauge_pack[0];
//let _car = outgauge_pack[1];
//let _flags = outgauge_pack[2];
//let _gear = outgauge_pack[3];
//let _speed = outgauge_pack[5]; // this is not showing wnything that changes
//let _rpm = outgauge_pack[6];
//let _turbo = outgauge_pack[7];
//let _engtemp = outgauge_pack[8];
//let _fuel = outgauge_pack[9];
//let _oilpressure = outgauge_pack[10];
//let _oiltemp = outgauge_pack[11];
//let _dashlights = outgauge_pack[12];
//let _showlights = outgauge_pack[13];
//let _throttle = outgauge_pack[14];
//let _brake = outgauge_pack[15];
//let _clutch = outgauge_pack[16];
//let _display1 = outgauge_pack[17];
//let _display2 = outgauge_pack[18];
