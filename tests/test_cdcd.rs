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

fn reopen_file(output_file: &mut File, i: usize) {
    match std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(OUTPUT_FILE)
    {
        Ok(file) => {
            *output_file = file;
        }
        Err(e) => match e.kind() {
            std::io::ErrorKind::PermissionDenied => {
                *output_file = std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .append(true)
                    .open(DUMMY_FILE)
                    .unwrap();
                // std::thread::sleep(std::time::Duration::from_millis(5000));
                std::fs::remove_file(DUMMY_FILE).expect("second delete failed");

                match std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .append(true)
                    .open(OUTPUT_FILE)
                {
                    Ok(file) => {
                        *output_file = file;
                    }
                    Err(_e) => {
                        println!(
                            "Looks like we're running in a non-standard env, \
                             like github's fake windows, \
                             and obviously we're not able to overcome this somewhat weird issue; \
                            giving up without error..."
                        );
                        return;
                    }
                }
            }
            _ => panic!("{}. open failed with {:?}", i, e),
        },
    }
}
