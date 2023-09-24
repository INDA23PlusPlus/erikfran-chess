use std::fmt::Display;

pub mod util;
use util::{Square, Rank, File, Board, Rows, FILE_ARRAY, RANK_ARRAY, BoardMove, get_square_array};

//TODO: think about if Copy and Clone are necessary just because compiler recommends it

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PieceTypes {
    Pawn(bool),
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Color {
    White,
    Black,
}

impl Color {
    ///returns the other color
    pub fn opposite(&self) -> Color {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Piece {
    pub piece: PieceTypes,
    pub color: Color,
}

#[derive(PartialEq, Debug)]
pub enum GameStatus {
    Checkmate(Color),
    Ongoing,
    Promoting,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum CastlingSide {
    KingSide,
    QueenSide,
}

#[derive(Debug)]
pub struct Castling {
    pub white: (Option<CastlingSide>, Option<CastlingSide>),
    pub black: (Option<CastlingSide>, Option<CastlingSide>),
}

#[derive(Copy, Clone, Debug)]
pub enum Move {
    Normal {
        from: Square,
        to: Square,
    },
    Castle {
        side: CastlingSide,
    },
}

///A chess game. All the data from the game is accessible in the fields of the struct but should only be mutated through the associated methods.
pub struct Game {
    pub board: Board,
    pub turn: Color,
    pub captured: Vec<Piece>,
    pub castling: Castling,
    //pub fifty_move_rule: u8,
    /*pub en_passant_white: Option<Square>,
    pub en_passant_black: Option<Square>,*/
    pub check: bool,
    pub game_status: GameStatus,
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
                    Some(Piece { piece: PieceTypes::Queen, color: Color::Black }),
                    Some(Piece { piece: PieceTypes::King, color: Color::Black }),
                    Some(Piece { piece: PieceTypes::Bishop, color: Color::Black }),
                    Some(Piece { piece: PieceTypes::Knight, color: Color::Black }),
                    Some(Piece { piece: PieceTypes::Rook, color: Color::Black })
                ]].into()
            },
            turn: Color::White,
            captured: vec![],
            castling: Castling {
                white: (Some(CastlingSide::KingSide), Some(CastlingSide::QueenSide)),
                black: (Some(CastlingSide::KingSide), Some(CastlingSide::QueenSide)),
            },
            /*fifty_move_rule: 0,
            en_passant_black: None,
            en_passant_white: None,*/
            check: false,
            game_status: GameStatus::Ongoing,
        }
    }

    /**
    Tries making move described by coordinates. If unsuccessful no move will be made and you can try making a leagal move again.
    The return vale is an option that shows if a player has won this turn and in that case which color won.
    */
    pub fn try_move(&mut self, mv: Move) -> Result<(), MoveError> {
        match mv {
            Move::Castle { side } => {
                let (home_row, possible_castles): (Rank, _) = match self.turn {
                    Color::White => (Rank::R1, self.castling.white),
                    Color::Black => (Rank::R8, self.castling.black),
                };

                match possible_castles {
                    (Some(CastlingSide::KingSide), ..) if side == CastlingSide::KingSide => {
                        if self.board[home_row][File::F].is_some() || self.board[home_row][File::G].is_some() {
                            return Err(MoveError::Collision);
                        }

                        let test_move = Move::Normal {
                            from: Square { file: File::E, rank: home_row },
                            to: Square { file: File::F, rank: home_row },
                        };
                        if self.check_check(mv, self.turn) || self.check_check(test_move, self.turn) || self.check {
                            return Err(MoveError::CastlingError);
                        }

                        if self.check_check(mv, self.turn.opposite()) {
                            self.check = true;
                        }
                        else { self.check = false; }

                        if self.checkmate_check(mv, self.turn.opposite()) {
                            self.game_status = GameStatus::Checkmate(self.turn);
                        }

                        match self.turn {
                            Color::White => { self.castling.white = (None, None); },
                            Color::Black => { self.castling.black = (None, None); },
                        }

                        self.board[home_row][File::G] = self.board[home_row][File::E];
                        self.board[home_row][File::F] = self.board[home_row][File::H];
                        self.board[home_row][File::H] = None;
                        self.board[home_row][File::E] = None;
                    },
                    (.., Some(CastlingSide::QueenSide)) if side == CastlingSide::QueenSide => {
                        if self.board[home_row][File::D].is_some() || self.board[home_row][File::C].is_some() || self.board[home_row][File::B].is_some() {
                            return Err(MoveError::Collision);
                        }

                        let test_move = Move::Normal {
                            from: Square { file: File::E, rank: home_row },
                            to: Square { file: File::D, rank: home_row },
                        };
                        if self.check_check(mv, self.turn) || self.check_check(test_move, self.turn) || self.check {
                            return Err(MoveError::CastlingError);
                        }

                        if self.check_check(mv, self.turn.opposite()) {
                            self.check = true;
                        }
                        else { self.check = false; }

                        if self.checkmate_check(mv, self.turn.opposite()) {
                            self.game_status = GameStatus::Checkmate(self.turn);
                        }

                        match self.turn {
                            Color::White => { self.castling.white = (None, None); },
                            Color::Black => { self.castling.black = (None, None); },
                        }

                        self.board[home_row][File::C] = self.board[home_row][File::E];
                        self.board[home_row][File::D] = self.board[home_row][File::A];
                        self.board[home_row][File::A] = None;
                        self.board[home_row][File::E] = None;
                    },
                    _ => return Err(MoveError::CastlingError),
                }


            },
            Move::Normal { from, to } => {
                let mut origin = match self.board[from] {
                    Some( Piece { color: c, .. } ) if c != self.turn => return Err(MoveError::OpponentPiece),
                    Some(o) => o,
                    None => return Err(MoveError::EmptySquare),
                };

                let captured = match self.board[to] {
                    Some(Piece { color: c, .. }) if c == origin.color =>
                        return Err(MoveError::Collision),
                    destination @ _ => destination
                };


                //TODO: return errors if moves collide, are not possible for the specific piece or lead to check.
                match origin.piece {
                    PieceTypes::Bishop => {
                        if from.file.abs_diff(to.file) != from.rank.abs_diff(to.rank) {
                            return Err(MoveError::WrongPieceMovement);
                        }
                        else if self.collision_check_line(from, to, self.turn) {
                            return Err(MoveError::Collision);
                        }
                    },
                    PieceTypes::King => {
                        if !((from.file).abs_diff(to.file) <= 1 && (from.rank).abs_diff(to.rank) <= 1) {
                            return Err(MoveError::WrongPieceMovement);
                        }
                        else if self.collision_check(to, self.turn) {
                            return Err(MoveError::Collision);
                        }

                        match from.rank {
                            Rank::R1 => { self.castling.white = (None, None); },
                            Rank::R8 => { self.castling.black = (None, None); },
                            _ => {}
                        }

                    },
                    PieceTypes::Queen => {
                        if from.file.abs_diff(to.file) == from.rank.abs_diff(to.rank) ||
                            (from.file.abs_diff(to.file) > 0) ^ (from.rank.abs_diff(to.rank) > 0) {
                            if self.collision_check_line(from, to, self.turn) {
                                return Err(MoveError::Collision);
                            }
                        } else {
                            return Err(MoveError::WrongPieceMovement);
                        }
                    },
                    PieceTypes::Rook => {
                        if !((from.file.abs_diff(to.file) > 0) ^ (from.rank.abs_diff(to.rank) > 0)){
                            return Err(MoveError::WrongPieceMovement);
                        }
                        else if self.collision_check_line(from, to, self.turn) {
                            return Err(MoveError::Collision);
                        }
                        match from.file {
                            File::A => {
                                match from.rank {
                                    Rank::R1 => { self.castling.white.1 = None; },
                                    Rank::R8 => { self.castling.black.1 = None; },
                                    _ => {}
                                }
                            },
                            File::H => {
                                match from.rank {
                                    Rank::R1 => { self.castling.white.0 = None; },
                                    Rank::R8 => { self.castling.black.0 = None; },
                                    _ => {}
                                }
                            },
                            _ => {}
                        }
                    },
                    PieceTypes::Pawn(moved) => {
                        let multiply: i32 = match self.turn {
                            Color::White => 1,
                            Color::Black => -1
                        };

                        if !((to.rank.abs_diff(from.rank) == 1
                            && to.file.abs_diff(from.file) <= 1)
                            || (to.rank.abs_diff(from.rank) == 2
                            && to.file.abs_diff(from.file) == 0)) {
                            return Err(MoveError::WrongPieceMovement);
                        }

                        if to.file.abs_diff(from.file) == 1 && to.rank.abs_diff(from.rank) == 1 {
                            if self.board[to].is_none() {
                                return Err(MoveError::WrongPieceMovement);
                            }
                        }

                        if self.board[Rank::try_from(multiply + i32::from(from.rank)).unwrap()][from.file].is_some()
                            && to.file.abs_diff(from.file) == 0 {
                            return Err(MoveError::Collision);
                        }

                        if self.collision_check(to, self.turn) && to.file.abs_diff(from.file) == 1 {
                            return Err(MoveError::Collision);
                        }

                        if to.rank.abs_diff(from.rank) == 2 {
                            if moved { return Err(MoveError::PawnDubbleMove); }
                            if self.board[to].is_some() {
                                return Err(MoveError::Collision);
                            }
                            /*else if from.rank.num() == origin_row {
                                self.en_passant_black = Some(to);
                                //TODO: add en passant
                            }*/
                        }
                        //checks for promotions
                        /*if origin_row - multiply == to.rank.num() {
                            //TODO: add promotions
                            //IDEA: use a game state enum to indicate when promotion happens.
                        }*/
                        origin = Piece { piece: PieceTypes::Pawn(true), color: origin.color };
                    },
                    PieceTypes::Knight => {
                        if !(((from.file.abs_diff(to.file) == 2)
                            && (from.rank.abs_diff(to.rank) == 1)) ^
                            ((from.file.abs_diff(to.file) == 1)
                            && (from.rank.abs_diff(to.rank) == 2))) {
                            return Err(MoveError::WrongPieceMovement);
                        }
                        else if self.collision_check(to, self.turn) {
                            return Err(MoveError::Collision);
                        }
                    },
                }

                if self.check_check(mv, self.turn) {
                    return Err(MoveError::SelfCheck);
                }

                if let Some(c) = captured {
                    self.captured.push(c);
                }

                if self.check_check(mv, self.turn.opposite()) {
                    self.check = true;
                }
                else { self.check = false; }

                if self.checkmate_check(mv, self.turn.opposite()) {
                    self.game_status = GameStatus::Checkmate(self.turn);
                }

                self.board[to] = Some(origin);
                self.board[from] = None;
            },
        }

        self.turn = self.turn.opposite();

        //TODO: fifty move rule
        //TODO: make draw possible

        Ok(())
    }

    ///checks for collisions with pieces of the color color or multiple collisions with the other color in a line from origin to destination. Returns the sqaure where the piece cant go becasuse of the collison. Can input out of bounds cords and it will stop at the edge
    fn collision_check_line(&self, from: Square, to: Square, color: Color) -> bool {
        let direction: (i32, i32) = (
            (to.file.num() - from.file.num()).signum(),
            (to.rank.num() - from.rank.num()).signum()
        );

        //let t_2 = (from.file.num(), from.rank.num(), to.file.num(), to.rank.num());

        let range =
            if direction.0 != 0 { from.file.abs_diff(to.file) }
            else { from.rank.abs_diff(to.rank) };

        /*
        if a enemy is encountered a collison should only be reported the next square
        if that exsist therfore we use enemy counter to keep track of that has happened.
        */
        let mut enemy_counter = false;
        for i in 1..=range {
            let x = from.file.num() + direction.0 * i;
            let y = from.rank.num() + direction.1 * i;

            let square = Square::try_from((x, y)).unwrap();

            if enemy_counter {
                return true
            }

            if let Some(piece) = self.board[square] {
                if piece.color == color {
                    return true
                }
                else { enemy_counter = true; }
            }
        }
        false
    }

    ///checks for collision for a piece of color color in the position x, y.
    fn collision_check(&self, square: Square, color: Color) -> bool {
        match self.board[square] {
            Some(Piece { color: c, .. }) if c == color => true,
            _ => false
        }
    }

    ///the color is who is in check. Will just panic if its passed a invalid move
    fn check_check(&mut self, mv: Move, color: Color) -> bool {
        let temp_board = self.board.clone();

        match mv {
            Move::Normal { from, to } => {
                match self.board[to] {
                    Some( Piece { piece: PieceTypes::King, color: c } ) if self.turn.opposite() == c => return true,
                    _ => {}
                }

                self.board[to] = Some(self.board[from].unwrap());
                self.board[from] = None;
            },
            Move::Castle { side } => {
                let (home_row, possible_castles): (Rank, _) = match self.turn {
                    Color::White => (Rank::R1, &self.castling.white),
                    Color::Black => (Rank::R8, &self.castling.black),
                };

                match possible_castles {
                    (Some(CastlingSide::KingSide), ..) if side == CastlingSide::KingSide => {
                        self.board[home_row][File::G] = self.board[home_row][File::E];
                        self.board[home_row][File::F] = self.board[home_row][File::H];
                        self.board[home_row][File::H] = None;
                        self.board[home_row][File::E] = None;
                    },
                    (.., Some(CastlingSide::QueenSide)) if side == CastlingSide::QueenSide => {
                        self.board[home_row][File::C] = self.board[home_row][File::E];
                        self.board[home_row][File::D] = self.board[home_row][File::A];
                        self.board[home_row][File::A] = None;
                        self.board[home_row][File::E] = None;
                    },
                    _ => return false,
                }
            }
        }

        let mut king_pos = None;
        for square in get_square_array() {
            if let Some(piece) = self.board[square] {
                //println!("{:?}, {:?}", piece.piece == PieceTypes::King, piece.color == color);
                if (piece.piece == PieceTypes::King && piece.color == color) {
                    king_pos = Some(square);
                }
            }
        }
        //println!();
        //println!("move: {:?}", mv);
        assert!(king_pos.is_some());
        let mut result = false;

        for square in get_square_array() {
            if let Some(piece) = self.board[square] {
                if piece.color == color {
                    continue;
                }
            }
            if let Ok((moves, _)) = self.possible_moves(square, false) {
                if moves[king_pos.unwrap()].is_some() {
                    //println!("move_2: {:?}", moves[king_pos.unwrap()]);
                    result = true;
                    break;
                }
            }
        }

        self.board = temp_board;

        result
    }

    ///the color is who is in check. Will just panic if its passed a invalid move
    fn checkmate_check(&mut self, mv: Move, color: Color) -> bool {
        if !self.check {
            return false;
        }

        let temp_board = self.board.clone();

        match mv {
            Move::Normal { from, to } => {
                self.board[to] = Some(self.board[from].unwrap());
                self.board[from] = None;
            },
            Move::Castle { side } => {
                let (home_row, possible_castles): (Rank, _) = match self.turn {
                    Color::White => (Rank::R1, &self.castling.white),
                    Color::Black => (Rank::R8, &self.castling.black),
                };

                match possible_castles {
                    (Some(CastlingSide::KingSide), ..) if side == CastlingSide::KingSide => {
                        self.board[home_row][File::G] = self.board[home_row][File::E];
                        self.board[home_row][File::F] = self.board[home_row][File::H];
                        self.board[home_row][File::H] = None;
                        self.board[home_row][File::E] = None;
                    },
                    (.., Some(CastlingSide::QueenSide)) if side == CastlingSide::QueenSide => {
                        self.board[home_row][File::C] = self.board[home_row][File::E];
                        self.board[home_row][File::D] = self.board[home_row][File::A];
                        self.board[home_row][File::A] = None;
                        self.board[home_row][File::E] = None;
                    },
                    _ => panic!(),
                }
            }
        }

        let mut result = true;

        for square in get_square_array() {
            if let Some(piece) = self.board[square] {
                if piece.color != color {
                    continue;
                }
            }
            //ignore castles since they cant be done if you are in check.
            if let Ok((moves, _)) = self.possible_moves(square, false) {
                for square_1 in get_square_array() {
                    if moves[square_1].is_some() {
                        let test_move = Move::Normal {
                            from: square,
                            to: square_1,
                        };
                        result = !self.check_check(test_move, color);
                    }
                }
            }
        }

        self.board = temp_board;

        result
    }

    fn possible_moves_directions(
        &mut self,
        color: Color,
        from: Square,
        possible_board: &mut BoardMove,
        directions: Vec<(i32, i32)>,
        check_checks: bool
    ) {
        for direction in directions {
            for i in 1..=8 {
                let x_1 = from.file.num() + direction.0 * i;
                let y_1 = from.rank.num() + direction.1 * i;

                if let Ok(square) = Square::try_from((x_1, y_1)) {
                    match self.board[square] {
                        Some( Piece { color: c, .. } ) if c == color => break,
                        Some( Piece { color: c, .. } ) if c != color => {
                            self.test_move(possible_board, x_1, y_1, from, color, check_checks);
                            break;
                        },
                        None => self.test_move(possible_board, x_1, y_1, from, color, check_checks),
                        Some(_) => unreachable!()
                    }
                }

                self.test_move(possible_board, x_1, y_1, from, color, check_checks);
            }
        }
    }

    fn test_move(&mut self, possible_board: &mut BoardMove, x: i32, y: i32, from: Square, color: Color, check_checks: bool) {
        let mut test = false;

        if let Ok(square) = Square::try_from((x, y)) {
            if !self.collision_check(square, color) {
                let test_move = Move::Normal {
                    from,
                    to: square,
                };
                if !check_checks || !self.check_check(test_move, self.turn) {
                    test = true;
                    possible_board[square] = Some(test_move);
                }
            }
        }
        //if (x > 7 || x < 0 || y > 7 || y < 0) && test { println!("{:?}, {}, {}", from, x, y) }
    }

    /// returns a board with all the normal moves possible for the piece in the from Square.
    /// potential Castelmoves are returned in a seperate Vec.
    /// If check_checks is true the function will check if the move leads to check and if it does it will not be included in the return value. 
    /// this is only false when calling the function internaly to prevent stack overflow.
    /// TLDR: check_checks should always be true when calling this function.
    pub fn possible_moves(&mut self, from: Square, check_checks: bool) -> Result<(BoardMove, Vec<Move>), MoveError> {
        let piece = self.board[from].map_or(Err(MoveError::EmptySquare), |p| Ok(p))?;
        let mut possible_board = BoardMove::from([[None; 8]; 8]);
        let mut castles: Vec<Move> = vec![];

        let straight_directions: Vec<(i32, i32)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
        let diagonal_directions: Vec<(i32, i32)> = vec![(1, 1), (-1, 1), (-1, -1), (1, -1)];
        match piece.piece {
            PieceTypes::Knight => {
                for direction in straight_directions {
                    let x_temp: i32 = direction.0 * 2 + from.file.num();
                    let y_temp: i32 = direction.1 * 2 + from.rank.num();

                    let x_1 = x_temp + direction.1;
                    let y_1 = y_temp + direction.0;
                    self.test_move(&mut possible_board, x_1, y_1, from, piece.color, check_checks);

                    let x_2 = x_temp - direction.1;
                    let y_2 = y_temp - direction.0;
                    self.test_move(&mut possible_board, x_2, y_2, from, piece.color, check_checks);
                }
            },
            PieceTypes::King => {
                for direction in [straight_directions, diagonal_directions].concat() {
                    let x_1 = from.file.num() + direction.0;
                    let y_1 = from.rank.num() + direction.1;

                    self.test_move(&mut possible_board, x_1, y_1, from, piece.color, check_checks);
                }
            },
            PieceTypes::Bishop => {
                self.possible_moves_directions(piece.color, from, &mut possible_board, diagonal_directions, check_checks);
            },
            PieceTypes::Rook => {
                self.possible_moves_directions(piece.color, from, &mut possible_board, straight_directions, check_checks);

                let (home_row, possible_castles): (Rank, _) = match piece.color {
                    Color::White => (Rank::R1, self.castling.white),
                    Color::Black => (Rank::R8, self.castling.black),
                };

                if let (Some(CastlingSide::KingSide), ..) = possible_castles {
                    'label: {
                        //println!("{:?}", self.castling);
                        let test_move = Move::Normal {
                            from: Square { file: File::E, rank: home_row },
                            to: Square { file: File::F, rank: home_row },
                        };
                        let test_move_1 = Move::Normal {
                            from: Square { file: File::E, rank: home_row },
                            to: Square { file: File::G, rank: home_row },
                        };
                        if self.board[home_row][File::F].is_some()
                            || self.board[home_row][File::G].is_some()
                            || self.check_check(test_move, self.turn)
                            || self.check_check(test_move_1, self.turn)
                            || self.check {
                            break 'label;
                        }

                        castles.push(Move::Castle {
                            side: CastlingSide::KingSide,
                        });
                    }
                }
                if let (.., Some(CastlingSide::QueenSide)) = possible_castles {
                    'label: {
                        let test_move = Move::Normal {
                            from: Square { file: File::E, rank: home_row },
                            to: Square { file: File::D, rank: home_row },
                        };
                        let test_move_1 = Move::Normal {
                            from: Square { file: File::E, rank: home_row },
                            to: Square { file: File::C, rank: home_row },
                        };
                        if self.board[home_row][File::D].is_some()
                            || self.board[home_row][File::C].is_some()
                            || self.board[home_row][File::B].is_some()
                            || self.check_check(test_move, self.turn)
                            || self.check_check(test_move_1, self.turn)
                            || self.check {
                            break 'label;
                        }

                        castles.push(Move::Castle {
                            side: CastlingSide::QueenSide,
                        });
                    }
                }
            },
            PieceTypes::Queen => {
                self.possible_moves_directions(piece.color, from, &mut possible_board, [straight_directions, diagonal_directions].concat(), check_checks);
            },
            PieceTypes::Pawn(moved) => {
                let movement_direction: i32 = match piece.color {
                    Color::Black => -1,
                    Color::White => 1,
                };

                //TODO: add en passant
                //checks one or two moves forward
                let y_1: i32 = from.rank.num() + movement_direction;

                if let Ok(rank_1) = Rank::try_from(y_1) {
                    if self.board[rank_1][from.file].is_none() {
                        self.test_move(&mut possible_board, from.file.into(), y_1, from, piece.color, check_checks);

                        let y_2: i32 = from.rank.num() + movement_direction * 2;

                        if let Ok(rank_2) = Rank::try_from(y_2) {
                            if !moved && self.board[rank_2][from.file].is_none() {
                                self.test_move(&mut possible_board, from.file.into(), y_2, from, piece.color, check_checks);
                            }
                        }
                    }
                }

                //checks for enemy's in the diagonal
                let x_3 = from.file.num() + 1;
                if let Ok(square) = Square::try_from((x_3, y_1)) {
                    if let Some(enemy) = self.board[square] {
                        if enemy.color != piece.color {
                            self.test_move(&mut possible_board, x_3, y_1, from, piece.color, check_checks);
                        }
                    }
                }

                let x_4 = from.file.num() - 1;
                if let Ok(square) = Square::try_from((x_4, y_1)) {
                    if let Some(enemy) = self.board[square] {
                        if enemy.color != piece.color {
                            self.test_move(&mut possible_board, x_4, y_1, from, piece.color, check_checks);
                        }
                    }
                }
            }
        }
        Ok((possible_board, castles))
    }

    /*pub fn translate_move_to_san(&self, mv: Moves) -> &str {
        todo!()
    }

    pub fn translate_move_from_san(input: &str) -> (usize, usize, usize, usize) {
        todo!()
    }

    pub fn translate_game_from_fen(input: &str) -> Game {
        todo!()
    }

    pub fn translate_game_to_fen(&self) -> &str {
        todo!()
    }*/
}

