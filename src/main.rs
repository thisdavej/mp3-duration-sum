use std::path::{Path, PathBuf};

use rayon::prelude::*;
use structopt::StructOpt;

use human_time::ToHumanTimeString;
use std::time::Duration;

#[derive(StructOpt)]
struct Cli {
    /// Path to directory containing .mp3 files
    #[structopt(parse(from_os_str))]
    path: PathBuf,

    /// By default, the duration is displayed in milliseconds. Use this option to display duration in a human-readable format (e.g., "3h,12m,5s" for 3 hours, 12 minutes, and 5 seconds).
    #[structopt(short = "u", long = "human-time")]
    human_time: bool,
}

fn main() {
    let args = Cli::from_args();
    let path = Path::new(&args.path);

    let entries: Vec<PathBuf> = path
        .read_dir()
        .expect("path needs to exist")
        .map(|entry| entry.unwrap().path())
        .collect();

    let total: u64 = entries
        .par_iter()
        .map(|path| {
            let ext = path.extension();
            if ext.is_some() && ext.unwrap() == "mp3" {
                let duration = mp3_duration::from_path(&path).unwrap();
                duration.as_secs() * 1000 + duration.subsec_millis() as u64
            } else {
                0
            }
        })
        .sum();

    if args.human_time {
        let human_time_total = Duration::from_millis(total).to_human_time_string();
        println!("{}", human_time_total)
    } else {
        println!("{}", total);
    }
}
