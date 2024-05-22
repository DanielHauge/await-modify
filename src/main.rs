use notify::{self, Event};
use notify::{RecursiveMode, Watcher};
use std::env::args;
use std::path::PathBuf;
use std::process::exit;

fn main() -> Result<(), notify::Error> {
    // Check first argument is a file
    let file = match args().nth(1) {
        Some(file) => PathBuf::from(file),
        None => panic!("Please provide a file to watch"),
    };
    if !file.exists() {
        panic!("Please provide a file or directory to watch");
    }

    println!("Awaiting changes to {:?} ...", file);

    let mut watcher = notify::recommended_watcher(|res: Result<_, _>| match res {
        Ok(Event { kind: k, .. }) => {
            if k.is_modify() {
                exit(0)
            }
        }
        Err(e) => println!("watch error: {:?}", e),
    })?;

    watcher.watch(&file, RecursiveMode::Recursive)?;

    // Block forever
    loop {
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert!(true)
    }
}
