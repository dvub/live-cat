use clap::Parser;
use notify::{Event, RecursiveMode, Watcher};
use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
    time::Duration,
};

/// Print the contents of a file to the standard output, similarly to the "cat" command on linux.
/// This program also watches the file and prints the file again whenever the file changes.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the desired file
    #[arg(short, long)]
    path: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let target_path = Args::parse().path;
    // TODO:
    // fix this dumbness
    let cloned_path = target_path.clone();
    output_file_content(&target_path);

    let file_change_handler = move |res: std::result::Result<Event, notify::Error>| match res {
        Ok(event) => match event.kind {
            notify::EventKind::Modify(_) => {
                output_file_content(&cloned_path);
            }
            notify::EventKind::Remove(_) => todo!(),
            _ => {}
        },
        Err(e) => println!("watch error: {:?}", e),
    };

    let mut watcher = notify::recommended_watcher(file_change_handler)?;
    watcher.watch(&target_path, RecursiveMode::Recursive)?;
    // TODO:
    // fix this
    loop {
        std::thread::sleep(Duration::from_secs(100));
    }
}

fn output_file_content(path: &Path) {
    // clear command doesn't exist on windows :(
    if !cfg!(target_os = "windows") {
        std::process::Command::new("clear")
            .status()
            .expect("Error running command");
    }

    let file_contents = read_to_string(path).expect("Error reading target file");
    println!("{file_contents}");
}
