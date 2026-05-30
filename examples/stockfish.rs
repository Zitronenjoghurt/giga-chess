use giga_chess::stockfish::command::{SfCommand, SfGo, SfPosition};
use giga_chess::stockfish::event::SfEvent;
use giga_chess::stockfish::StockfishManager;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

fn main() {
    let mut child = Command::new("stockfish")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to start stockfish");

    let mut stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();
    let reader = BufReader::new(stdout);

    let mut manager = StockfishManager::default();
    flush_commands(&mut manager, &mut stdin);

    let mut phase = Phase::WaitingForUci;

    for line in reader.lines() {
        let line = line.unwrap();
        let event = match manager.read_line(&line) {
            Ok(Some(event)) => event,
            Ok(None) => continue,
            Err(e) => {
                eprintln!("  !! {e:?}");
                continue;
            }
        };

        match (&phase, &event) {
            (Phase::WaitingForUci, SfEvent::Ok) => {
                println!("engine ready, setting up position...\n");

                manager.send(SfCommand::IsReady);
                manager.send(SfCommand::Position(SfPosition {
                    fen: None,
                    moves: vec![
                        "e2e4", "e7e5", "g1f3", "b8c6", "f1c4", "f8c5", "c2c3", "g8f6", "d2d4",
                    ]
                    .into_iter()
                    .map(String::from)
                    .collect(),
                }));

                phase = Phase::WaitingForReady;
            }
            (Phase::WaitingForReady, SfEvent::ReadyOk) => {
                println!("searching depth 18...\n");
                manager.send(SfCommand::Go(SfGo {
                    depth: Some(18),
                    ..Default::default()
                }));
                phase = Phase::Searching;
            }
            (Phase::Searching, SfEvent::Info(info)) => {
                if let (Some(depth), Some(score), pv) = (&info.depth, &info.score, &info.pv)
                    && !pv.is_empty()
                {
                    let score_str = format_score(score);
                    let pv_str: String = pv.iter().take(6).cloned().collect::<Vec<_>>().join(" ");
                    let dots = if pv.len() > 6 { " ..." } else { "" };
                    println!("  depth {depth:>2}  {score_str:>8}  {pv_str}{dots}");
                }
            }
            (Phase::Searching, SfEvent::BestMove { mv, ponder }) => {
                println!("\n  bestmove: {mv}");
                if let Some(p) = ponder {
                    println!("  ponder:   {p}");
                }

                println!("\n--- mate puzzle: white to mate in 2 ---\n");

                manager.send(SfCommand::Position(SfPosition {
                    fen: Some(
                        "2bqkbn1/2pppp2/np2N3/r3P1p1/p2N2B1/5Q2/PPPPPP1P/RNB1K2R w KQ - 0 1".into(),
                    ),
                    moves: vec![],
                }));
                manager.send(SfCommand::Go(SfGo {
                    depth: Some(10),
                    ..Default::default()
                }));

                phase = Phase::SolvingMate;
            }
            (Phase::SolvingMate, SfEvent::Info(info)) => {
                if let Some(score) = &info.score {
                    let score_str = format_score(score);
                    if let (Some(depth), pv) = (&info.depth, &info.pv)
                        && !pv.is_empty()
                    {
                        println!("  depth {depth:>2}  {score_str:>8}  {}", pv.join(" "));
                    }
                }
            }
            (Phase::SolvingMate, SfEvent::BestMove { mv, .. }) => {
                println!("\n  solution: {mv}");
                println!("\ndone!");

                manager.send(SfCommand::Quit);
                flush_commands(&mut manager, &mut stdin);
                break;
            }
            _ => {}
        }

        flush_commands(&mut manager, &mut stdin);
    }

    let _ = child.wait();
}

fn flush_commands(manager: &mut StockfishManager, stdin: &mut impl Write) {
    for cmd in manager.drain_commands() {
        writeln!(stdin, "{cmd}").unwrap();
        stdin.flush().unwrap();
    }
}

fn format_score(score: &giga_chess::stockfish::event::SfScore) -> String {
    use giga_chess::stockfish::event::SfScore;
    match score {
        SfScore::Cp { value, .. } => {
            let pawns = *value as f64 / 100.0;
            format!("{pawns:+.2}")
        }
        SfScore::Mate { value, .. } => format!("M{value}"),
    }
}

enum Phase {
    WaitingForUci,
    WaitingForReady,
    Searching,
    SolvingMate,
}
