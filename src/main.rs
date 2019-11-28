use structopt::StructOpt;
use std::fs::File;
use std::io::{Read, stdin};
use std::process::exit;

#[derive(StructOpt, Debug)]
struct Cli {
    files: Vec<String>
}

static mut CTRLC_PRESSED: bool = false;

fn main() {
    let args = Cli::from_args();

    let mut freqs = [0usize; 256];
    let mut buffer = vec![0; 1024 * 1024 * 32];

    let mut process = |stream: &mut dyn Read| {
        while let Ok(count) = stream.read(buffer.as_mut_slice()) {
            if count == 0 || unsafe { CTRLC_PRESSED } {
                break;
            }
            for el in &buffer[0..count] {
                freqs[*el as usize] += 1;
            }
        }
    };

    ctrlc::set_handler(|| unsafe { CTRLC_PRESSED = true } );

    if args.files.len() > 0 {
        for file in args.files {
            let mut fd = File::open(file).unwrap();
            process(&mut fd);
            if unsafe { CTRLC_PRESSED } {
                break;
            }
        }
    }
    else {
        process(&mut stdin());
    }

    println!("[[Global chars occurrences]]");
    for (idx, cnt) in freqs.iter().enumerate() {
        if *cnt != 0 {
            println!("\t{}: {}", idx as u8 as char, *cnt);
        }
    }
}