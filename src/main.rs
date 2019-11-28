use structopt::StructOpt;
use std::fs::File;
use std::io::Read;

#[derive(StructOpt, Debug)]
struct Cli {
    files: Vec<String>
}

fn main() {
    let args = Cli::from_args();

    let mut freqs = [0usize; 256];
    let mut buffer = [0; 1024 * 1024];

    for file in args.files {
        let mut fd = File::open(file).unwrap();
        while let Ok(count) = fd.read(&mut buffer) {
            if count == 0 {
                break;
            }
            for el in &buffer[0..count] {
                freqs[*el as usize] += 1;
            }
        }
    }

    println!("[[Global chars occurrences]]");
    for (idx, cnt) in freqs.iter().enumerate() {
        if *cnt != 0 {
            println!("\t{}: {}", idx as u8 as char, *cnt);
        }
    }
}