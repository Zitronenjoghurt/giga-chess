use giga_chess::lichess::parser::LichessPuzzleParser;
use giga_chess::prelude::{Color, Piece};
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

const PUZZLE_PATH: &str = "lichess_db_puzzle.csv.zst";
const PUZZLE_URL: &str = "https://database.lichess.org/lichess_db_puzzle.csv.zst";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(PUZZLE_PATH).exists() {
        println!("Downloading puzzle database...");
        let response = ureq::get(PUZZLE_URL).call()?;

        let total_bytes = response
            .headers()
            .get("content-length")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);

        let dl_bar = ProgressBar::new(total_bytes);
        dl_bar.set_style(
            ProgressStyle::default_bar()
                .template("{bar:40.cyan/dim} {bytes}/{total_bytes} ({bytes_per_sec}) [{eta}]")?
                .progress_chars("━╸─"),
        );

        let mut out = File::create(PUZZLE_PATH)?;
        std::io::copy(
            &mut dl_bar.wrap_read(response.into_body().into_reader()),
            &mut out,
        )?;
        dl_bar.finish_and_clear();
    }

    let file = BufReader::new(File::open(PUZZLE_PATH)?);
    let parser = LichessPuzzleParser::new(file)?;

    let parse_bar = ProgressBar::new_spinner();
    parse_bar.set_style(
        ProgressStyle::default_spinner().template("{spinner:.green} {msg} [{elapsed_precise}]")?,
    );

    let mut stats = Stats::default();
    let mut errors = 0usize;
    for (i, result) in parser.enumerate() {
        if i % 1000 == 0 {
            parse_bar.set_message(format!("{i} puzzles processed ({errors} errors)"));
            parse_bar.tick();
        }

        match result {
            Ok(entry) => {
                stats.count += 1;
                stats.ratings.push(entry.rating);
                stats.rating_deviation.push(entry.rating_deviation);
                stats.times_played.push(entry.times_played);
                stats.move_counts.push(entry.moves.len());

                for theme in entry.themes {
                    *stats.themes.entry(theme).or_default() += 1;
                }
                for tag in entry.opening_tags {
                    *stats.opening_tags.entry(tag).or_default() += 1;
                }
                for piece in Piece::ALL {
                    for color in Color::ALL {
                        stats
                            .piece_counts
                            .entry((piece, color))
                            .or_default()
                            .push(entry.pos.board.specific_piece_count(piece, color));
                    }
                }
            }
            Err(e) => {
                errors += 1;
                if errors <= 20 {
                    eprintln!("Error at row {i}: {e}");
                } else if errors == 21 {
                    eprintln!("...more errors were omitted")
                }
            }
        }
    }

    parse_bar.finish_with_message(format!("{} puzzles done ({errors} errors)", stats.count));

    print_num_stats("Rating", &mut stats.ratings);
    println!();
    print_num_stats("Rating Deviation", &mut stats.rating_deviation);
    println!();
    print_num_stats("Times Played", &mut stats.times_played);
    println!();
    print_num_stats("Moves per Puzzle", &mut stats.move_counts);
    println!();
    print_freq("Themes", &stats.themes, 100);
    println!();
    print_freq("Opening Tags", &stats.opening_tags, 30);
    println!();

    println!("Piece Counts:");
    for color in Color::ALL {
        println!("  {color:?}:");
        for piece in Piece::ALL {
            if let Some(values) = stats.piece_counts.get_mut(&(piece, color)) {
                print_num_stats_u8(&format!("{piece:?}"), values);
            }
        }
    }

    Ok(())
}

#[derive(Default)]
struct Stats {
    count: usize,
    ratings: Vec<usize>,
    rating_deviation: Vec<usize>,
    times_played: Vec<usize>,
    move_counts: Vec<usize>,
    themes: HashMap<String, usize>,
    opening_tags: HashMap<String, usize>,
    piece_counts: HashMap<(Piece, Color), Vec<u8>>,
}

fn print_num_stats(name: &str, values: &mut [usize]) {
    values.sort_unstable();
    let len = values.len();
    let sum: usize = values.iter().sum();
    println!("{name}:");
    println!("  min: {}", values[0]);
    println!("  max: {}", values[len - 1]);
    println!("  avg: {:.1}", sum as f64 / len as f64);
    println!("  med: {}", values[len / 2]);
}

fn print_num_stats_u8(name: &str, values: &mut [u8]) {
    values.sort_unstable();
    let len = values.len();
    let sum: usize = values.iter().map(|&v| v as usize).sum();
    println!(
        "  {name}: min={} max={} avg={:.2} med={}",
        values[0],
        values[len - 1],
        sum as f64 / len as f64,
        values[len / 2]
    );
}

fn print_freq(name: &str, map: &HashMap<String, usize>, top_n: usize) {
    let mut entries: Vec<_> = map.iter().collect();
    entries.sort_by(|a, b| b.1.cmp(a.1));
    println!("{name} ({} unique):", entries.len());
    for (key, count) in entries.iter().take(top_n) {
        let pct = **count as f64 / map.values().sum::<usize>() as f64 * 100.0;
        println!("  {key}: {count} ({pct:.1}%)");
    }
    if entries.len() > top_n {
        println!("  ... and {} more", entries.len() - top_n);
    }
}
