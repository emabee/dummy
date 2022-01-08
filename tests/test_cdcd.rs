use std::io::Write;

const OUTPUT_FILE: &str = "test_cdcd.txt";

#[test]
fn main() {
    let mut output_file;
    // std::thread::sleep(Duration::from_millis(10));
    output_file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(OUTPUT_FILE)
        .unwrap();
    writeln!(output_file, "sdlsakjdpuwqeksadlsakd").expect("first write failed");

    std::fs::remove_file(OUTPUT_FILE).expect("first delete failed");

    output_file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(OUTPUT_FILE)
        .expect("second open failed");

    writeln!(output_file, "sdlsakjdpuwqeksadlsakd 1").expect("second write failed");

    std::fs::remove_file(OUTPUT_FILE).expect("second delete failed");

    output_file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(OUTPUT_FILE)
        .expect("second open failed");

    writeln!(output_file, "sdlsakjdpuwqeksadlsakd 2").expect("second write failed");

    std::fs::remove_file(OUTPUT_FILE).expect("second delete failed");

    output_file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(OUTPUT_FILE)
        .expect("second open failed");

    writeln!(output_file, "sdlsakjdpuwqeksadlsakd 3").expect("second write failed");
}
