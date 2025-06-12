/// A square on the chess board, indexing starts with 0 at A1, 1 at B1 and ends with 63 at H8.
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct Square(u8);

impl Square {
    pub const fn new(value: u8) -> Self {
        Self(value)
    }

    pub fn from_file_rank(file: u8, rank: u8) -> Self {
        Self((file - 1) + (rank - 1) * 8)
    }

    pub fn is_white(&self) -> bool {
        (self.get_file() + self.get_rank()) % 2 == 0
    }

    pub fn get_value(&self) -> u8 {
        self.0
    }

    pub fn get_rank(&self) -> u8 {
        (self.0 / 8) + 1
    }

    pub fn get_file(&self) -> u8 {
        (self.0 % 8) + 1
    }

    pub fn is_left_edge(&self) -> bool {
        self.0 % 8 == 0
    }

    pub fn is_right_edge(&self) -> bool {
        self.0 % 8 == 7
    }

    pub fn is_upper_edge(&self) -> bool {
        self.0 >= 56
    }

    pub fn is_lower_edge(&self) -> bool {
        self.0 <= 7
    }

    pub fn index_left(&self) -> Option<u8> {
        if self.is_left_edge() {
            None
        } else {
            Some(self.0 - 1)
        }
    }

    pub fn index_right(&self) -> Option<u8> {
        if self.is_right_edge() {
            None
        } else {
            Some(self.0 + 1)
        }
    }

    pub fn index_up(&self) -> Option<u8> {
        if self.is_upper_edge() {
            None
        } else {
            Some(self.0 + 8)
        }
    }

    pub fn index_down(&self) -> Option<u8> {
        if self.is_lower_edge() {
            None
        } else {
            Some(self.0 - 8)
        }
    }

    pub fn index_up_right(&self) -> Option<u8> {
        if self.is_upper_edge() || self.is_right_edge() {
            None
        } else {
            Some(self.0 + 9)
        }
    }

    pub fn index_up_left(&self) -> Option<u8> {
        if self.is_upper_edge() || self.is_left_edge() {
            None
        } else {
            Some(self.0 + 7)
        }
    }

    pub fn index_down_right(&self) -> Option<u8> {
        if self.is_lower_edge() || self.is_right_edge() {
            None
        } else {
            Some(self.0 - 7)
        }
    }

    pub fn index_down_left(&self) -> Option<u8> {
        if self.is_lower_edge() || self.is_left_edge() {
            None
        } else {
            Some(self.0 - 9)
        }
    }

    pub fn index_jump(&self, file: i8, rank: i8) -> Option<u8> {
        let current_file = self.get_file();
        let current_rank = self.get_rank();
        let new_file = current_file as i8 + file;
        let new_rank = current_rank as i8 + rank;
        if new_file < 1 || new_file > 8 || new_rank < 1 || new_rank > 8 {
            None
        } else {
            Some(Square::from_file_rank(new_file as u8, new_rank as u8).get_value())
        }
    }

    pub fn trace_up(&self) -> impl Iterator<Item = u8> {
        std::iter::successors(self.index_up(), |&idx| Square::new(idx).index_up())
    }

    pub fn trace_down(&self) -> impl Iterator<Item = u8> {
        std::iter::successors(self.index_down(), |&idx| Square::new(idx).index_down())
    }

    pub fn trace_left(&self) -> impl Iterator<Item = u8> {
        std::iter::successors(self.index_left(), |&idx| Square::new(idx).index_left())
    }

    pub fn trace_right(&self) -> impl Iterator<Item = u8> {
        std::iter::successors(self.index_right(), |&idx| Square::new(idx).index_right())
    }

    pub fn trace_up_left(&self) -> impl Iterator<Item = u8> {
        std::iter::successors(self.index_up_left(), |&idx| {
            Square::new(idx).index_up_left()
        })
    }

    pub fn trace_up_right(&self) -> impl Iterator<Item = u8> {
        std::iter::successors(self.index_up_right(), |&idx| {
            Square::new(idx).index_up_right()
        })
    }

    pub fn trace_down_left(&self) -> impl Iterator<Item = u8> {
        std::iter::successors(self.index_down_left(), |&idx| {
            Square::new(idx).index_down_left()
        })
    }

    pub fn trace_down_right(&self) -> impl Iterator<Item = u8> {
        std::iter::successors(self.index_down_right(), |&idx| {
            Square::new(idx).index_down_right()
        })
    }
}
