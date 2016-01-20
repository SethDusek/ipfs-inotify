extern crate inotify;
extern crate getopts;

use std::process::Command;
use inotify::INotify;
use inotify::ffi::IN_CREATE; // we only need in_create
use std::path::Path;
use getopts::Options;
use std::env;
use std::ffi::OsStr;

fn ipfs_add<T: AsRef<OsStr>>(filepath: T) -> Result<String,std::string::FromUtf8Error> {
    let output = Command::new("ipfs")
                .arg("add")
                .arg("-n")
                .arg("-q")
                .arg(filepath.as_ref())
                .output()
                .unwrap();
    String::from_utf8(output.stdout)
}

fn main() {
    let mut dirstr: String = String::from("/tmp");
    let mut watcher = INotify::init().unwrap();
    let mut opts = Options::new();
    opts.optflag("a","all","ipfs add all files instead of just ones that are ipfs hashes");
    opts.optopt("p","path","Path to watch. Default: /tmp","/tmp");
    let args: Vec<String> = env::args().collect();
    let args = opts.parse(&args[1..]).unwrap_or_else(|e| { panic!("{}",e); });
    let argpath = args.opt_str("path");
    if let Some(path) = argpath {
        dirstr = String::from(path);
    }
    let dirpath = Path::new(&dirstr);
    watcher.add_watch(&dirpath, IN_CREATE).unwrap();
    let mut watchall: bool = false;
    if args.opt_present("all") { watchall = true; }

    loop {
        let events = watcher.wait_for_events().unwrap();
        for event in events.iter() {
            if ! event.is_dir() {
                let hashpath = dirpath.join(&event.name);
                if watchall {
                    println!("{}",hashpath.display()); 
                    let hash = ipfs_add(hashpath).unwrap_or_else(|_| { panic!("failed to generate a hash"); });
                    println!("{}",hash);
                }

                else if !watchall && &event.name.starts_with("Qm")==&true {
                    println!("{}",hashpath.display()); 
                    let hash = ipfs_add(hashpath).unwrap_or_else(|_| { panic!("failed to generate a hash"); });
                    println!("{}",hash);

                }
            }
        }
    }
}
