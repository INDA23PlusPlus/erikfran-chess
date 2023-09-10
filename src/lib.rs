//TODO: think about if Copy and Clone are necessary just because compiler recommends it
#[derive(Copy, Clone)]
pub enum PieceTypes {
    Pawn,
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
    piece: PieceTypes,
    color: Color,
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
                [Some(Piece { piece: PieceTypes::Pawn, color: Color::White }); 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [Some(Piece { piece: PieceTypes::Pawn, color: Color::Black }); 8],
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
        }
    }

    ///Tries making move described by coordinates. If unsuccessful no move will be made and you can try making a leagal move again. The return vale is an option that shows if a player has won this turn and in that case which color won.
    pub fn try_move(&mut self, x_origin: usize, y_origin: usize, x_destination: usize, y_destination: usize) -> Result<Option<Color>, MoveError> {
        let origin = match self.board[x_origin][y_origin] {
            Some( Piece { color: c, .. } ) if c != self.turn => 
            return Err(MoveError::OpponentPiece("".into())),
            Some(o) => o,
            None => return Err(MoveError::EmptyOriginSquare("".into())),
        };

        let captured = match self.board[x_destination][y_destination] {
            Some(Piece { color: c, .. }) if c == origin.color => 
            return Err(MoveError::SelfCollision("".into())),
            destination @ _ => destination
        };

        //TODO: return errors if moves collide, are not possible for the specific piece or lead to check.
        match origin.piece {
            PieceTypes::Bishop => {
                
            },
            PieceTypes::King => {

            },
            PieceTypes::Queen => {
                
            },
            PieceTypes::Rook => {
                
            },
            PieceTypes::Pawn => {
                
            },
            PieceTypes::Knight => {
                
            },
        }

        self.board[x_destination][y_destination] = Some(origin);
        self.board[x_origin][y_origin] = None;

        if let Some(c) = captured {
            self.captured.push(c);
        }

        //TODO: Check for checkmate

        Ok(None)
    }

    pub fn possible_moves(x: usize, y: usize) -> [[bool; 8]; 8] {
        todo!()
    }

    pub fn translate_game_to_chessnotation(&self) -> &str {
        todo!()
    }

    pub fn translate_move_to_chessnotation(&self, x_origin: usize, y_origin: usize, x_destination: usize, y_destination: usize) -> &str {
        todo!()
    }

    pub fn translate_move_from_chessnotation(input: &str) -> (usize, usize, usize, usize) {
        todo!()
    }

    pub fn translate_game_from_chessnotation(input: &str) -> Game {
        todo!()
    }
}

//TODO read how to and then make a proper error implementation with std::error::Error.
#[derive(Debug)]
pub enum MoveError {
    SelfCollision(String),
    OpponentPiece(String),
    EmptyOriginSquare(String),
    WrongPieceMovment(String),
}