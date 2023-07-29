use std::fs::File;
use std::io::{self, BufRead};

#[derive(Copy, Clone)]
enum Pieces {
    PAWN, KNIGHT, BISHOP,
    ROOK, QUEEN, KING,
    NoPiece
}

#[derive(Copy, Clone)]
enum Player {
    WHITE, BLACK,
    NoPlayer
}

#[derive(Copy, Clone)]
struct Piece {
    color: Player,
    piece: Pieces,
}

const BOARD_SIZE : usize  = 8;
const WHITE      : Player = Player::WHITE;
const BLACK      : Player = Player::BLACK;

const EMPTY_SQUARE : Piece = Piece {
    color : Player::NoPlayer,
    piece : Pieces::NoPiece,
};


type ChessBoard = [[Piece; BOARD_SIZE]; BOARD_SIZE];

fn piece_to_char(piece: Piece) -> char {
    let res : char;
    match piece {
        Piece {color: Player::WHITE, piece: Pieces::PAWN}   => res = 'P',
        Piece {color: Player::WHITE, piece: Pieces::ROOK}   => res = 'R',
        Piece {color: Player::WHITE, piece: Pieces::KNIGHT} => res = 'N',
        Piece {color: Player::WHITE, piece: Pieces::BISHOP} => res = 'B',
        Piece {color: Player::WHITE, piece: Pieces::QUEEN}  => res = 'Q',
        Piece {color: Player::WHITE, piece: Pieces::KING}   => res = 'K',

        Piece {color: Player::BLACK, piece: Pieces::PAWN}   => res = 'p',
        Piece {color: Player::BLACK, piece: Pieces::ROOK}   => res = 'r',
        Piece {color: Player::BLACK, piece: Pieces::KNIGHT} => res = 'n',
        Piece {color: Player::BLACK, piece: Pieces::BISHOP} => res = 'b',
        Piece {color: Player::BLACK, piece: Pieces::QUEEN}  => res = 'q',
        Piece {color: Player::BLACK, piece: Pieces::KING}   => res = 'k',
        Piece {color:_, piece: _} => res = '.'
    }

    res
}

fn char_to_piece(ch : char) -> Piece {
    let mut res : Piece;

    match ch {
        'p' | 'P' => res = Piece{ 
                piece : Pieces::PAWN,
                color : Player::NoPlayer 
            },
        'r' | 'R' => res = Piece{ 
                piece : Pieces::ROOK,
                color : Player::NoPlayer 
            }, 
        'n' | 'N' => res =Piece{ 
                piece : Pieces::KNIGHT,
                color : Player::NoPlayer 
            }, 
        'b' | 'B' => res =Piece{ 
                piece : Pieces::BISHOP,
                color : Player::NoPlayer 
            }, 
        'q' | 'Q' => res =Piece{ 
                piece : Pieces::QUEEN,
                color : Player::NoPlayer 
            }, 
        'k' | 'K' => res =Piece{ 
                piece : Pieces::KING,
                color : Player::NoPlayer 
            }, 
        '.' | _   => return EMPTY_SQUARE,
    }

    if 'a' <= ch && ch <= 'z' {
        res.color = BLACK;
    } else if 'A' <= ch && ch <= 'Z' {
        res.color = WHITE;
    } else {
        return EMPTY_SQUARE;
    }

    res
}

fn print_board(board : &ChessBoard) {
    for i in 0 .. BOARD_SIZE {
        for j in 0 .. BOARD_SIZE {
            print!("{} ", piece_to_char(board[j][i]));
        }
        println!();
    }

}

fn init_board(file_name : &Path) ->  ChessBoard {
    let mut res : ChessBoard = [[Piece {color : Player::NoPlayer,
                                        piece : Pieces::NoPiece}; BOARD_SIZE]; BOARD_SIZE];

    if let Ok(lines) = read_lines(file_name) {
        for (y, line) in lines.enumerate() {
            if let Ok(str) = line {
                let mut x = 1;
                for ch in str.chars() {
                    res[BOARD_SIZE-x][y] = char_to_piece(ch);
                    x += 1;
                }
            }
        }
    }

    res
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
                                 where P: AsRef<Path>, {
    let file = File::open(filename)?;

    Ok(io::BufReader::new(file).lines())
}