//use colored::Colorize;
use crossterm::{cursor, terminal, ExecutableCommand};
use std::io::stdout;

#[allow(dead_code)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

#[allow(dead_code)]
struct IntAndRgb {
    val: u8,
    r: u8,
    g: u8,
    b: u8,
}

pub fn print_values(u1: i32, _max: i32) -> std::io::Result<()> {
    let _colors = [
        IntAndRgb {
            val: 1,
            r: 53,
            g: 168,
            b: 255,
        },
        IntAndRgb {
            val: 2,
            r: 53,
            g: 140,
            b: 255,
        },
        IntAndRgb {
            val: 3,
            r: 53,
            g: 111,
            b: 255,
        },
        IntAndRgb {
            val: 4,
            r: 53,
            g: 83,
            b: 255,
        },
        IntAndRgb {
            val: 5,
            r: 25,
            g: 25,
            b: 255,
        },
        IntAndRgb {
            val: 6,
            r: 25,
            g: 82,
            b: 197,
        },
        IntAndRgb {
            val: 7,
            r: 25,
            g: 111,
            b: 168,
        },
        IntAndRgb {
            val: 8,
            r: 25,
            g: 140,
            b: 140,
        },
        IntAndRgb {
            val: 9,
            r: 25,
            g: 168,
            b: 111,
        },
        IntAndRgb {
            val: 10,
            r: 25,
            g: 197,
            b: 82,
        },
        IntAndRgb {
            val: 11,
            r: 25,
            g: 226,
            b: 53,
        },
        IntAndRgb {
            val: 12,
            r: 53,
            g: 255,
            b: 25,
        },
        IntAndRgb {
            val: 13,
            r: 140,
            g: 255,
            b: 25,
        },
        IntAndRgb {
            val: 14,
            r: 168,
            g: 255,
            b: 25,
        },
        IntAndRgb {
            val: 15,
            r: 197,
            g: 255,
            b: 25,
        },
        IntAndRgb {
            val: 16,
            r: 226,
            g: 255,
            b: 25,
        },
        IntAndRgb {
            val: 17,
            r: 255,
            g: 255,
            b: 25,
        },
        IntAndRgb {
            val: 18,
            r: 255,
            g: 226,
            b: 25,
        },
        IntAndRgb {
            val: 19,
            r: 255,
            g: 197,
            b: 25,
        },
        IntAndRgb {
            val: 20,
            r: 255,
            g: 168,
            b: 25,
        },
        IntAndRgb {
            val: 21,
            r: 255,
            g: 140,
            b: 25,
        },
        IntAndRgb {
            val: 22,
            r: 255,
            g: 111,
            b: 25,
        },
        IntAndRgb {
            val: 23,
            r: 255,
            g: 82,
            b: 25,
        },
        IntAndRgb {
            val: 24,
            r: 255,
            g: 25,
            b: 25,
        },
        IntAndRgb {
            val: 25,
            r: 178,
            g: 25,
            b: 25,
        },
    ];
    let mut stdout = stdout(); // lock stdout and use the same locked instance throughout
    stdout.execute(terminal::Clear(terminal::ClearType::FromCursorDown))?;
    let _col_phase: Rgb = Rgb { r: 0, g: 0, b: 0 };

    println!("{u1}");
    println!("#");
    //println!(
    //    "   {}",
    //    " ".to_string()
    //        .on_truecolor(col_phase.r, col_phase.g, col_phase.b)
    //        .truecolor(100, 100, 100), //.truecolor(col_phase.r, col_phase.g, col_phase.b)
    //);
    //println!(
    //    "{}     {}",
    //    " ".to_string().on_truecolor(255, 255, 255),
    //    " ".to_string().on_truecolor(255, 255, 255),
    //);
    //println!(
    //    "   {}",
    //    " ".to_string()
    //        .on_truecolor(col_phase.r, col_phase.g, col_phase.b)
    //        .truecolor(0, 0, 0), //.truecolor(col_phase.r, col_phase.g, col_phase.b)
    //);
    //println!("");
    //println!(
    //    "   {}",
    //    " ".to_string()
    //        .on_truecolor(col_phase.r, col_phase.g, col_phase.b)
    //        .truecolor(0, 0, 0), //.truecolor(col_phase.r, col_phase.g, col_phase.b)
    //);
    //println!(
    //    "{}     {}",
    //    " ".to_string().on_truecolor(255, 255, 255),
    //    " ".to_string().on_truecolor(255, 255, 255),
    //);
    //std::thread::sleep(std::time::Duration::from_millis(100));
    stdout.execute(cursor::MoveUp(2))?;
    Ok(())
}
