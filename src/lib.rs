use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::{
    ops::Deref,
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

pub fn start_external_rotate_watcher(folder: &Path, trigger: Arc<AtomicBool>) {
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = watcher(tx, std::time::Duration::from_millis(50)).unwrap();
    std::fs::create_dir_all(folder).unwrap();
    let watched_folder = std::fs::canonicalize(folder).unwrap();
    watcher
        .watch(&watched_folder, RecursiveMode::NonRecursive)
        .unwrap();

    // in a separate thread, wait for events for the log file
    let builder =
        std::thread::Builder::new().name("flexi_logger-external_rotate_watcher".to_string());
    #[cfg(not(feature = "dont_minimize_extra_stacks"))]
    let builder = builder.stack_size(128 * 1024);
    builder
        .spawn(move || {
            let _keep_watcher_alive = watcher;
            loop {
                match rx.recv() {
                    Ok(debounced_event) => {
                        match debounced_event {
                            DebouncedEvent::NoticeRemove(ref _path)
                            | DebouncedEvent::Remove(ref _path)
                            | DebouncedEvent::Rename(ref _path, _) => {
                                // if path.canonicalize().map(|x| x == logfile).unwrap_or(false) {
                                // trigger a restart of the state with append mode
                                trigger.deref().store(true, Ordering::Relaxed);
                                // }
                            }
                            _event => {}
                        }
                    }
                    Err(e) => {
                        println!("error while watching the log file, caused by {}", &e,);
                    }
                }
            }
        })
        .unwrap();
}
