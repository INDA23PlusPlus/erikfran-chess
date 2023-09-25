# Chess
A backend library for chess
### API description
#### The Game instance
The game instance is the main object of the library. It contains the board and the current state of the game. It can be created with the new function.
```rust
let mut game = Game::new();
```
you can access all the state of a game in the public fields. For example from the CLI example I print the of the state of the game like so:
```rust
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
```

#### The Board
The board works like a 2d array of pieces. It can be index either by [Rank][File] or by [Square] like so:
```rust
let piece = game.board[Rank::One][File::A];
```
Every square is an option, None meaning empty square, of a Piece struct that contains the role of the piece and the color of the piece. For example in the CLI  example I print the board like so:
```rust
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
```
observe how RANK_ARRAY, FILE_ARRAY and get_square_array can be used to iterate over all possible Rank, File or squares.

#### The possible_moves function
The possible_moves function returns a board with Options<> of moves instead och pieces. The intended use for this is to first generate the posssible moves for a piece and then take one of those possible moves and pass it to the try_move function. The possible_moves is called like this
```rust
from = Square { file: File::A, rank: Rank::R1 };
let possible_moves = game.possible_moves(from);
```

#### The try_move function
To make a move you can use the try_move function. It takes a move mutates the game instance and returns a Result. If the move is legal it returns Ok(()) otherwise it returns an error of type Move error which implements Display with explenations of the errors. The move struct is used to describe a move. It contains the start and end position of the move in the Normal variant or castling side in the Castling variant. The start and end squares are represented by the Square struct that contains a file and a rank. File, Rank and Square all implement from<i32> or from<(i32, i32)> respectively. However ideally you should not have to construct your own move since it it can be taken from the possible_moves function. A simple move would look like this:
```rust
from = Square { file: File::E, rank: Rank::R2 };
let possible_moves = game.possible_moves(from);
match try_move(possible_moves[Rank::R4][File::E]) {
    Ok(_) => println!("Move was legal"),
    Err(e) => println!("Move was illegal: {}", e),
}
```
### Features
 - Basic moments
 - Castling
 - Checkmate detection(glitch)
 - legal move generation
### Problems
The possible_moves function is very broken and also the checkmate functionality since it relies on it. It does however seem playable 
### Example
A CLI implementation is available in the examples folder. it can be run with "cargo run --example cli"

