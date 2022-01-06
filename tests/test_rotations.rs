use dummy::start_external_rotate_watcher;
use std::{
    io::Write,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

#[test]
fn test_rotations() {
    let log_dir = PathBuf::from("logs");
    std::fs::remove_dir_all(log_dir.clone()).ok();

    let mut mv_dir = log_dir.clone();
    mv_dir.push("moved");
    std::fs::create_dir_all(mv_dir.clone()).unwrap();
    let mv_dir2 = mv_dir.clone();
    let trigger = Arc::new(AtomicBool::new(false));
    start_external_rotate_watcher(&log_dir, Arc::clone(&trigger));

    let mut output = log_dir.clone();
    output.push("test.txt");

    let output_clone = output.clone();

    // start a thread that messes with the output file
    let worker_handle = std::thread::Builder::new()
        .name("file rotator".to_string())
        .spawn(move || {
            for i in 0..4 {
                std::thread::sleep(std::time::Duration::from_millis(400));
                // rotate the log file
                let mut target_name = mv_dir2.clone();
                target_name.push(format!("file{}.txt", i));
                println!(
                    "Renaming the log file {:?} to {:?}",
                    &output_clone, &target_name,
                );
                match std::fs::rename(output_clone.clone(), &target_name.clone()) {
                    Ok(()) => {}
                    Err(e) => {
                        println!(
                            "Cannot rename log file {:?} to {:?} due to {:?}",
                            &output_clone, &target_name, e
                        )
                    }
                }
            }
            for _ in 0..2 {
                std::thread::sleep(std::time::Duration::from_millis(400));
                match std::fs::remove_file(output_clone.clone()) {
                    Ok(()) => {
                        println!("Removed the log file {:?}", &output_clone,)
                    }
                    Err(e) => {
                        // should be panic - is defused because test doesn't work properly on linux
                        println!(
                            "Cannot remove the log file {:?} due to {:?}",
                            &output_clone, e
                        )
                    }
                }
            }
        })
        .unwrap();

    let mut output_file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(output.clone())
        .unwrap();

    // write lines to output in a slow loop
    for i in 1..400 {
        std::thread::sleep(std::time::Duration::from_millis(10));
        writeln!(output_file, "YYY {} AAA", i).unwrap();

        if trigger.swap(false, Ordering::Relaxed) {
            println!(
                "    Trigger was pulled, reopening the file! (in loop {})",
                i
            );
            output_file = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(output.clone())
                .unwrap();
        }
    }

    worker_handle.join().unwrap();

    // print the files we created
    println!("");
    let mut counter = 0;
    for entry in std::fs::read_dir(log_dir).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_file() {
            println!(
                "{} with {} lines",
                std::fs::canonicalize(entry.path()).unwrap().display(),
                count_lines(&entry.path(), &mut counter)
            );
        }
    }
    for entry in std::fs::read_dir(mv_dir).unwrap() {
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
        std::fs::read_to_string(output)
            .unwrap()
            .lines()
            .last()
            .unwrap()
    );

    assert!(false, "Ending with error to see the output");
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