//TODO read how to and then make a proper error implementation with std::error::Error.
#[derive(Debug, PartialEq)]
pub enum MoveError {
    OpponentPiece,
    ///trying to access an empty square
    EmptySquare,
    WrongPieceMovement,
    Collision,
    PawnDubbleMove,
    CastlingError,
    SelfCheck,
    None,
}

use std::error;
use std::fmt;

impl fmt::Display for MoveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MoveError::OpponentPiece => write!(f, "Opponent piece: the piece you are trying to move is not yours"),
            MoveError::EmptySquare => write!(f, "Empty square: the square you are trying to move from is empty"),
            MoveError::WrongPieceMovement => write!(f, "Wrong piece movement: the piece you are trying to move cant move like that"),
            MoveError::Collision => write!(f, "Collision: the piece you are trying to move cant move there because of a collision. Either with a piece of your own or the opponents"),
            MoveError::PawnDubbleMove => write!(f, "Pawn dubble move: the pawn you are trying to move has already moved. Can only move two squares on the first move"),
            MoveError::CastlingError => write!(f, "Castling error: the castling you are trying to do is not possible"),
            MoveError::SelfCheck => write!(f, "Self check: the move you are trying to do leads to check for yourself"),
            MoveError::None => write!(f, "None: the move you are trying to do is not possible"),
        }
    }
}

