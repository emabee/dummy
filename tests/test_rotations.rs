use std::{
    io::Write,
    path::{Path, PathBuf},
    time::Duration,
};

const LOG_FOLDER: &str = "logs";
const MOVED_FOLDER: &str = "logs/moved";
const OUTPUT_FILE: &str = "logs/test.txt";

#[test]
fn write_to_file_and_watch_for_events() {
    std::fs::remove_dir_all(LOG_FOLDER).ok();
    std::fs::create_dir_all(LOG_FOLDER).unwrap();
    std::fs::create_dir_all(MOVED_FOLDER).unwrap();

    let mut output_file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(OUTPUT_FILE)
        .unwrap();

    // write lines to output in a slow loop
    for i in 1..220 {
        std::thread::sleep(Duration::from_millis(10));
        writeln!(output_file, "YYY {} AAA", i).unwrap();

        match i {
            25 | 50 | 75 | 100 => {
                // rename the log file
                let mut target_name = PathBuf::from(MOVED_FOLDER);
                target_name.push(format!("file{}.txt", i));
                match std::fs::rename(OUTPUT_FILE, &target_name.clone()) {
                    Ok(()) => {
                        println!(
                            "Renamed the log file {:?} to {:?}",
                            &OUTPUT_FILE, &target_name
                        );
                        output_file = std::fs::OpenOptions::new()
                            .write(true)
                            .create(true)
                            .append(true)
                            .open(OUTPUT_FILE)
                            .unwrap();
                    }
                    Err(e) => {
                        panic!(
                            "Cannot rename the log file {:?} to {:?} due to {:?}",
                            &OUTPUT_FILE, &target_name, e
                        )
                    }
                }
            }
            125 | 150 | 175 | 200 => match std::fs::remove_file(OUTPUT_FILE) {
                Ok(()) => {
                    println!("Removed the log file {:?}", &OUTPUT_FILE);
                    std::mem::drop(&mut output_file);
                    std::thread::sleep(Duration::from_millis(1000));
                    output_file = std::fs::OpenOptions::new()
                        .write(true)
                        .create(true)
                        .append(true)
                        .open(OUTPUT_FILE)
                        .unwrap();
                }
                Err(e) => {
                    panic!(
                        "Cannot remove the log file {:?} due to {:?}",
                        &OUTPUT_FILE, e
                    );
                }
            },
            _ => {}
        }
    }

    // print a summary of the resulting files
    println!("");
    let mut counter = 0;
    for entry in std::fs::read_dir(LOG_FOLDER).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_file() {
            match std::fs::canonicalize(entry.path()) {
                Ok(p) => println!(
                    "{} with {} lines",
                    p.display(),
                    count_lines(&entry.path(), &mut counter)
                ),
                Err(_e) => println!("{} with {} lines", entry.path().display(), 0),
            }
        }
    }
    for entry in std::fs::read_dir(MOVED_FOLDER).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_file() {
            println!(
                "{} with {} lines",
                std::fs::canonicalize(entry.path()).unwrap().display(),
                count_lines(&entry.path(), &mut counter)
            );
        }
    }

    println!("Found {} lines in all files together", counter);
    println!(
        "Output file ends with ->{}<-",
        std::fs::read_to_string(OUTPUT_FILE)
            .unwrap()
            .lines()
            .last()
            .unwrap()
    );
}

fn count_lines(path: &Path, counter: &mut usize) -> usize {
    match std::fs::read_to_string(path) {
        Ok(s) => {
            let count = s.lines().filter(|line| line.contains("AAA")).count();
            *counter += count;
            count
        }
        Err(_e) => 0,
    }
}
