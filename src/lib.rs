#![feature(let_chains)]

use std::{usize, fmt::Display, ops::Index};

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

    /**
    Tries making move described by coordinates. If unsuccessful no move will be made and you can try making a leagal move again. 
    The return vale is an option that shows if a player has won this turn and in that case which color won.
    */
    pub fn try_move(&mut self, x_origin: usize, y_origin: usize, x_destination: usize, y_destination: usize) -> std::result::Result<Option<Color>, Error> {
        let origin = match self.board[y_origin][x_origin] {
            Some( Piece { color: c, .. } ) if c != self.turn => 
            return Err(Error::OpponentPiece),
            Some(o) => o,
            None => return Err(Error::EmptySquare),
        };

        let captured = match self.board[y_destination][x_destination] {
            Some(Piece { color: c, .. }) if c == origin.color => 
            return Err(Error::Collision),
            destination @ _ => destination
        };

        
        //TODO: return errors if moves collide, are not possible for the specific piece or lead to check.
        match origin.piece {
            PieceTypes::Bishop => {
                if x_origin.abs_diff(x_destination) != y_origin.abs_diff(y_destination) {
                    return Err(Error::WrongPieceMovement);
                }
                else if self.collision_check_line(x_origin, y_origin, x_destination, y_destination, self.turn).is_some() {
                    return Err(Error::Collision);
                }
            },
            PieceTypes::King => {
                if !((x_origin).abs_diff(x_destination) <= 1 && (y_origin).abs_diff(y_destination) <= 1) {
                    return Err(Error::WrongPieceMovement);
                }
                //casteling
            },
            PieceTypes::Queen => {
                if x_origin.abs_diff(x_destination) == y_origin.abs_diff(y_destination) || 
                (x_origin.abs_diff(x_destination) > 0) ^ (y_origin.abs_diff(y_destination) > 0) {
                    if self.collision_check_line(x_origin, y_origin, x_destination, y_destination, self.turn).is_some() {
                        return Err(Error::Collision);
                    }
                } else {
                    return Err(Error::WrongPieceMovement);
                }
            },
            PieceTypes::Rook => {
                if !((x_origin.abs_diff(x_destination) > 0) ^ (y_origin.abs_diff(y_destination) > 0)){
                    return Err(Error::WrongPieceMovement);
                }
                else if self.collision_check_line(x_origin, y_origin, x_destination, y_destination, self.turn).is_some() {
                    return Err(Error::Collision);
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
                    return Err(Error::WrongPieceMovement);
                }
                //checks for en passant
                else if y_destination.abs_diff(y_origin) == 2 {
                    if moved { return Err(Error::PawnDubbleMove); }
                    if self.collision_check_line(x_origin, y_origin, x_destination, y_destination, self.turn).is_some() {
                        return Err(Error::Collision);
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
                    return Err(Error::WrongPieceMovement);
                }
            },
        }

        self.board[y_destination][x_destination] = Some(origin);
        self.board[y_origin][x_origin] = None;

        if let Some(c) = captured {
            self.captured.push(c);
        }

        if self.checkmate_check(x_origin, y_origin, x_destination, y_destination, self.turn) {
            return Ok(Some(self.turn));
        }
        if self.checkmate_check(x_origin, y_origin, x_destination, y_destination, self.turn.opposite()) {
            return Ok(Some(self.turn.opposite()));
        }
        //TODO: Check for checkmate and checks
        //TODO: fifty move rule
        //TODO: make draw possible

        Ok(None)
    }

    ///checks for collisions with pieces of the color color or multiple collisions with the other color in a line from origin to destination. Returns the sqaure where the piece cant go becasuse of the collison. Can input out of bounds cords and it will stop at the edge 
    fn collision_check_line(&self, x_origin: usize, y_origin: usize, x_destination: usize, y_destination: usize, color: Color) -> Option<(usize, usize)> {
        let direction = (
            (x_destination as i32 - x_origin as i32).signum(), 
            (y_destination as i32 - y_origin as i32).signum()
        );

        let range = 
            if direction.0 != 0 { x_origin.abs_diff(x_destination) as i32 }
            else { y_origin.abs_diff(y_destination) as i32 };

        /*
        if a enamy is encountered a collison should only be reported the next square 
        if that exsist therfore we use enemy counter to keep track of that has happened.
        */
        let mut enemy_counter = false;
        for i in 1..=range {
            let x: usize = (x_origin as i32 + direction.0 * i) as usize;
            let y: usize = (y_origin as i32 + direction.1 * i) as usize;

            if (x > 7 || x < 0) || (y > 7 || y < 0) { return Some((x, y)); }

            if enemy_counter { return Some((x, y)) }

            if let Some(piece) = self.board[y][x] { 
                if piece.color == color { return Some((x, y)); }
                else { enemy_counter = true; }
            }
        }
        None
    }

    ///checks for collision for a piece of color color in the position x, y.
    fn collision_check(&self, x: usize, y: usize, color: Color) -> bool {
        match self.board[y][x] {
            Some(Piece { color: c, .. }) if c == color => true,
            _ => false
        }
    }

    ///the color is who is in check. Will just panic if its passed a invalid move 
    fn check_check(&self, x_origin: usize, y_origin: usize, x_destination: usize, y_destination: usize, color: Color) -> bool {
        let mut test_board = self.board.clone();

        test_board[y_destination][x_destination] = Some(self.board[y_origin][x_origin].unwrap());
        test_board[y_origin][x_origin] = None;

        let mut king_pos: (usize, usize) = (9, 9);
        for x in 0..8 {
            for y in 0..8 {
                if let Some(piece) = test_board[y][x] {
                    if piece.piece == PieceTypes::King && piece.color == color {
                        king_pos = (x, y);
                    }
                }
            }
        }

        if king_pos == (9, 9) { panic!() }

        for x in 0..8 {
            for y in 0..8 {
                if let Ok(moves) = self.possible_moves(x, y, false) {
                    if moves[king_pos.1][king_pos.0] { return true; }
                }
            }
        }
        
        false
    }

    ///the color is who is in check. Will just panic if its passed a invalid move 
    fn checkmate_check(&self, x_origin: usize, y_origin: usize, x_destination: usize, y_destination: usize, color: Color) -> bool {
        let mut test_board = self.board.clone();

        test_board[y_destination][x_destination] = Some(self.board[y_origin][x_origin].unwrap());
        test_board[y_origin][x_origin] = None;
        
        if !self.check_check(x_origin, y_origin, x_destination, y_destination, color) {
            return false;
        }

        for x in 0..8 {
            for y in 0..8 {
                if let Ok(moves) = self.possible_moves(x, y, false) {
                    for x_1 in 0..8 {
                        for y_1 in 0..8 {
                            if moves[y_1][x_1] {
                                if !self.check_check(x, y, x_1, y_1, color) {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }

        false
    }

    fn possible_moves_directions(
        &self, 
        color: Color,
        x: usize, 
        y: usize, 
        check_check: bool, 
        possible_board: &mut [[bool; 8]; 8], 
        directions: Vec<(i32, i32)>
    ) {
        for direction in directions {
            let x_temp = (direction.0 * 8 + y as i32) as usize;
            let y_temp = (direction.1 * 8 + x as i32) as usize;

            //will collide since we are going out of bounds so we can use unwrap.
            let cords = self.collision_check_line(x, y, x_temp, y_temp, color).unwrap();
            
            
            let range = 
                if direction.0 != 0 { x.abs_diff(cords.0) as i32 }
                else { y.abs_diff(cords.1) as i32 };

            for i in 1..=range {
                let x_1: usize = (x as i32 + direction.0 * i) as usize;
                let y_1: usize = (y as i32 + direction.1 * i) as usize;

                possible_board[y_1][x_1] = if check_check {
                    !self.check_check(x, y, x_1, y_1, color)
                } else { true }
            } 
        }
    }

    /// check_check is true if you want to exclude moves resulting in checks so should always be true for gui related usage
    pub fn possible_moves(&self, x: usize, y: usize, check_check: bool) -> std::result::Result<[[bool; 8]; 8], Error> {
        if let Some(piece) = self.board[y][x] {
            let mut possible_board = [[false; 8]; 8];

            let straight_directions = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
            let mut diagonal_directions = vec![(1, 1), (-1, 1), (-1, -1), (1, -1)];

            match piece.piece {
                PieceTypes::Knight => {
                    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];

                    for direction in directions {
                        let x_temp = direction.0 * 2 + y as i32;
                        let y_temp = direction.1 * 2 + x as i32;

                        let x_1 = (y_temp + direction.0) as usize;
                        let y_1 = (x_temp + direction.1) as usize;
                        possible_board[x_1][y_1] = !self.collision_check(x_1, y_1, piece.color);

                        let x_2 = (y_temp - direction.0) as usize;
                        let y_2 = (x_temp - direction.1) as usize;
                        possible_board[x_2][y_2] = !self.collision_check(x_2, y_2, piece.color);
                    }
                },
                PieceTypes::King => {
                    todo!();
                },
                PieceTypes::Bishop => {
                    self.possible_moves_directions(piece.color, x, y, check_check, &mut possible_board, diagonal_directions);
                },
                PieceTypes::Rook => {
                    self.possible_moves_directions(piece.color, x, y, check_check, &mut possible_board, straight_directions);
                },
                PieceTypes::Queen => {
                    self.possible_moves_directions(piece.color, x, y, check_check, &mut possible_board, [straight_directions, diagonal_directions].concat());
                },
                PieceTypes::Pawn(moved) => {
                    let movement_direction: i32 = match piece.color {
                        Color::Black => -1,
                        Color::White => 1,
                    };
                    
                    //TODO: add en passant
                    //checks one or two moves forward
                    let y_1 = (y as i32 + movement_direction) as usize;
                    if y_1 < 8 { 
                        possible_board[y_1][x] = 
                            !self.collision_check(x, y_1 as usize, piece.color);
                    }
                    
                    let y_2 = (y as i32 + movement_direction * 2) as usize;
                    if y_2 < 8 && !moved { 
                        possible_board[y_2][x] = 
                            !self.collision_check(x, y_2 as usize, piece.color); 
                    }
                    
                    //checks for enemys
                    let x_3 = x + 1;
                    possible_board[x_3][y_1] = 
                        self.collision_check(x_3, y_1, piece.color.opposite());
                    
                    let x_4 = y - 1;
                    possible_board[x_4][y_1] = 
                        self.collision_check(x_4, y_1, piece.color.opposite());
                }
            }
            Ok(possible_board)
        }
        else { Err(Error::EmptySquare) }
    }

    pub fn translate_move_to_chessnotation(&self, x_origin: usize, y_origin: usize, x_destination: usize, y_destination: usize) -> &str {
        todo!()
    }

    pub fn translate_move_from_chessnotation(input: &str) -> (usize, usize, usize, usize) {
        todo!()
    }

    pub fn translate_game_from_fen(input: &str) -> Game {
        todo!()
    }

    pub fn translate_game_to_fen(&self) -> &str {
        todo!()
    }
}

//TODO read how to and then make a proper error implementation with std::error::Error.
#[derive(Debug)]
pub enum Error {
    OpponentPiece,
    ///trying to access an empty square
    EmptySquare,
    WrongPieceMovement,
    Collision,
    PawnDubbleMove,
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

    use reqwest;
    use tokio;  
    use std::io::Read;

    struct TestGame {
        moves: Vec<[usize; 4]>,
    }

    impl TestGame {
        fn new() -> TestGame {
            TestGame {
                moves: vec![]
            }
        }
    }

    impl Visitor for TestGame {
        type Result = Vec<[usize; 4]>;

        fn begin_game(&mut self) {
            self.moves = vec![];
        }

        fn san(&mut self, san_plus: SanPlus) {
            println!("works2");
            if let San::Normal { file, rank, to, .. } = san_plus.san {
                println!("{:?}, {:?}", rank, file);
                if let Some(file2) = file {
                    if let Some(rank2) = rank {
                        println!("works");
                        self.moves.push([
                            usize::from(file2), 
                            usize::from(rank2), 
                            usize::from(to.file()), 
                            usize::from(to.rank())
                            ]);
                        }
                }
            }
        }

        fn begin_variation(&mut self) -> Skip {
            Skip(true) // stay in the mainline
        }

        fn end_game(&mut self) -> Self::Result {
            self.moves.clone()
        }
    }

    #[tokio::test]
    async fn database_games_test() -> Result<(), Error> {
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
            let moves = match reader.read_game(&mut test_game)
                .map_err(|_| Error::None)? 
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
        }

        r
    }
}