impl error::Error for MoveError { }

#[cfg(test)]
mod tests {
    use pgn_reader::{Visitor, Skip, BufferedReader, SanPlus, Outcome};
    use super::*;
    use shakmaty::{Chess, Position};

    use reqwest;
    use tokio;
    use std::{io::Read, env};
    use shakmaty::Outcome::Decisive;

    use super::*;

    /*#[test]
    fn test_possible_moves_knight() {
        env::set_var("RUST_BACKTRACE", "1");

        let game = Game::new();
        let possible_moves = game.possible_moves(1, 0, true).unwrap();
        println!("{:?}", possible_moves);
        assert_eq!(possible_moves[2][2], true);
        assert_eq!(possible_moves[0][2], false);
    }

    #[test]
    fn test_possible_moves_bishop() {
        env::set_var("RUST_BACKTRACE", "1");

        let mut game = Game::new();
        game.board[3][3] = Some(Piece { color: Color::White, piece: PieceTypes::Bishop});
        let possible_moves = game.possible_moves(3, 3, true).unwrap();
        assert_eq!(possible_moves[4][4], true);
        assert_eq!(possible_moves[2][2], true);
        assert_eq!(possible_moves[4][3], false);
    }

    #[test]
    fn test_possible_moves_rook() {
        env::set_var("RUST_BACKTRACE", "1");

        let mut game = Game::new();
        game.board[3][3] = Some(Piece { color: Color::White, piece: PieceTypes::Rook});
        let possible_moves = game.possible_moves(3, 3, true).unwrap();
        assert_eq!(possible_moves[3][4], true);
        assert_eq!(possible_moves[3][2], true);
        assert_eq!(possible_moves[2][3], true);
        assert_eq!(possible_moves[4][3], true);
        assert_eq!(possible_moves[4][4], false);
    }

    #[test]
    fn test_possible_moves_queen() {
        env::set_var("RUST_BACKTRACE", "1");

        let mut game = Game::new();
        game.board[3][3] = Some(Piece { color: Color::White, piece: PieceTypes::Queen});
        let possible_moves = game.possible_moves(3, 3, true).unwrap();
        assert_eq!(possible_moves[4][4], true);
        assert_eq!(possible_moves[2][2], true);
        assert_eq!(possible_moves[4][5], false);
        assert_eq!(possible_moves[3][4], true);
        assert_eq!(possible_moves[3][2], true);
        assert_eq!(possible_moves[2][3], true);
        assert_eq!(possible_moves[4][3], true);
    }

    #[test]
    fn test_possible_moves_pawn() {
        env::set_var("RUST_BACKTRACE", "1");

        let game = Game::new();
        let possible_moves = game.possible_moves(1, 1, true).unwrap();
        assert_eq!(possible_moves[2][1], true);
        assert_eq!(possible_moves[3][1], true);
        assert_eq!(possible_moves[2][2], false);
        assert_eq!(possible_moves[2][0], false);
    }

    #[test]
    fn test_possible_moves_empty_square() {
        env::set_var("RUST_BACKTRACE", "1");

        let game = Game::new();
        let result = game.possible_moves(4, 4, true);
        assert_eq!(result, Err(Error::EmptySquare));
    }*/


