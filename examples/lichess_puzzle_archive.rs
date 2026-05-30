use giga_chess::lichess::archive::LichessPuzzleArchive;
use giga_chess::lichess::parser::LichessPuzzleParser;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::io::BufReader;

const PUZZLE_PATH: &str = "lichess_db_puzzle.csv.zst";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let original_size = std::fs::metadata(PUZZLE_PATH)?.len();

    let bar = ProgressBar::new(original_size);
    bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "{bar:40.cyan/dim} {bytes}/{total_bytes} ({percent}%) {msg} [{elapsed_precise}]",
            )?
            .progress_chars("━╸─"),
    );

    let file = BufReader::new(File::open(PUZZLE_PATH)?);
    let parser = LichessPuzzleParser::new(bar.wrap_read(file))?;

    let archive = LichessPuzzleArchive::try_from_iter(parser)?;
    let archive_size = archive.as_bytes().len() as u64;
    let pct = archive_size as f64 / original_size as f64 * 100.0;

    bar.finish_with_message(format!(
        "done — {} → {} bytes ({pct:.1}% of original)",
        original_size, archive_size
    ));

    std::fs::write("lichess_puzzle_archive.bin", archive.as_bytes())?;
    Ok(())
}
