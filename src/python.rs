use crate::TAB;
use crate::show_board;
use std::io::StdoutLock;
use std::io::Write;

use chess::MoveGen;
use chess::{Game, GameResult};

fn recursive_generate(current_depth: usize, target_depth: usize, out: &mut StdoutLock, game: Game) {
    // 2 lots of indentation per recursion
    let indent = TAB.repeat(current_depth);

    match game.result() {
        None => {}
        Some(result) => {
            match result {
                GameResult::WhiteCheckmates => {
                    writeln!(
                        out,
                        "{}print(\"Game over. White wins by checkmate.\")",
                        indent,
                    )
                    .unwrap();
                    writeln!(out, "{}exit(0)", indent,).unwrap();
                }
                GameResult::BlackCheckmates => {
                    writeln!(
                        out,
                        "{}print(\"Game over. Black wins by checkmate.\")",
                        indent,
                    )
                    .unwrap();
                    writeln!(out, "{}exit(0)", indent,).unwrap();
                }
                GameResult::Stalemate => {
                    writeln!(out, "{}print(\"Game over. Draw by stalemate.\")", indent).unwrap();
                    writeln!(out, "{}exit(0)", indent,).unwrap();
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
            "{}print(\"Game over. Draw by repetition.\")",
            indent
        )
        .unwrap();
        writeln!(out, "{}exit(0)", indent,).unwrap();
        return;
    }

    if current_depth == target_depth {
        writeln!(out, "{}raise NotImplementedError(\"Moves for this position have not been generated yet! Please open an issue on the project's GitHub page.\")", indent).unwrap();
        return;
    }

    writeln!(
        out,
        "{}choice = input(\"{:?} to move. {}. \")",
        indent,
        game.side_to_move(),
        (current_depth / 2) + 1
    )
    .unwrap();
    let mut first_if = true;
    for (idx, chess_move) in MoveGen::new_legal(&game.current_position()).enumerate() {
        // Progress report only for current_depth 0
        if current_depth == 0 {
            eprint!("{}/20\r", idx + 1);
        }

        let iffy = if first_if { "if" } else { "elif" };
        first_if = false;

        writeln!(out, "{}{} choice == \"{}\":", indent, iffy, chess_move).unwrap();
        let mut recursed_game = game.clone();
        recursed_game.make_move(chess_move);
        show_board(
            out,
            &recursed_game.current_position(),
            format!("{indent}    ").as_str(),
            "print(",
            ")",
        );
        recursive_generate(current_depth + 1, target_depth, out, recursed_game);
    }
    writeln!(out, "{}else:", indent).unwrap();
    writeln!(out, "{}    raise ValueError(f\"Invalid move: {{choice}}. Moves must be entered in the form a1b2.\")", indent).unwrap();
}

pub fn write_python_program(depth: usize, out: &mut StdoutLock) {
    let game = Game::new();

    writeln!(out, "print(\"Welcome to chess!\")").unwrap();
    writeln!(out, "print()").unwrap();
    writeln!(out).unwrap();
    show_board(out, &game.current_position(), "", "print(", ")");

    recursive_generate(0, depth, out, game);
}
