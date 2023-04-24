use std::net::UdpSocket;

pub fn get_os_data(outsim_addr: &str) -> std::io::Result<()> {
    let mut buf_outsim = [0; 256];
    let sock_outsim = UdpSocket::bind(outsim_addr)?;
    let conn = match sock_outsim.recv_from(&mut buf_outsim) {
        Ok(a) => Some(a),
        Err(e) => {
            println!("ERROR ON RECV: {}", e);
            None
        }
    };
    let amt_s = match conn {
        Some(u) => u.0,
        None => 0,
    };
    let data_s = &buf_outsim[..amt_s];
    if data_s.len() > 20 {
        //println!("{:?}", data_s.len()); // 256
        println!(
            "{:?} {:?} {:?} {:?} {:?}||{:?} {:?} {:?} {:?} {:?}",
            data_s[10],
            data_s[11],
            data_s[12],
            data_s[13],
            data_s[14],
            data_s[15],
            data_s[16],
            data_s[17],
            data_s[18],
            data_s[19],
        );
        //let car_raw = [data_g[3], data_g[4], data_g[5], data_g[6]];
        // TODO: this does not work well with MODS
        //let car_str = std::str::from_utf8(&car_raw).unwrap_or("AAA");
        //let car = mytrim(car_str.trim().to_string());
        //rpm = f32::from_le_bytes([data_g[16], data_g[17], data_g[18], data_g[19]]);
        //maxrpm = car_maxrpm.get(&car).copied().unwrap_or(15000);
    }
    Ok(())
}

// REFERENCE
//  0 always 76?
//  1 always 70?
//  2 always 83?
//  3 always 84?
//  4 always 0?
//  5 always 0?
//  6 always 0?
//  7 always 0?
//  8 0 to 255 (catched every tenth)
//  9 when 8 gets to 255, this one increases by 1
// 10 always 0?
// 11 always 0?
//
// REFERENCE PREV
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_packet() {
        let packet = [
            1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
        ];
        let parsed_packet = parse_packet(&packet);
        assert_eq!(parsed_packet.pos_x, 0.0);
        assert_eq!(parsed_packet.pos_y, 0.0);
        assert_eq!(parsed_packet.pos_z, 0.0);
        assert_eq!(parsed_packet.vel_x, 0.0);
        assert_eq!(parsed_packet.vel_y, 0.0);
        assert_eq!(parsed_packet.vel_z, 0.0);
        assert_eq!(parsed_packet.roll, 0.0);
        assert_eq!(parsed_packet.pitch, 0.0);
        assert_eq!(parsed_packet.yaw, 0.0);
    }

    #[test]
    fn test_process_packet() {
        let mut game_state = GameState::new();
        let packet = [
            1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
        ];
        process_packet(&mut game_state, &packet);
        assert_eq!(game_state.pos_x, 0.0);
        assert_eq!(game_state.pos_y, 0.0);
        assert_eq!(game_state.pos_z, 0.0);
        assert_eq!(game_state.vel_x, 0.0);
        assert_eq!(game_state.vel_y, 0.0);
        assert_eq!(game_state.vel_z, 0.0);
        assert_eq!(game_state.roll, 0.0);
        assert_eq!(game_state.pitch, 0.0);
        assert_eq!(game_state.yaw, 0.0);
    }
}
