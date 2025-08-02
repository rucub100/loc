use std::process::exit;
use std::time::Instant;

use crate::files::Files;
use crate::source_file::SourceFile;

mod files;
mod ignore;
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
    let mut loc_count: u32 = 0;
    for file in files {
        match file {
            Err(e) => {
                errors_count += 1;
                eprintln!("{e:?}");
            }
            Ok(path) => {
                if let Some(source_file) = SourceFile::from_path(path) {
                    files_count += 1;
                    let loc = source_file.loc();
                    loc_count += loc;
                    println!(
                        "{:?}: {:?} - {:?}",
                        source_file.get_lang(),
                        source_file.get_path(),
                        loc
                    );
                }
            }
        }
    }

    let elapsed = stop_watch.elapsed();
    println!("Processed {files_count} files");
    println!("Lines of code: {loc_count}");
    if errors_count > 0 {
        eprintln!("Errors count: {errors_count}");
    }
    println!("Finished in {elapsed:?}");
}