    struct TestGame {
        moves: Vec<crate::Move>,
        chess: Chess,
        outcome: Option<Outcome>,
    }

    impl TestGame {
        fn new() -> TestGame {
            TestGame {
                moves: vec![],
                chess: Chess::default(),
                outcome: None,
            }
        }
    }

    impl Visitor for TestGame {
        type Result = (Vec<crate::Move>, Option<Outcome>);

        fn begin_game(&mut self) {
            self.moves = vec![];
            self.chess = Chess::default();
            self.outcome = None;
        }

        fn san(&mut self, san_plus: SanPlus) {
            let mov = san_plus.san
                .to_move(&self.chess)
                .unwrap();

            match mov {
                shakmaty::Move::Normal { from, to, .. } => {
                    self.moves.push(
                        crate::Move::Normal {
                            from: square_to_square(from),
                            to: square_to_square(to)
                        }
                    );
                },
                shakmaty::Move::Castle { king, rook } => {
                    self.moves.push(crate::Move::Castle {
                        side: match rook {
                            shakmaty::Square::A1 | shakmaty::Square::A8 => CastlingSide::QueenSide,
                            shakmaty::Square::H1 | shakmaty::Square::H8 => CastlingSide::KingSide,
                            _ => unreachable!(),
                        }
                    });
                }
                _ => {},
            }

            self.chess = self.chess.clone().play(&mov).unwrap();
        }

