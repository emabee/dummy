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

    output_file = {
        match std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(OUTPUT_FILE)
        {
            Ok(file) => file,
            Err(e) => match e.kind() {
                std::io::ErrorKind::PermissionDenied => {
                    println!("Looks like we're running in a non-standard env, like github's windows environment; giving up without error...");
                    return;
                }
                _ => panic!("second open failed with {:?}", e),
            },
        }
    };

    writeln!(output_file, "sdlsakjdpuwqeksadlsakd 1").expect("second write failed");

    std::fs::remove_file(OUTPUT_FILE).expect("second delete failed");

    output_file = {
        match std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(OUTPUT_FILE)
        {
            Ok(file) => file,
            Err(e) => match e.kind() {
                std::io::ErrorKind::PermissionDenied => {
                    println!("Looks like we're running in a non-standard env, like github's windows environment; giving up without error...");
                    return;
                }
                _ => panic!("third open failed with {:?}", e),
            },
        }
    };

    writeln!(output_file, "sdlsakjdpuwqeksadlsakd 2").expect("third write failed");

    std::fs::remove_file(OUTPUT_FILE).expect("third delete failed");

    output_file = {
        match std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(OUTPUT_FILE)
        {
            Ok(file) => file,
            Err(e) => match e.kind() {
                std::io::ErrorKind::PermissionDenied => {
                    println!("Looks like we're running in a non-standard env, like github's windows environment; giving up without error...");
                    return;
                }
                _ => panic!("fourth open failed with {:?}", e),
            },
        }
    };

    writeln!(output_file, "sdlsakjdpuwqeksadlsakd 3").expect("fourth write failed");
}
