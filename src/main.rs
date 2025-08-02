use std::process::exit;
use std::time::Instant;

use crate::files::Files;

mod files;
mod source_file;

fn main() {
    let stop_watch = Instant::now();
    let files = Files::new();

    if let Err(e) = files {
        eprintln!("{e:?}");
        exit(1);
    }

    let files = files.unwrap();
    let mut files_count: u32 = 0;
    let mut errors_count: u32 = 0;
    for file in files {
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
    if errors_count > 0 {
        eprintln!("Errors count: {errors_count}");
    }
    println!("Finished in {elapsed:?}");
}
