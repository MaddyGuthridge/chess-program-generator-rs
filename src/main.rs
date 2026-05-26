use std::io::StdoutLock;
use std::io::Write;
use std::io::stdout;
use std::time::Instant;

use chess::Color;
use chess::Game;
use chess::GameResult;
use chess::MoveGen;
use chess::{Board, File, Piece, Rank, Square};
use clap::Parser;
use humantime::format_duration;

const TAB: &str = "    ";
const BG_WHITE: char = '◽';
const BG_BLACK: char = '◾';

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of times to greet
    #[arg(short, long)]
    depth: usize,
}

#[inline(always)]
fn piece_ch(piece: Piece, color: Color) -> char {
    match color {
        Color::White => match piece {
            Piece::Queen => '♕',
            Piece::King => '♔',
            Piece::Rook => '♖',
            Piece::Bishop => '♗',
            Piece::Knight => '♘',
            Piece::Pawn => '♙',
        },
        Color::Black => match piece {
            Piece::Queen => '♛',
            Piece::King => '♚',
            Piece::Rook => '♜',
            Piece::Bishop => '♝',
            Piece::Knight => '♞',
            Piece::Pawn => '♟',
        },
    }
}

#[inline(always)]
fn show_board(out: &mut StdoutLock, board: &Board, indent: &str) {
    for r in (0..8).rev() {
        write!(out, "{}println!(\"", indent).unwrap();
        for c in 0..8 {
            let bg = if (r + c) % 2 == 0 { BG_WHITE } else { BG_BLACK };
            let square = Square::make_square(Rank::from_index(r), File::from_index(c));
            if let Some(piece) = board.piece_on(square) {
                write!(out, "{} ", piece_ch(piece, board.color_on(square).unwrap())).unwrap();
            } else {
                write!(out, "{}", bg).unwrap();
            }
        }
        writeln!(out, "\");").unwrap();
    }
}

fn recursive_generate(current_depth: usize, target_depth: usize, out: &mut StdoutLock, game: Game) {
    // 2 lots of indentation per recursion, plus one for the outer main function
    let indent = TAB.repeat(current_depth * 2 + 1);

    match game.result() {
        None => {}
        Some(result) => {
            match result {
                GameResult::WhiteCheckmates => {
                    writeln!(
                        out,
                        "{}println!(\"Game over. White wins by checkmate.\")",
                        indent,
                    )
                    .unwrap();
                }
                GameResult::BlackCheckmates => {
                    writeln!(
                        out,
                        "{}println!(\"Game over. Black wins by checkmate.\")",
                        indent,
                    )
                    .unwrap();
                }
                GameResult::Stalemate => {
                    writeln!(out, "{}println!(\"Game over. Draw by stalemate.\")", indent).unwrap();
                }
                _ => {
                    panic!("bruh moment");
                }
            };
            return;
        }
    }

    if game.can_declare_draw() {
        writeln!(
            out,
            // They ain't reaching a depth of 50
            "{}println!(\"Game over. Draw by repetition.\")",
            indent
        )
        .unwrap();
        return;
    }

    if current_depth == target_depth {
        writeln!(out, "{}todo!(\"Moves for this position have not been generated yet! Please open an issue on the project's GitHub page.\");", indent).unwrap();
        return;
    }

    writeln!(
        out,
        "{}print!(\"{:?} to move. {}. \");",
        indent,
        game.side_to_move(),
        (current_depth / 2) + 1
    )
    .unwrap();
    writeln!(out, "{}io::stdout().flush().unwrap();", indent).unwrap();
    writeln!(out, "{}choice.clear();", indent).unwrap();
    writeln!(out, "{}stdin.read_line(&mut choice).unwrap();", indent).unwrap();
    writeln!(out, "{}match choice.trim_end() {{", indent).unwrap();
    for (idx, chess_move) in MoveGen::new_legal(&game.current_position()).enumerate() {
        // Progress report only for current_depth 0
        if current_depth == 0 {
            eprint!("{}/20\r", idx);
        }

        writeln!(out, "{}   \"{}\" => {{", indent, chess_move).unwrap();
        let mut recursed_game = game.clone();
        recursed_game.make_move(chess_move);
        recursive_generate(current_depth + 1, target_depth, out, recursed_game);
        writeln!(out, "{}    }}", indent).unwrap();
    }
    writeln!(
        out,
        "{}    _ => panic!(\"Invalid move: {{}}. Moves must be entered in the form a1b2.\", choice.trim_end())",
        indent
    )
    .unwrap();
    writeln!(out, "{}}}", indent).unwrap();
}

fn main() {
    let depth = Args::parse().depth;
    let mut out = stdout().lock();

    let t_start = Instant::now();

    eprintln!("Generating to a depth of {}", depth);

    let game = Game::new();

    writeln!(out, "use std::io;").unwrap();
    writeln!(out, "use std::io::Write;").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "fn main() {{").unwrap();
    writeln!(out, "    let mut choice = String::new();").unwrap();
    writeln!(out, "    let stdin = io::stdin();").unwrap();
    writeln!(out, "    println!(\"Welcome to chess!\");").unwrap();
    writeln!(out, "    println!();").unwrap();
    writeln!(out).unwrap();
    show_board(&mut out, &game.current_position(), TAB);

    recursive_generate(0, depth, &mut out, game);

    writeln!(out, "}}").unwrap();
    
    let t_end = Instant::now();

    eprintln!("\nDone in {}", format_duration(t_end - t_start));
}
