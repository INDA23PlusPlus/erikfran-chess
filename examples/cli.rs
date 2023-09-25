extern crate chess;
use chess::*;
use chess::util::*;
use std::io;
use chess::Move::Castle;

fn cord_to_square(cord: &str) -> Square {
    let mut chars = cord.chars();
    let file = match chars.next().unwrap() {
        'a' => File::A,
        'b' => File::B,
        'c' => File::C,
        'd' => File::D,
        'e' => File::E,
        'f' => File::F,
        'g' => File::G,
        'h' => File::H,
        _ => {
            panic!("Invalid file")
        },
    };
    let rank = match chars.next().unwrap() {
        '1' => Rank::R1,
        '2' => Rank::R2,
        '3' => Rank::R3,
        '4' => Rank::R4,
        '5' => Rank::R5,
        '6' => Rank::R6,
        '7' => Rank::R7,
        '8' => Rank::R8,
        _ => {
            panic!("Invalid rank")
        },
    };
    Square {
        file,
        rank,
    }
}

fn option_to_char(o: Option<Move>) -> char {
    match o {
        Some(_) => '#',
        None => 'O',
    }
}

fn main() {
    let mut game = Game::new();

    let mut input = String::new();
    let stdin = io::stdin();

    println!("  A  B  C  D  E  F  G  H ");
    let mut y_cord = 8;
    for y in RANK_ARRAY.iter().rev() {
        let mut t = String::new();

        for x in FILE_ARRAY {
            t += format!("[{}]", piece_to_char(game.board[y.clone()][x])).as_str();
        }
        println!("{y_cord}{}",t);
        y_cord -= 1;
    }

    loop {
        //displaying the game state
        println!("It is {:?}'s turn.", game.turn);

        if game.check {
            println!("{:?} is in check!", game.turn);
        }

        let castling_color = match game.turn {
            Color::Black => game.castling.black,
            Color::White => game.castling.white,
        };
        print!("These are the remaining castles for {:?}: ", game.turn);
        if let Some(side) = castling_color.0 {
            print!("{:?}, ", side);
        }
        if let Some(side) = castling_color.1 {
            print!("{:?}", side);
        }
        println!();


        print!("These are the captured pieces: ");
        for piece in game.captured.iter() {
            print!("{:?}({:?}), ", piece.piece, piece.color);
        }
        println!();

        println!("Which piece do you want to move? (e.g. e2)");

        input.clear();
        stdin.read_line(&mut input).unwrap();
        let cords = input.as_str();

        let from = cord_to_square(cords);

        match game.possible_moves(from, true) {
            Ok((moves, castels)) => {
                // prints possible board
                println!("Possible moves: ");
                println!("  A  B  C  D  E  F  G  H ");
                let mut y_cord = 8;
                for y in RANK_ARRAY.iter().rev() {
                    let mut t = String::new();

                    for x in FILE_ARRAY {
                        t += format!("[{}]", option_to_char(moves[y.clone()][x]).to_string()).as_str();
                    }
                    println!("{y_cord}{}",t);
                    y_cord -= 1;
                }
                if castels.len() > 0 {
                    print!("Avalable castles: ");
                    for castle in castels {
                        print!("{:?}", match castle {
                            Castle { side: CastlingSide::KingSide } => "king side (e.g O-O)",
                            Castle { side: CastlingSide::QueenSide } => "queen side (e.g O-O-O)",
                            _ => "error",
                        });
                    }
                    println!();
                }
            },
            Err(e) => {
                println!("{}", e);
                continue;
            },
        }

        println!("Where do you want to move it? (e.g. e4, O-O or O-O-O)");

        input.clear();
        stdin.read_line(&mut input).unwrap();
        let cords = input.as_str();

        let mv = match cords {
            "O-O" =>  { Move::Castle { side: CastlingSide::KingSide } },
            "O-O-O" => { Move::Castle { side: CastlingSide::QueenSide } },
            _ => { Move::Normal { from, to: cord_to_square(cords) } },
        };

        match mv {
            Move::Normal { from, to } => {
                println!("{:?} is trying to move {}, from {:?}, {:?} to {:?}, {:?}", game.turn, piece_to_char(game.board[from]), from.file, from.rank, to.file, to.rank);
            },
            Move::Castle { side } => {
                println!("{:?} is castling {}", game.turn, match side {
                    CastlingSide::KingSide => "king side",
                    CastlingSide::QueenSide => "queen side",
                });
            },
        }

        //make move
        match game.try_move(mv) {
            Ok(()) => {
                // prints board if successful
                println!("  A  B  C  D  E  F  G  H ");
                let mut y_cord = 8;
                for y in RANK_ARRAY.iter().rev() {
                    let mut t = String::new();

                    for x in FILE_ARRAY {
                        t += format!("[{}]", piece_to_char(game.board[y.clone()][x])).as_str();
                    }
                    println!("{y_cord}{}",t);
                    y_cord -= 1;
                }
            },
            Err(e) => {
                //prints error explanation if it fails
                println!("{}", e);
                continue;
            },
        }
    }
}

fn piece_to_char(piece: Option<Piece>) -> char {
    match piece {
        Some(Piece { piece: PieceTypes::Bishop, .. }) => 'b',
        Some(Piece { piece: PieceTypes::Rook, .. }) => 'r',
        Some(Piece { piece: PieceTypes::King, .. }) => 'K',
        Some(Piece { piece: PieceTypes::Queen, .. }) => 'q',
        Some(Piece { piece: PieceTypes::Knight, .. }) => 'k',
        Some(Piece { piece: PieceTypes::Pawn(_), .. }) => 'p',
        None => '.',
    }
}