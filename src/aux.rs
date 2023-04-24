use crate::colors::print_values;
use std::cmp;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::net::UdpSocket;
use std::{thread, time::Duration};

pub fn formatter(msg: f32, mut max_msg: i32, max_arduino: i32) -> String {
    if max_msg == 0 {
        max_msg = 1
    };
    let result = format!("{}", msg as i32 * max_arduino / max_msg);
    result
}

pub fn mytrim(word: String) -> String {
    let mut result: Vec<char> = [].to_vec();
    for c in word.chars() {
        if c as u32 != 0 {
            result.push(c);
        }
    }
    result.into_iter().collect()
}

#[allow(dead_code)]
fn run_from_mock() -> std::io::Result<()> {
    let path_s = "./outsim.txt";
    let path_g = "./outgauge.txt";
    let f_s = File::open(path_s).expect("no file found");
    let f_g = File::open(path_g).expect("no file found");
    let br_s = BufReader::new(f_s);
    let br_g = BufReader::new(f_g);
    let lines_s: Vec<Vec<u8>> = br_s
        .split(b'\n')
        .collect::<Result<_, _>>()
        .unwrap_or_else(|_| panic!("Failed converting file into lines. Path: {}", path_s));
    let lines_g: Vec<Vec<u8>> = br_g
        .split(b'\n')
        .collect::<Result<_, _>>()
        .unwrap_or_else(|_| panic!("Failed converting file into lines. Path: {}", path_g));

    let mut _input_len = 0;
    _input_len = cmp::max(lines_s.len(), lines_g.len());
    for ix in 0.._input_len {
        if lines_g.len() <= _input_len {
            let outgauge_pack = &lines_g[ix];
            if outgauge_pack.len() >= 5 {
                let _rpm = outgauge_pack[6];
                println!("RPM {}", _rpm);
            }
        }
        println!("-------------------");
        thread::sleep(Duration::from_millis(50));
    }
    Ok(())
}

#[allow(dead_code)]
fn record_data() -> std::io::Result<()> {
    // TODO: format this in a way that is similar to the regular UDP packets
    //       also: one line of data for each datapoint
    // Create UDP sockets.
    let sock_outsim = UdpSocket::bind("127.0.0.1:8000")?;
    let sock_outgauge = UdpSocket::bind("127.0.0.1:8001")?;

    let mut buf_outsim = [0u8; 256];
    let mut buf_outgauge = [0u8; 256];

    let pathsim = "outsim.txt";
    let pathgauge = "outgauge.txt";
    let mut output_s = File::create(pathsim)?;
    let mut output_g = File::create(pathgauge)?;

    loop {
        // Receive data.
        let (amt_s, _src_s) = sock_outsim.recv_from(&mut buf_outsim)?;
        let data_s = &buf_outsim[..amt_s];
        let (amt_g, _src_g) = sock_outgauge.recv_from(&mut buf_outgauge)?;
        let data_g = &buf_outgauge[..amt_g];

        if data_s.is_empty() {
            break; // Lost connection
        }
        if data_g.is_empty() {
            break; // Lost connection
        }
        output_s.write_all(data_s)?;
        output_g.write_all(data_g)?;
    }

    // Release the socket.
    drop(sock_outsim);
    drop(sock_outgauge);

    Ok(())
}

#[allow(dead_code)]
fn test_colors() -> std::io::Result<()> {
    let max = 20000;
    for i in 0..max {
        print_values(i, max)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lfs_to_wheel() {
        let lfs_value = 32767;
        let wheel_value = lfs_to_wheel(lfs_value);
        assert_eq!(wheel_value, 1.0);

        let lfs_value = 16383;
        let wheel_value = lfs_to_wheel(lfs_value);
        assert_eq!(wheel_value, 0.5);

        let lfs_value = 0;
        let wheel_value = lfs_to_wheel(lfs_value);
        assert_eq!(wheel_value, -1.0);
    }

    #[test]
    fn test_wheel_to_lfs() {
        let wheel_value = 1.0;
        let lfs_value = wheel_to_lfs(wheel_value);
        assert_eq!(lfs_value, 32767);

        let wheel_value = 0.5;
        let lfs_value = wheel_to_lfs(wheel_value);
        assert_eq!(lfs_value, 16383);

        let wheel_value = -1.0;
        let lfs_value = wheel_to_lfs(wheel_value);
        assert_eq!(lfs_value, 0);
    }
}
