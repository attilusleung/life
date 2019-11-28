use rand::random;
use std::fmt;
mod tests;

#[derive(Debug, PartialEq, Clone)]
pub struct Board {
    board: Vec<Vec<bool>>,
    pub size: (usize, usize),
}

struct View<'a> {
    board: &'a Board,
    vec_view: Vec<&'a [bool]>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        for (i, vec) in self.board.iter().enumerate() {
            for elem in vec.iter() {
                write!(f, "{}", (if *elem { "■" } else { " " }))?;
            }
            if i < self.size.0 - 1 {
                write!(f, "\n")?
            }
        }
        Ok(())
    }
}

impl Board {
    pub fn make(n: i32, m: i32) -> Board {
        let (n, m) = (n as usize, m as usize);
        let board = vec![vec![false; m]; n];

        Board {
            board,
            size: (n, m),
        }
    }

    pub fn make_rand(n: i32, m: i32) -> Board {
        let mut ret = Board::make(n, m);

        for i in ret.board.iter_mut() {
            for j in i.iter_mut() {
                *j = random::<bool>();
            }
        }
        ret
    }

    pub fn next(&self) -> Board {
        let mut ret = vec![vec![false; self.size.1]; self.size.0];
        for i in 0..self.size.0 {
            for j in 0..self.size.1 {
                ret[i][j] = self.check_alive((i, j))
            }
        }
        Board {
            board: ret,
            size: self.size,
        }
    }

    pub fn next_mut_inplace(&mut self) {
        self.board = self.next().board
    }

    fn check_alive(&self, (i, j): (usize, usize)) -> bool {
        let mut count = 0;
        for p in i.checked_sub(1).unwrap_or(0)..=checked_bounds(i, 1, self.size.0).unwrap_or(i) {
            for q in j.checked_sub(1).unwrap_or(0)..=checked_bounds(j, 1, self.size.1).unwrap_or(j)
            {
                if (p, q) != (i, j) && self.board[p][q] {
                    count += 1;
                }
            }
        }
        match (self.board[i][j], count) {
            (true, 2..=3) => true,
            (false, 3) => true,
            _ => false,
        }
    }

    fn view_board(&self) -> View {
        let v = self
            .board
            .iter()
            .map(|x| x.as_slice())
            .collect::<Vec<&[bool]>>();
        (move || View {
            board: self,
            vec_view: v,
        })()
    }
}

impl<'a> View<'a> {
    fn as_slice(&self) -> &[&[bool]] {
        self.vec_view.as_slice()
    }
}

fn checked_bounds(i: usize, a: usize, j: usize) -> Option<usize> {
    match i.checked_add(a as usize) {
        None => None,
        Some(n) => {
            if n >= j as usize {
                None
            } else {
                Some(n)
            }
        }
    }
}
