use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    time::Duration,
};

const LOG_FOLDER: &str = "logs";
const MOVED_FOLDER: &str = "logs/moved";
const OUTPUT_FILE: &str = "logs/test_rename.txt";

#[test]
fn test_rename() {
    std::fs::remove_dir_all(LOG_FOLDER).ok();
    std::fs::create_dir_all(LOG_FOLDER).unwrap();
    std::fs::create_dir_all(MOVED_FOLDER).unwrap();

    let mut output_file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(OUTPUT_FILE)
        .unwrap();

    // write lines to output in a slow loop
    for i in 1..120 {
        std::thread::sleep(Duration::from_millis(10));
        writeln!(output_file, "YYY {} AAA", i).unwrap();

        // rename the log file
        match i {
            25 | 50 | 75 | 100 => {
                let mut target_name = PathBuf::from(MOVED_FOLDER);
                target_name.push(format!("file{}.txt", i));
                std::fs::rename(OUTPUT_FILE, &target_name.clone()).unwrap();
                println!(
                    "Renamed the log file {:?} to {:?}",
                    &OUTPUT_FILE, &target_name
                );
                reopen_file(&mut output_file).unwrap();
            }
            _ => {}
        }
    }

    let mut counter = 0;
    for entry in std::fs::read_dir(LOG_FOLDER).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_file() {
            println!(
                "{} with {} lines",
                std::fs::canonicalize(entry.path()).unwrap().display(),
                count_lines(&entry.path(), &mut counter)
            );
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

    assert_eq!(119, counter,);
    assert_eq!(
        "YYY 119 AAA",
        std::fs::read_to_string(OUTPUT_FILE)
            .unwrap()
            .lines()
            .last()
            .unwrap(),
    );
}

fn reopen_file(output_file: &mut File) -> Result<(), std::io::Error> {
    const DUMMY_FILE: &str = "test_rename_dummy.txt";
    match open_file(OUTPUT_FILE) {
        Ok(file) => {
            // proved to work on standard windows, linux, mac
            *output_file = file;
        }
        Err(_unexpected_error) => {
            // there are environments, like github's windows container,
            // where this extra step helps to overcome the _unexpected_error
            *output_file = open_file(DUMMY_FILE)?;
            std::fs::remove_file(DUMMY_FILE)?;
            *output_file = open_file(OUTPUT_FILE)?;
        }
    }
    Ok(())
}
fn open_file(p: &str) -> Result<std::fs::File, std::io::Error> {
    std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(p)
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
