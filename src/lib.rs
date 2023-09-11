//TODO: think about if Copy and Clone are necessary just because compiler recommends it
#[derive(Copy, Clone)]
pub enum PieceTypes {
    Pawn(bool),
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone)]
pub struct Piece {
    pub piece: PieceTypes,
    pub color: Color,
}

///A chess game. All the data from the game is accessible in the fields of the struct but should only be mutated through the associated methods.
pub struct Game {
    pub board: [[Option<Piece>; 8]; 8],
    pub turn: Color,
    pub captured: Vec<Piece>,
    pub castling_black: bool,
    pub castling_white: bool,
    pub fifty_move_rule: u8,
    pub en_passant: Option<(usize, usize)>,
    pub check: bool,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: {
                [[
                    Some(Piece { piece: PieceTypes::Rook, color: Color::White }), 
                    Some(Piece { piece: PieceTypes::Knight, color: Color::White }), 
                    Some(Piece { piece: PieceTypes::Bishop, color: Color::White }), 
                    Some(Piece { piece: PieceTypes::Queen, color: Color::White }), 
                    Some(Piece { piece: PieceTypes::King, color: Color::White }), 
                    Some(Piece { piece: PieceTypes::Bishop, color: Color::White }),
                    Some(Piece { piece: PieceTypes::Knight, color: Color::White }),
                    Some(Piece { piece: PieceTypes::Rook, color: Color::White })
                ],
                [Some(Piece { piece: PieceTypes::Pawn(false), color: Color::White }); 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [Some(Piece { piece: PieceTypes::Pawn(false), color: Color::Black }); 8],
                [
                    Some(Piece { piece: PieceTypes::Rook, color: Color::Black }), 
                    Some(Piece { piece: PieceTypes::Knight, color: Color::Black }), 
                    Some(Piece { piece: PieceTypes::Bishop, color: Color::Black }), 
                    Some(Piece { piece: PieceTypes::King, color: Color::Black }), 
                    Some(Piece { piece: PieceTypes::Queen, color: Color::Black }), 
                    Some(Piece { piece: PieceTypes::Bishop, color: Color::Black }),
                    Some(Piece { piece: PieceTypes::Knight, color: Color::Black }),
                    Some(Piece { piece: PieceTypes::Rook, color: Color::Black })
                ]]
            },
            turn: Color::White,
            captured: vec![],
            castling_black: true,
            castling_white: true,
            fifty_move_rule: 0,
            en_passant: None,
            check: false,
        }
    }

    ///Tries making move described by coordinates. If unsuccessful no move will be made and you can try making a leagal move again. The return vale is an option that shows if a player has won this turn and in that case which color won.
    pub fn try_move(&mut self, x_origin: usize, y_origin: usize, x_destination: usize, y_destination: usize) -> Result<Option<Color>, MoveError> {
        let origin = match self.board[y_origin][x_origin] {
            Some( Piece { color: c, .. } ) if c != self.turn => 
            return Err(MoveError::OpponentPiece("".into())),
            Some(o) => o,
            None => return Err(MoveError::EmptyOriginSquare("".into())),
        };

        let captured = match self.board[y_destination][x_destination] {
            Some(Piece { color: c, .. }) if c == origin.color => 
            return Err(MoveError::SelfCollision("".into())),
            destination @ _ => destination
        };

        //TODO: return errors if moves collide, are not possible for the specific piece or lead to check.
        match origin.piece {
            PieceTypes::Bishop => {
                if x_origin.abs_diff(x_destination) != y_origin.abs_diff(y_destination) {
                    return Err(MoveError::WrongPieceMovement);
                }
                else if self.collision_check_diagonal(x_origin, y_origin, x_destination, y_destination) {
                    return Err(MoveError::Collision);
                }
            },
            PieceTypes::King => {
                if !((x_origin).abs_diff(x_destination) <= 1 && (y_origin).abs_diff(y_destination) <= 1) {
                    return Err(MoveError::WrongPieceMovement);
                }
                //casteling
            },
            PieceTypes::Queen => {
                if x_origin.abs_diff(x_destination) == y_origin.abs_diff(y_destination)  {
                    if self.collision_check_diagonal(x_origin, y_origin, x_destination, y_destination) {
                        return Err(MoveError::Collision);
                    }
                }
                else if (x_origin.abs_diff(x_destination) > 0) ^ (y_origin.abs_diff(y_destination) > 0) {
                    if self.collision_check_straight(x_origin, y_origin, x_destination, y_destination) {
                        return Err(MoveError::Collision);
                    }
                }
                else {
                    return Err(MoveError::WrongPieceMovement);
                }
            },
            PieceTypes::Rook => {
                if !((x_origin.abs_diff(x_destination) > 0) ^ (y_origin.abs_diff(y_destination) > 0)){
                    return Err(MoveError::WrongPieceMovement);
                }
                else if self.collision_check_straight(x_origin, y_origin, x_destination, y_destination) {
                    return Err(MoveError::Collision);
                }
                if match self.turn {
                    Color::Black => self.castling_black,
                    Color::White => self.castling_white
                } {
                    //TODO add castling
                }
            },
            PieceTypes::Pawn(moved) => {
                let origin_row: i32;
                let multiply: i32 = match self.turn {
                    Color::White => { origin_row = 1; 1 },
                    Color::Black => { origin_row = 6; -1 }
                };

                if !(multiply * (y_destination - y_origin) as i32 <= 2) {
                    return Err(MoveError::WrongPieceMovement);
                }
                //checks for en passant
                else if y_destination.abs_diff(y_origin) == 2 {
                    if moved { return Err(MoveError::PawnDubbleMove); }
                    if self.collision_check_straight(x_origin, y_origin, x_destination, y_destination) {
                        return Err(MoveError::Collision);
                    }
                    else if y_origin as i32 == origin_row {
                        self.en_passant = Some((x_destination, y_destination));
                        //TODO: add en passant
                    }
                }
                //checks for promotions
                if origin_row - multiply == y_destination as i32 {
                    //TODO: add promotions 
                    //IDEA: use a game state enum to indicate when promotion happens.
                }
                self.board[y_origin][x_origin].unwrap().piece = PieceTypes::Pawn(true)
            },
            PieceTypes::Knight => {
                if !(((x_origin.abs_diff(x_destination) == 2) && (y_origin.abs_diff(y_destination) == 1)) ^ ((x_origin.abs_diff(x_destination) == 1) && (y_origin.abs_diff(y_destination) == 2))) {
                    return Err(MoveError::WrongPieceMovement);
                }
            },
        }

        self.board[y_destination][x_destination] = Some(origin);
        self.board[y_origin][x_origin] = None;

        if let Some(c) = captured {
            self.captured.push(c);
        }

        //TODO: Check for checkmate and checks
        //TODO: fifty move rule
        //TODO: make draw possible

        Ok(None)
    }

    fn collision_check_straight(&self, x_origin: usize, y_origin: usize, x_destination: usize, y_destination: usize) -> bool {
        false
    }

    fn collision_check_diagonal(&self, x_origin: usize, y_origin: usize, x_destination: usize, y_destination: usize) -> bool {
        false
    }

    pub fn possible_moves(x: usize, y: usize) -> [[bool; 8]; 8] {
        todo!()
    }

    pub fn translate_game_to_pgn(&self) -> &str {
        todo!()
    }

    pub fn translate_move_to_chessnotation(&self, x_origin: usize, y_origin: usize, x_destination: usize, y_destination: usize) -> &str {
        todo!()
    }

    pub fn translate_move_from_chessnotation(input: &str) -> (usize, usize, usize, usize) {
        todo!()
    }

    pub fn translate_game_from_pgn(input: &str) -> Game {
        todo!()
    }
}

