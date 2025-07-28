use std::process::exit;
use std::time::Instant;

use crate::files::Files;

mod files;

fn main() {
    let stop_watch = Instant::now();
    let files = Files::new();

    if let Err(e) = files {
        eprintln!("{e:?}");
        exit(1);
    }

    let mut files_count = 0u32;
    let mut errors_count = 0u32;
    for file in files.unwrap() {
        match file {
            Err(e) => {
                errors_count += 1;
                eprintln!("{e:?}");
            }
            Ok(f) => {
                files_count += 1;
                println!("{f:?}");
            }
        }
    }

    let elapsed = stop_watch.elapsed();
    println!("Processed {files_count} files");
    println!("Finished in {elapsed:?}");
}
