use std::fs::File;
use std::io::Write;

const LOG_FOLDER: &str = "logs";
const OUTPUT_FILE: &str = "logs/test_delete.txt";

#[test]
fn main() {
    std::fs::remove_dir_all(LOG_FOLDER).ok();
    std::fs::create_dir_all(LOG_FOLDER).unwrap();

    let mut output_file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(OUTPUT_FILE)
        .unwrap();
    writeln!(output_file, "sdlsakjdpuwqeksadlsakd 1").expect("first write failed");

    std::fs::remove_file(OUTPUT_FILE).expect("first delete failed");

    reopen_file(&mut output_file).unwrap();
    writeln!(output_file, "sdlsakjdpuwqeksadlsakd 2").expect("second write failed");

    std::fs::remove_file(OUTPUT_FILE).expect("second delete failed");

    reopen_file(&mut output_file).unwrap();
    writeln!(output_file, "sdlsakjdpuwqeksadlsakd 3").expect("third write failed");

    std::fs::remove_file(OUTPUT_FILE).expect("third delete failed");

    reopen_file(&mut output_file).unwrap();
    writeln!(output_file, "sdlsakjdpuwqeksadlsakd 3").expect("fourth write failed");
}

fn reopen_file(output_file: &mut File) -> Result<(), std::io::Error> {
    const DUMMY_FILE: &str = "test_delete_dummy.txt";
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
