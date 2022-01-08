use std::fs::File;
use std::io::Write;

const OUTPUT_FILE: &str = "test_cdcd.txt";
const DUMMY_FILE: &str = "test_cdcd_dummy.txt";

#[test]
fn main() {
    let mut output_file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(OUTPUT_FILE)
        .unwrap();
    writeln!(output_file, "sdlsakjdpuwqeksadlsakd 1").expect("first write failed");

    std::fs::remove_file(OUTPUT_FILE).expect("first delete failed");

    reopen_file(&mut output_file, 2);
    writeln!(output_file, "sdlsakjdpuwqeksadlsakd 2").expect("second write failed");

    std::fs::remove_file(OUTPUT_FILE).expect("second delete failed");

    reopen_file(&mut output_file, 3);
    writeln!(output_file, "sdlsakjdpuwqeksadlsakd 3").expect("third write failed");

    std::fs::remove_file(OUTPUT_FILE).expect("third delete failed");

    reopen_file(&mut output_file, 4);
    writeln!(output_file, "sdlsakjdpuwqeksadlsakd 3").expect("fourth write failed");
}

fn reopen_file(output_file: &mut File, i: usize) -> Result<(), std::io::Error> {
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
}

fn open_file(p: &str) -> Result<std::fs::File, std::io::Error> {
    std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(p)
}
