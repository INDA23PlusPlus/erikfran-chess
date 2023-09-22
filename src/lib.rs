use std::fmt::Display;

mod util;
use util::{ Square, Rank, File, Board, Rows, FILE_ITER, RANK_ITER };
use crate::util::{Board_Move, SQUARE_ITER};

//TODO: think about if Copy and Clone are necessary just because compiler recommends it

#[derive(Copy, Clone, PartialEq, Eq)]
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

pub enum GameStatus {
    Checkmate(Color),
    Draw,
    Ongoing,
    Promoting,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum CastlingSide {
    KingSide,
    QueenSide,
}

pub struct Castling {
    white: (Option<CastlingSide>, Option<CastlingSide>),
    black: (Option<CastlingSide>, Option<CastlingSide>),
}

#[derive(Copy, Clone)]
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
    pub fifty_move_rule: u8,
    pub en_passant_white: Option<Square>,
    pub en_passant_black: Option<Square>,
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
                    Some(Piece { piece: PieceTypes::King, color: Color::Black }),
                    Some(Piece { piece: PieceTypes::Queen, color: Color::Black }),
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
            fifty_move_rule: 0,
            en_passant_black: None,
            en_passant_white: None,
            check: false,
            game_status: GameStatus::Ongoing,
        }
    }

    /**
    Tries making move described by coordinates. If unsuccessful no move will be made and you can try making a leagal move again.
    The return vale is an option that shows if a player has won this turn and in that case which color won.
    */
    pub fn try_move(&mut self, mv: Move) -> Result<(), Error> {
        match mv {
            Move::Castle { side } => {
                let (home_row, possible_castles): (Rank, _) = match self.turn {
                    Color::White => (Rank::R1, &self.castling.white),
                    Color::Black => (Rank::R8, &self.castling.black),
                };

                match possible_castles {
                    (Some(CastlingSide::KingSide), ..) if side == CastlingSide::KingSide => {
                        if self.board[home_row][File::F].is_some() || self.board[home_row][File::G].is_some() {
                            return Err(Error::Collision);
                        }

                        let test_move = Move::Normal {
                            from: Square { file: File::E, rank: home_row },
                            to: Square { file: File::F, rank: home_row },
                        };
                        if self.check_check(test_move, self.turn) || self.check {
                            return Err(Error::CastlingError);
                        }

                        self.board[home_row][File::G] = self.board[home_row][File::E];
                        self.board[home_row][File::F] = self.board[home_row][File::H];
                        self.board[home_row][File::H] = None;
                        self.board[home_row][File::E] = None;
                    },
                    (.., Some(CastlingSide::QueenSide)) if side == CastlingSide::QueenSide => {
                        if self.board[home_row][File::D].is_some() || self.board[home_row][File::C].is_some() || self.board[home_row][File::B].is_some() {
                            return Err(Error::Collision);
                        }

                        let test_move = Move::Normal {
                            from: Square { file: File::E, rank: home_row },
                            to: Square { file: File::D, rank: home_row },
                        };
                        if self.check_check(test_move, self.turn) || self.check {
                            return Err(Error::CastlingError);
                        }

                        self.board[home_row][File::C] = self.board[home_row][File::E];
                        self.board[home_row][File::D] = self.board[home_row][File::A];
                        self.board[home_row][File::A] = None;
                        self.board[home_row][File::E] = None;
                    },
                    _ => return Err(Error::CastlingError),
                }
            },
            Move::Normal { from, to } => {
                let origin = match self.board[from] {
                    Some( Piece { color: c, .. } ) if c != self.turn => return Err(Error::OpponentPiece),
                    Some(o) => o,
                    None => return Err(Error::EmptySquare),
                };

                let captured = match self.board[to] {
                    Some(Piece { color: c, .. }) if c == origin.color =>
                        return Err(Error::Collision),
                    destination @ _ => destination
                };


                //TODO: return errors if moves collide, are not possible for the specific piece or lead to check.
                match origin.piece {
                    PieceTypes::Bishop => {
                        if from.file.abs_diff(to.file) != from.rank.abs_diff(to.rank) {
                            return Err(Error::WrongPieceMovement);
                        }
                        else if self.collision_check_line(from, to, self.turn) {
                            return Err(Error::Collision);
                        }
                    },
                    PieceTypes::King => {
                        if !((from.file).abs_diff(to.file) <= 1 && (from.rank).abs_diff(to.rank) <= 1) {
                            return Err(Error::WrongPieceMovement);
                        }
                        else if self.collision_check(to, self.turn) {
                            return Err(Error::Collision);
                        }
                    },
                    PieceTypes::Queen => {
                        if from.file.abs_diff(to.file) == from.rank.abs_diff(to.rank) ||
                            (from.file.abs_diff(to.file) > 0) ^ (from.rank.abs_diff(to.rank) > 0) {
                            if self.collision_check_line(from, to, self.turn) {
                                return Err(Error::Collision);
                            }
                        } else {
                            return Err(Error::WrongPieceMovement);
                        }
                    },
                    PieceTypes::Rook => {
                        if !((from.file.abs_diff(to.file) > 0) ^ (from.rank.abs_diff(to.rank) > 0)){
                            return Err(Error::WrongPieceMovement);
                        }
                        else if self.collision_check_line(from, to, self.turn) {
                            return Err(Error::Collision);
                        }
                    },
                    PieceTypes::Pawn(moved) => {
                        let origin_row: i32;
                        let multiply: i32 = match self.turn {
                            Color::White => { origin_row = 1; 1 },
                            Color::Black => { origin_row = 6; -1 }
                        };

                        if !(multiply * (to.rank.abs_diff(from.rank)) as i32 <= 2) {
                            return Err(Error::WrongPieceMovement);
                        }
                        //checks for en passant
                        else if to.rank.abs_diff(from.rank) == 2 {
                            if moved { return Err(Error::PawnDubbleMove); }
                            if self.collision_check_line(from, to, self.turn) {
                                return Err(Error::Collision);
                            }
                            else if from.rank as i32 == origin_row {
                                self.en_passant_black = Some(to);
                                //TODO: add en passant
                            }
                        }
                        //checks for promotions
                        /*if origin_row - multiply == to.rank as i32 {
                            //TODO: add promotions
                            //IDEA: use a game state enum to indicate when promotion happens.
                        }*/
                        self.board[from].unwrap().piece = PieceTypes::Pawn(true)
                    },
                    PieceTypes::Knight => {
                        if !(((from.file.abs_diff(to.file) == 2)
                            && (from.rank.abs_diff(to.rank) == 1)) ^
                            ((from.file.abs_diff(to.file) == 1)
                            && (from.rank.abs_diff(to.rank) == 2))) {
                            return Err(Error::WrongPieceMovement);
                        }
                        else if self.collision_check(to, self.turn) {
                            return Err(Error::Collision);
                        }
                    },
                }

                if self.check_check(mv, self.turn) {
                    return Err(Error::SelfCheck);
                }

                if let Some(c) = captured {
                    self.captured.push(c);
                }

                self.board[to] = Some(origin);
                self.board[from] = None;
            },
        }

        if self.check_check(mv, self.turn.opposite()) {
            self.check = true;
        }
        else { self.check = false; }

        if self.checkmate_check(mv, self.turn.opposite()) {
            self.game_status = GameStatus::Checkmate(self.turn);
        }

        self.turn = self.turn.opposite();

        //TODO: fifty move rule
        //TODO: make draw possible

        Ok(())
    }

    ///checks for collisions with pieces of the color color or multiple collisions with the other color in a line from origin to destination. Returns the sqaure where the piece cant go becasuse of the collison. Can input out of bounds cords and it will stop at the edge
    fn collision_check_line(&self, from: Square, to: Square, color: Color) -> bool {
        let direction = (
            (to.file as i32 - from.file as i32).signum(),
            (from.rank as i32 - from.rank as i32).signum()
        );

        let range =
            if direction.0 != 0 { from.file.abs_diff(to.file) }
            else { from.rank.abs_diff(to.rank) };

        /*
        if a enemy is encountered a collison should only be reported the next square
        if that exsist therfore we use enemy counter to keep track of that has happened.
        */
        let mut enemy_counter = false;
        for i in 1..=range {
            let x = from.file as i32 + direction.0 * i;
            let y = from.rank as i32 + direction.1 * i;

            let square = Square::try_from((x, y)).unwrap();

            if enemy_counter { return true }

            if let Some(piece) = self.board[square] {
                if piece.color == color { return true }
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

        let mut king_pos = None;
        for square in SQUARE_ITER {
            if let Some(piece) = self.board[square] {
                if piece.piece == PieceTypes::King && piece.color == color {
                    king_pos = Some(square);
                }
            }
        }

        assert!(king_pos.is_some());
        let mut result = false;

        for square in SQUARE_ITER {
            if let Ok((moves, _)) = self.possible_moves(square) {
                if moves[king_pos.unwrap()].is_some() {
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

        let mut result = false;

        for square in SQUARE_ITER {
            //ignore castles since they cant be done if you are in check.
            if let Ok((moves, _)) = self.possible_moves(square) {
                for square_1 in SQUARE_ITER {
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

        false
    }

    fn possible_moves_directions(
        &mut self,
        color: Color,
        from: Square,
        possible_board: &mut Board_Move,
        directions: Vec<(i32, i32)>
    ) {
        for direction in directions {
            for i in 1..=8 {
                let x_1 = from.rank as i32 + direction.0 * i;
                let y_1 = from.rank as i32 + direction.1 * i;

                self.test_move(possible_board, x_1, y_1, from, color);
            }
        }
    }

    fn test_move(&mut self, possible_board: &mut Board_Move, x: i32, y: i32, from: Square, color: Color) {
        if let Ok(square) = Square::try_from((x, y)) {
            if !self.collision_check(square, color) {
                let test_move = Move::Normal {
                    from,
                    to: square,
                };
                if !self.check_check(test_move, self.turn) {
                    possible_board[square] = Some(test_move);
                }
            }
        }
    }

    /// returns a board with all the normal moves possible for the piece in the from Square.
    /// potential Castelmoves are returned in a seperate Vec.
    pub fn possible_moves(&mut self, from: Square) -> Result<(Board_Move, Vec<Move>), Error> {
        let piece = self.board[from].map_or(Err(Error::EmptySquare), |p| Ok(p))?;
        let mut possible_board = Board_Move::from([[None; 8]; 8]);
        let mut castles: Vec<Move> = vec![];

        let straight_directions: Vec<(i32, i32)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
        let diagonal_directions: Vec<(i32, i32)> = vec![(1, 1), (-1, 1), (-1, -1), (1, -1)];
        match piece.piece {
            PieceTypes::Knight => {
                for direction in straight_directions {
                    let x_temp: i32 = direction.0 * 2 + from.file as i32;
                    let y_temp: i32 = direction.1 * 2 + from.rank as i32;

                    let x_1 = x_temp + direction.1;
                    let y_1 = y_temp + direction.0;
                    self.test_move(&mut possible_board, x_1, y_1, from, piece.color);

                    let x_2 = x_temp - direction.1;
                    let y_2 = y_temp - direction.0;
                    self.test_move(&mut possible_board, x_2, y_2, from, piece.color);
                }
            },
            PieceTypes::King => {
                for direction in [straight_directions, diagonal_directions].concat() {
                    let x_1 = from.file as i32 + direction.0;
                    let y_1 = from.rank as i32 + direction.1;

                    self.test_move(&mut possible_board, x_1, y_1, from, piece.color);
                }
            },
            PieceTypes::Bishop => {
                self.possible_moves_directions(piece.color, from, &mut possible_board, diagonal_directions);
            },
            PieceTypes::Rook => {
                self.possible_moves_directions(piece.color, from, &mut possible_board, straight_directions);

                let (home_row, possible_castles): (Rank, _) = match piece.color {
                    Color::White => (Rank::R1, &self.castling.white),
                    Color::Black => (Rank::R8, &self.castling.black),
                };

                if let (Some(CastlingSide::KingSide), ..) = possible_castles {
                    'label: {
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
                self.possible_moves_directions(piece.color, from, &mut possible_board, [straight_directions, diagonal_directions].concat());
            },
            PieceTypes::Pawn(moved) => {
                let movement_direction: i32 = match piece.color {
                    Color::Black => -1,
                    Color::White => 1,
                };

                //TODO: add en passant
                //checks one or two moves forward
                let y_1: i32 = from.rank as i32 + movement_direction;
                self.test_move(&mut possible_board, from.rank.into(), y_1, from, piece.color);

                if !moved {
                    let y_2: i32 = from.rank as i32 + movement_direction * 2;
                    self.test_move(&mut possible_board, from.rank.into(), y_2, from, piece.color);
                }

                //checks for enemy's in the diagonal
                let x_3 = from.file as i32 + 1;
                self.test_move(&mut possible_board, x_3, y_1, from, piece.color);

                let x_4 = from.file as i32 - 1;
                self.test_move(&mut possible_board, x_4, y_1, from, piece.color);
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
pub enum Error {
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

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error")
    }
}

impl std::error::Error for Error { }

//TODO: add tests
#[cfg(test)]
mod tests {
    use pgn_reader::{Visitor, Skip, BufferedReader, SanPlus, San, File, Rank, Square};
    use crate::{ Game, Error, Color, Piece, PieceTypes };
    use shakmaty::{ Chess, Position, Move };

    use reqwest;
    use tokio;
    use std::{io::Read, env};

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
        moves: Vec<(Move)>,
        chess: Chess,
    }

    impl TestGame {
        fn new() -> TestGame {
            TestGame {
                moves: vec![],
                chess: Chess::default()
            }
        }
    }

    impl Visitor for TestGame {
        type Result = Vec<crate::Move>;

        fn begin_game(&mut self) {
            self.moves = vec![];
            self.chess = Chess::default();
        }

        fn san(&mut self, san_plus: SanPlus) {
            let mov = san_plus.san
                .to_move(&self.chess)
                .unwrap();

            match mov {
                shakmaty::Move::Normal { from, to, .. } => {
                    self.moves.push(Move::Normal {
                        from: Square {
                            file: file::from(i32::from(from.file())),
                            rank: rank::from(i32::from(from.rank()))
                        },
                        to: Square {
                            file: file::from(i32::from(to.file())),
                            rank: rank::from(i32::from(to.rank()))
                        });
                }
            }

            self.chess = self.chess.clone().play(&mov).unwrap();
        }

        fn begin_variation(&mut self) -> Skip {
            Skip(true) // stay in the mainline
        }

        fn end_game(&mut self) -> Self::Result {
            self.moves.clone()
        }
    }

    #[tokio::test]
    async fn database_games_test() {
        env::set_var("RUST_BACKTRACE", "1");
        /*let pgn = b"1. e4 e5 2. Nf3 (2. f4)
            { game paused due to bad weather }
            2... Nf6 *";*/
        let mut r = Ok(());

        println!("1");

        for i in 0..10 {
            let res = reqwest::get("https://lichess.org/game/export/TJxUmbWK")
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            let pgn = res.as_bytes();
            println!("{}", res);


            let mut reader = BufferedReader::new_cursor(&pgn[..]);

            let mut test_game = TestGame::new();
            let moves = match reader.read_game(&mut test_game).unwrap()
            {
                Some(m) => m,
                None => vec![],
            };

            /* let move_list = [
            [0, 1, 0, 2], //Pawn
            [0, 0, 0, 1], //Rook
            [1, 0, 2, 2], //Knight
            [2, 0, 4, 2], //bishop
            [3, 0, 2, 0], //Queen
            [4, 0, 3, 0], //King
            //[3, 0, 3, 0], //King
            ];*/

            let mut game = Game::new();

            println!("{:?}", moves);

            for row in moves {
                let from.file = row[0];
                let from.rank = row[1];
                let to.file = row[2];
                let to.rank = row[3];

                println!("moving {}, from {from.file}, {from.rank} to {to.file}, {to.rank}", match game.board[from] {
                    Some(Piece { piece: PieceTypes::Bishop, .. }) => "b",
                    Some(Piece { piece: PieceTypes::Rook, .. }) => "r",
                    Some(Piece { piece: PieceTypes::King, .. }) => "K",
                    Some(Piece { piece: PieceTypes::Queen, .. }) => "q",
                    Some(Piece { piece: PieceTypes::Knight, .. }) => "k",
                    Some(Piece { piece: PieceTypes::Pawn(_), .. }) => "p",
                    None => ".",
                });

                let win = game.try_move(from.file, from.rank, to.file, to.rank).unwrap();

                println!("moved {}", match game.board[to] {
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
                    println!("moved {}, {} to {}, {} and {} won the game", from.file, from.rank, to.file, to.rank, winner);
                }
                else {
                    println!("moved {}, {} to {}, {}", from.file, from.rank, to.file, to.rank);
                }

                println!(" 01234567");
                let mut y_cord: usize = 0;
                for y in game.board {
                    let mut t = String::new();
                    let mut x_cord: usize = 0;
                    for x in y {
                        /*if x_cord == from.file && y_cord == from.rank {
                            //t += "*";
                            x_cord += 1;
                            continue;
                        }
                        if x_cord == to.file && y_cord == to.rank {
                            //t += "#";
                            x_cord += 1;
                            //continue;
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
                    println!("{y_cord}{}",t);

                    y_cord += 1;
                }
            }
        }
    }
}