//TODO read how to and then make a proper error implementation with std::error::Error.
#[derive(Debug)]
pub enum MoveError {
    SelfCollision(String),
    OpponentPiece(String),
    EmptyOriginSquare(String),
    WrongPieceMovement,
    Collision,
    PawnDubbleMove
}

//TODO: add tests
#[cfg(test)]
mod tests {
    use crate::{ Game, MoveError, Color, Piece, PieceTypes };

    #[test]
    fn it_works() -> Result<(), MoveError> {
        let move_list = [
                [0, 1, 0, 2], //Pawn
                [0, 0, 0, 1], //Rook
                [1, 0, 2, 2], //Knight
                [2, 0, 4, 2], //bishop
                [3, 0, 2, 0], //Queen
                [4, 0, 3, 0], //King
                //[3, 0, 3, 0], //King
            ];

        let mut game = Game::new();

        let mut r: Result<(), MoveError> = Ok(());
        
        for row in move_list {
            let x_origin = row[0];
            let y_origin = row[1];
            let x_destination = row[2];
            let y_destination = row[3];
            
            println!("moving {}, from {x_origin}, {y_origin} to {x_destination}, {y_destination}", match game.board[y_origin][x_origin] {
                Some(Piece { piece: PieceTypes::Bishop, .. }) => "b",
                Some(Piece { piece: PieceTypes::Rook, .. }) => "r",
                Some(Piece { piece: PieceTypes::King, .. }) => "K",
                Some(Piece { piece: PieceTypes::Queen, .. }) => "q",
                Some(Piece { piece: PieceTypes::Knight, .. }) => "k",
                Some(Piece { piece: PieceTypes::Pawn(_), .. }) => "p",
                None => ".",
            });
            r = match game.try_move(x_origin, y_origin, x_destination, y_destination) {
                Ok(win) => { 
                    println!("moved {}", match game.board[y_destination][x_destination] {
                        Some(Piece { piece: PieceTypes::Bishop, .. }) => "b",
                        Some(Piece { piece: PieceTypes::Rook, .. }) => "r",
                        Some(Piece { piece: PieceTypes::King, .. }) => "K",
                        Some(Piece { piece: PieceTypes::Queen, .. }) => "q",
                        Some(Piece { piece: PieceTypes::Knight, .. }) => "k",
                        Some(Piece { piece: PieceTypes::Pawn(_), .. }) => "p",
                        None => ".",
                    });
                    
                    if let Some(color) = win{
                        let winner = match color {
                            Color::Black => "black",
                            Color::White => "white"
                        };
                        println!("moved {}, {} to {}, {} and {} won the game", x_origin, y_origin, x_destination, y_destination, winner);
                    }
                    else {
                        println!("moved {}, {} to {}, {}", x_origin, y_origin, x_destination, y_destination);
                    }
                    Ok(())
                },
                Err(error) => { return Err(error); }
            };
            
            let mut y_cord: usize = 0;
            for y in game.board {
                let mut t = String::new();
                let mut x_cord: usize = 0;
                for x in y {
                    /*if x_cord == x_origin && y_cord == y_origin {
                        t += "*";
                        x_cord += 1;
                        continue;
                    }
                    if x_cord == x_destination && y_cord == y_destination {
                        t += "#";
                        x_cord += 1;
                        continue;
                    }*/
                    
                    t += match x {
                        Some(Piece { piece: PieceTypes::Bishop, .. }) => "b",
                        Some(Piece { piece: PieceTypes::Rook, .. }) => "r",
                        Some(Piece { piece: PieceTypes::King, .. }) => "K",
                        Some(Piece { piece: PieceTypes::Queen, .. }) => "q",
                        Some(Piece { piece: PieceTypes::Knight, .. }) => "k",
                        Some(Piece { piece: PieceTypes::Pawn(_), .. }) => "p",
                        None => ".",
                    };
                    x_cord += 1;
                }
                y_cord += 1;
                println!("{}",t);
            }
        }
        r
    }
}