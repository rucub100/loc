use crate::files::{Files, FilesError};
use crate::source_file::SourceFile;
use std::process::exit;
use std::time::Instant;
use tokio::task::JoinSet;

mod files;
mod ignore;
mod source_file;

#[tokio::main]
async fn main() {
    let stop_watch = Instant::now();
    let files = Files::new();

    if let Err(e) = files {
        eprintln!("{e:?}");
        exit(1);
    }

    let files = files.unwrap();
    let mut handles = JoinSet::new();
    for file in files {
        handles.spawn(async move {
            let result: Option<Result<u32, FilesError>> = match file {
                Err(e) => Some(Err(e)),
                Ok(path) => {
                    if let Some(source_file) = SourceFile::from_path(path) {
                        let loc = source_file.loc().await;

                        println!(
                            "{:?}: {:?} - {:?}",
                            source_file.get_lang(),
                            source_file.get_path(),
                            loc
                        );

                        Some(Ok(loc))
                    } else {
                        None
                    }
                }
            };

            result
        });
    }

    let results = handles.join_all().await;
    let mut files_count: u32 = 0;
    let mut errors_count: u32 = 0;
    let mut loc_count: u32 = 0;
    for result in results {
        match result {
            Some(Ok(loc)) => {
                files_count += 1;
                loc_count += loc;
            }
            Some(Err(e)) => {
                eprintln!("{e:?}");
                errors_count += 1;
            }
            None => continue,
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
