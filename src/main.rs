extern crate inotify;

use std::process::Command;
use inotify::INotify;
use inotify::ffi::IN_CREATE; // we only need in_create
use std::path::Path;
use std::env;

fn main() {
    let mut watcher = INotify::init().unwrap();
    let args: Vec<String> = env::args().collect();
    let dirstr = &args[1];
    let dirpath: &Path = Path::new(&dirstr);
    watcher.add_watch(&dirpath, IN_CREATE).unwrap();
    loop {
        let events = watcher.wait_for_events().unwrap();

        for event in events.iter() {
            if ! event.is_dir() && event.name.starts_with("Qm") {
                let output = Command::new("ipfs")
                    .arg("add")
                    .arg("-q")
                    .arg(dirpath.join(&event.name))
                    .output()
                    .unwrap();
                println!("{}",String::from_utf8_lossy(&output.stdout));
            }
        }
    }
}