        fn begin_variation(&mut self) -> Skip {
            Skip(true) // stay in the mainline
        }

        fn outcome(&mut self, outcome: Option<Outcome>) {
            self.outcome = outcome;
        }

        fn end_game(&mut self) -> Self::Result {
            (self.moves.clone(), self.outcome.clone())
        }
    }

    fn square_to_square (square: shakmaty::Square) -> Square {
        Square {
            file: match square.file() {
                shakmaty::File::A => File::A,
                shakmaty::File::B => File::B,
                shakmaty::File::C => File::C,
                shakmaty::File::D => File::D,
                shakmaty::File::E => File::E,
                shakmaty::File::F => File::F,
                shakmaty::File::G => File::G,
                shakmaty::File::H => File::H,
            },
            rank: match square.rank() {
                shakmaty::Rank::First => Rank::R1,
                shakmaty::Rank::Second => Rank::R2,
                shakmaty::Rank::Third => Rank::R3,
                shakmaty::Rank::Fourth => Rank::R4,
                shakmaty::Rank::Fifth => Rank::R5,
                shakmaty::Rank::Sixth => Rank::R6,
                shakmaty::Rank::Seventh => Rank::R7,
                shakmaty::Rank::Eighth => Rank::R8,
            },
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

    #[tokio::test]
    async fn database_games_test() {
        env::set_var("RUST_BACKTRACE", "1");

        for i in 0..10 {
            let res = reqwest::get("https://lichess.org/game/export/4OtIh2oh")
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            let pgn = res.as_bytes();
            println!("{}", res);


            let mut reader = BufferedReader::new_cursor(&pgn[..]);

            let mut test_game = TestGame::new();
            let (moves, outcome) = match reader.read_game(&mut test_game).unwrap() {
                Some(m) => m,
                None => (vec![], None),
            };

            let mut game = Game::new();
            println!("{:?}", moves);


            for mv in moves {
                match mv {
                    Move::Normal { from, to } => {
                        println!("{:?} is moving {}, from {:?}, {:?} to {:?}, {:?}", game.turn, piece_to_char(game.board[from]), from.file, from.rank, to.file, to.rank);
                    },
                    Move::Castle { side } => {
                        println!("{:?} is castling {}", game.turn, match side {
                            CastlingSide::KingSide => "king side",
                            CastlingSide::QueenSide => "queen side",
                        });
                    },
                }

                game.try_move(mv).unwrap();

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
            }
            match outcome {
                Some(Decisive { winner: pgn_reader::Color::White }) => {
                    assert_eq!(game.game_status, GameStatus::Checkmate(Color::White));
                },
                Some(Decisive { winner: pgn_reader::Color::Black }) => {
                    assert_eq!(game.game_status, GameStatus::Checkmate(Color::Black));
                },
                _ => {}
            }
        }
    }
}