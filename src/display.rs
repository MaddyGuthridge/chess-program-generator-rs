use std::io::StdoutLock;
use std::io::Write;

use chess::File;
use chess::Rank;
use chess::{Board, Color, Piece, Square};

const BG_WHITE: char = '◽';
const BG_BLACK: char = '◾';

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
pub fn show_board(out: &mut StdoutLock, board: &Board, indent: &str, opening: &str, closing: &str) {
    for r in (0..8).rev() {
        write!(out, "{}{}\"", indent, opening).unwrap();
        for c in 0..8 {
            let bg = if (r + c) % 2 == 0 { BG_WHITE } else { BG_BLACK };
            let square = Square::make_square(Rank::from_index(r), File::from_index(c));
            if let Some(piece) = board.piece_on(square) {
                write!(out, "{} ", piece_ch(piece, board.color_on(square).unwrap())).unwrap();
            } else {
                write!(out, "{}", bg).unwrap();
            }
        }
        writeln!(out, "\"{}", closing).unwrap();
    }
}
