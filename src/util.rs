use std::ops::{Index, IndexMut};
use crate::{Move, Piece};

#[derive(Clone, Copy)]
pub struct Board {
    pub rows: Rows<Rows<Option<Piece>>>,
}

impl From<[[Option<Piece>; 8]; 8]> for Board {
    fn from (value: [[Option<Piece>; 8]; 8]) -> Self {
        Board { rows: Rows { squares: [
            Rows { squares: value[0] },
            Rows { squares: value[1] },
            Rows { squares: value[2] },
            Rows { squares: value[3] },
            Rows { squares: value[4] },
            Rows { squares: value[5] },
            Rows { squares: value[6] },
            Rows { squares: value[7] },
        ] } }
    }
}

impl Index<Square> for Board {
    type Output = Option<Piece>;

    fn index(&self, square: Square) -> &Self::Output {
        &self[square.rank][square.file]
    }
}

impl IndexMut<Square> for Board {
    fn index_mut(&mut self, square: Square) -> &mut Self::Output {
        &mut self[square.rank][square.file]
    }
}

impl Index<Rank> for Board {
    type Output = Rows<Option<Piece>>;

    fn index(&self, rank: Rank) -> &Self::Output {
        &self.rows.squares[rank as usize]
    }
}

impl IndexMut<Rank> for Board {
    fn index_mut(&mut self, rank: Rank) -> &mut Self::Output {
        &mut self.rows.squares[rank as usize]
    }
}

#[derive(Clone, Copy)]
pub struct BoardMove {
    pub rows: Rows<Rows<Option<Move>>>,
}

impl From<[[Option<Move>; 8]; 8]> for BoardMove {
    fn from (value: [[Option<Move>; 8]; 8]) -> Self {
        BoardMove { rows: Rows { squares: [
            Rows { squares: value[0] },
            Rows { squares: value[1] },
            Rows { squares: value[2] },
            Rows { squares: value[3] },
            Rows { squares: value[4] },
            Rows { squares: value[5] },
            Rows { squares: value[6] },
            Rows { squares: value[7] },
        ] } }
    }
}

impl Index<Square> for BoardMove {
    type Output = Option<Move>;

    fn index(&self, square: Square) -> &Self::Output {
        &self[square.rank][square.file]
    }
}

impl IndexMut<Square> for BoardMove {
    fn index_mut(&mut self, square: Square) -> &mut Self::Output {
        &mut self[square.rank][square.file]
    }
}

impl Index<Rank> for BoardMove {
    type Output = Rows<Option<Move>>;

    fn index(&self, rank: Rank) -> &Self::Output {
        &self.rows.squares[rank as usize]
    }
}

impl IndexMut<Rank> for BoardMove {
    fn index_mut(&mut self, rank: Rank) -> &mut Self::Output {
        &mut self.rows.squares[rank as usize]
    }
}

#[derive(Clone, Copy)]
pub struct Rows<T> {
    pub squares: [T; 8],
}

impl<T> Index<File> for Rows<T> {
    type Output = T;

    fn index(&self, file: File) -> &Self::Output {
        &self.squares[file as usize]
    }
}

impl<T> IndexMut<File> for Rows<T> {
    fn index_mut(&mut self, file: File) -> &mut Self::Output {
        &mut self.squares[file as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Square {
    pub file: File,
    pub rank: Rank,
}

impl Square {
    ///the smallest of either the rank or the file distance between two squares
    pub fn abs_diff_smallest(&self, other: &Square) -> i32 {
        self.file
            .abs_diff(other.file)
            .min(self.rank.abs_diff(other.rank))
    }
}

impl TryFrom<(i32, i32)> for Square {
    type Error = ();

    fn try_from(value: (i32, i32)) -> Result<Self, Self::Error> {
        let (file, rank) = value;
        Ok(Square {
            file: File::try_from(file)?,
            rank: Rank::try_from(rank)?,
        })
    }
}

pub const FILE_ARRAY: [File; 8] = [File::A, File::B, File::C, File::D, File::E, File::F, File::G, File::H];
pub const RANK_ARRAY: [Rank; 8]  = [Rank::R1, Rank::R2, Rank::R3, Rank::R4, Rank::R5, Rank::R6, Rank::R7, Rank::R8];
pub const fn get_square_array() -> [Square; 64]{
    let mut file: usize = 0;
    let mut square_array: [Square; 64] = [Square { file: File::A, rank: Rank::R1 }; 64];
    while file < 8 {
        let mut rank: usize = 0;
        while rank < 8 {
            square_array[file * 8 + rank] = Square { file: FILE_ARRAY[file], rank: RANK_ARRAY[rank] };
            rank += 1;
        }
        file += 1;
    }

    square_array
}


#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    pub fn abs_diff(&self, other: File) -> i32 {
        ((*self as i32) - (other as i32)).abs()
    }

    pub fn num(&self) -> i32 {
        *self as i32
    }
}

impl From<File> for i32 {
    fn from(file: File) -> i32 {
        file as i32
    }
}

impl TryFrom<i32> for File {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(File::A),
            1 => Ok(File::B),
            2 => Ok(File::C),
            3 => Ok(File::D),
            4 => Ok(File::E),
            5 => Ok(File::F),
            6 => Ok(File::G),
            7 => Ok(File::H),
            _ => Err(()),
        }
    }
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rank {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
}

impl Rank {
    pub fn abs_diff(&self, other: Rank) -> i32 {
        ((*self as i32) - (other as i32)).abs()
    }

    pub fn num(&self) -> i32 {
        *self as i32
    }
}

impl From<Rank> for i32 {
    fn from(rank: Rank) -> i32 {
        rank as i32
    }
}

impl TryFrom<i32> for Rank {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Rank::R1),
            1 => Ok(Rank::R2),
            2 => Ok(Rank::R3),
            3 => Ok(Rank::R4),
            4 => Ok(Rank::R5),
            5 => Ok(Rank::R6),
            6 => Ok(Rank::R7),
            7 => Ok(Rank::R8),
            _ => Err(()),
        }
    }
}
