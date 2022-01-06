use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    io::Write,
    path::{Path, PathBuf},
    sync::mpsc,
    time::Duration,
};

const COUNT: u8 = 2;

const LOG_FOLDER: &str = "logs";
const MOVED_FOLDER: &str = "logs/moved";
const OUTPUT_FILE: &str = "logs/test.txt";

#[test]
fn test_rotations() {
    if let Some(value) = dispatch(COUNT) {
        work(value)
    }
}

// launch child process from same executable and sets there an additional environment variable
// or finds this environment variable and returns its value
pub fn dispatch(count: u8) -> Option<u8> {
    const CTRL_INDEX: &str = "CTRL_INDEX";
    match std::env::var(CTRL_INDEX) {
        Err(_) => {
            println!("dispatcher");
            let progname = std::env::args().next().unwrap();
            let mut children = vec![];
            for value in 0..count {
                let mut command = std::process::Command::new(progname.clone());
                children.push(
                    command
                        .arg("--nocapture")
                        .env(CTRL_INDEX, value.to_string())
                        .spawn()
                        .unwrap(),
                );
            }
            for (value, mut child) in children.into_iter().enumerate() {
                let exit_status = child.wait().unwrap();
                assert!(exit_status.success(), "executor {} failed", value);
            }
            None
        }
        Ok(value) => {
            println!("executor {}", value);
            Some(value.parse().unwrap())
        }
    }
}

fn work(value: u8) {
    match value {
        0 => {
            write_to_file_and_watch_for_events();
        }
        1 => {
            mess_with_file();
        }
        COUNT..=u8::MAX => {
            unreachable!("dtrtgfg")
        }
    };
}

fn write_to_file_and_watch_for_events() {
    std::fs::remove_dir_all(LOG_FOLDER).ok();
    std::fs::create_dir_all(LOG_FOLDER).unwrap();
    std::fs::create_dir_all(MOVED_FOLDER).unwrap();
    let watched_folder = std::fs::canonicalize(&LOG_FOLDER).unwrap();

    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_millis(50)).unwrap();
    watcher
        .watch(&watched_folder, RecursiveMode::NonRecursive)
        .unwrap();
    println!("Watcher set up for {}", watched_folder.display());

    let mut output_file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(OUTPUT_FILE)
        .unwrap();

    // write lines to output in a slow loop
    for i in 1..400 {
        std::thread::sleep(Duration::from_millis(10));
        writeln!(output_file, "YYY {} AAA", i).unwrap();

        loop {
            match rx.try_recv() {
                Ok(debounced_event) => {
                    // println!("  Event detected ({:?})", debounced_event);
                    match debounced_event {
                        DebouncedEvent::NoticeRemove(ref _path)
                        | DebouncedEvent::Remove(ref _path)
                        | DebouncedEvent::Rename(ref _path, _) => {
                            println!("  Reopening the file! (in loop {})", i);
                            output_file = std::fs::OpenOptions::new()
                                .write(true)
                                .create(true)
                                .open(OUTPUT_FILE)
                                .unwrap();
                        }
                        _event => {}
                    }
                }
                Err(mpsc::TryRecvError::Empty) => {
                    break;
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    println!("  Error while watching the log file (in loop {})", i);
                    break;
                }
            }
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

fn mess_with_file() {
    // rename the log file
    for i in 0..3 {
        std::thread::sleep(Duration::from_millis(400));

        let mut target_name = PathBuf::from(MOVED_FOLDER);
        target_name.push(format!("file{}.txt", i));
        println!(
            "Renaming the log file {:?} to {:?}",
            &OUTPUT_FILE, &target_name,
        );
        match std::fs::rename(OUTPUT_FILE, &target_name.clone()) {
            Ok(()) => {}
            Err(e) => {
                println!(
                    "Cannot rename the log file {:?} to {:?} due to {:?}",
                    &OUTPUT_FILE, &target_name, e
                )
            }
        }
    }

    // remove the log file
    for _ in 0..3 {
        std::thread::sleep(Duration::from_millis(400));
        match std::fs::remove_file(OUTPUT_FILE) {
            Ok(()) => {
                println!("Removed the log file {:?}", &OUTPUT_FILE);
            }
            Err(e) => {
                // should be panic - is defused because test doesn't work properly on linux
                println!(
                    "Cannot remove the log file {:?} due to {:?}",
                    &OUTPUT_FILE, e
                );
            }
        }
    }
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
