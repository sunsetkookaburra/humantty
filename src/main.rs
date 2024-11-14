use std::env::{self, args};
use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::exit;

use clap::Parser;
use owo_colors::OwoColorize;

macro_rules! cprint {
    ($text:expr, $map:expr) => {
        print!(
            "{}",
            $text.if_supports_color(owo_colors::Stream::Stdout, $map)
        )
    };
}

/// Display ASCII control codes using their symbolic names, and extended 8th-bit bytes as hex octets.
///
/// All with a bit of colour! (Disabled when using a TTY or via NO_COLOR=1)
#[derive(Parser)]
struct Cli {
    #[arg(long, env)]
    no_color: bool,
    /// Input file. Use "-" for stdin
    #[arg(default_value = "-")]
    file: PathBuf,
}

fn main() {
    if args().any(|s| s == "--no-color") {
        env::set_var("NO_COLOR", "1");
    }
    let cli = Cli::parse();

    let mut stream: Box<dyn BufRead> = if cli.file.as_os_str() == "-" {
        Box::new(stdin().lock())
    } else {
        Box::new(BufReader::new(File::open(cli.file).unwrap()))
    };

    loop {
        let buf = stream
            .fill_buf()
            .expect("Unexpected I/O error when filling read buffer");
        if buf.is_empty() {
            println!();
            exit(0);
        }
        for c in buf {
            match *c {
                0 => cprint!("<NUL>", |s| s.bright_black()),
                1 => cprint!("<SOH>", |s| s.green()),
                2 => cprint!("<STX>", |s| s.green()),
                3 => cprint!("<ETX>", |s| s.green()),
                4 => cprint!("<EOT>", |s| s.green()),
                5 => cprint!("<ENQ>", |s| s.green()),
                6 => cprint!("<ACK>", |s| s.green()),
                7 => cprint!("<BEL>", |s| s.red()),
                8 => cprint!("<BS>", |s| s.red()),
                9 => cprint!("<TAB>", |s| s.yellow()),
                10 => {
                    cprint!("<LF>", |s| s.yellow());
                    println!();
                }
                11 => cprint!("<VT>", |s| s.yellow()),
                12 => cprint!("<FF>", |s| s.yellow()),
                13 => cprint!("<CR>", |s| s.yellow()),
                14 => cprint!("<SO>", |s| s.green()),
                15 => cprint!("<SI>", |s| s.green()),
                16 => cprint!("<DLE>", |s| s.red()),
                17 => cprint!("<DC1>", |s| s.green()),
                18 => cprint!("<DC2>", |s| s.green()),
                19 => cprint!("<DC3>", |s| s.green()),
                20 => cprint!("<DC4>", |s| s.green()),
                21 => cprint!("<NAK>", |s| s.green()),
                22 => cprint!("<SYN>", |s| s.green()),
                23 => cprint!("<ETB>", |s| s.green()),
                24 => cprint!("<CAN>", |s| s.green()),
                25 => cprint!("<EM>", |s| s.green()),
                26 => cprint!("<SUB>", |s| s.green()),
                27 => cprint!("<ESC>", |s| s.red()),
                28 => cprint!("<FS>", |s| s.green()),
                29 => cprint!("<GS>", |s| s.green()),
                30 => cprint!("<RS>", |s| s.green()),
                31 => cprint!("<US>", |s| s.green()),
                127 => cprint!("<DEL>", |s| s.red()),
                c if c < 128 => print!("{}", c as char),
                c => cprint!(format_args!("<{:02x}>", c), |s| s.blue()),
            };
        }
        let len = buf.len();
        stream.consume(len);
        stdout()
            .flush()
            .expect("Unexpected I/O error when flushing output buffer");
    }
}
