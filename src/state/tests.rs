#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn board_make() {
        let board = Board {
            board: vec![vec![false; 2]; 3],
            size: (3, 2),
        };
        assert_eq!(board, Board::make(3, 2))
    }

    #[test]
    fn board_next_empty() {
        let board = Board {
            board: vec![vec![false; 2]; 3],
            size: (3, 2),
        };
        assert_eq!(board, board.next())
    }

    #[test]
    fn board_next_alive_none() {
        let mut b = vec![vec![false; 2]; 3];
        b[0][0] = true;
        let board_test = Board {
            board: b,
            size: (3, 2),
        };
        let board_empty = Board {
            board: vec![vec![false; 2]; 3],
            size: (3, 2),
        };

        assert_eq!(board_empty, board_test.next())
    }

    #[test]
    fn board_next_alive_one() {
        let mut b = vec![vec![false; 2]; 3];
        b[0][0] = true;
        b[0][1] = true;
        let board_test = Board {
            board: b,
            size: (3, 2),
        };
        let board_empty = Board {
            board: vec![vec![false; 2]; 3],
            size: (3, 2),
        };

        assert_eq!(board_empty, board_test.next())
    }

    #[test]
    fn board_next_alive_two() {
        let mut b = vec![vec![false; 2]; 3];
        b[0][0] = true;
        b[1][0] = true;
        b[2][0] = true;
        let board_test = Board {
            board: b,
            size: (3, 2),
        };
        let mut b = vec![vec![false; 2]; 3];
        b[1][0] = true;
        b[1][1] = true;
        let board_desired = Board {
            board: b,
            size: (3, 2),
        };

        assert_eq!(
            board_desired,
            board_test.next(),
            "desired: {:?}, actual: {:?}",
            board_desired,
            board_test.next()
        )
    }

    #[test]
    fn board_next_alive_three() {
        let mut b = vec![vec![false; 2]; 3];
        b[0][0] = true;
        b[1][0] = true;
        b[2][0] = true;
        b[1][1] = true;
        let board_test = Board {
            board: b,
            size: (3, 2),
        };
        let b = vec![vec![true; 2]; 3];
        let board_desired = Board {
            board: b,
            size: (3, 2),
        };

        assert_eq!(
            board_desired,
            board_test.next(),
            "desired: {:?}, actual: {:?}",
            board_desired,
            board_test.next()
        )
    }

    #[test]
    fn board_next_overcrowd() {
        let b = vec![vec![true; 3]; 3];
        let board_test = Board {
            board: b,
            size: (3, 3),
        };
        let mut b = vec![vec![false; 3]; 3];
        b[0][0] = true;
        b[2][0] = true;
        b[0][2] = true;
        b[2][2] = true;
        let board_desired = Board {
            board: b,
            size: (3, 3),
        };

        assert_eq!(
            board_desired,
            board_test.next(),
            "desired: {:?}, actual: {:?}",
            board_desired,
            board_test.next()
        )
    }

    #[test]
    fn board_mut_next_empty() {
        let board = Board {
            board: vec![vec![false; 2]; 3],
            size: (3, 2),
        };
        let mut res = board.clone();
        res.next_mut_inplace();
        assert_eq!(board, res)
    }

    #[test]
    fn board_mut_next_alive_none() {
        let mut b = vec![vec![false; 2]; 3];
        b[0][0] = true;
        let mut board_test = Board {
            board: b,
            size: (3, 2),
        };
        let board_empty = Board {
            board: vec![vec![false; 2]; 3],
            size: (3, 2),
        };

        board_test.next_mut_inplace();
        assert_eq!(board_empty, board_test)
    }

    #[test]
    fn board_mut_next_alive_one() {
        let mut b = vec![vec![false; 2]; 3];
        b[0][0] = true;
        b[0][1] = true;
        let mut board_test = Board {
            board: b,
            size: (3, 2),
        };
        board_test.next_mut_inplace();
        let board_empty = Board {
            board: vec![vec![false; 2]; 3],
            size: (3, 2),
        };

        assert_eq!(board_empty, board_test)
    }

    #[test]
    fn board_mut_next_alive_two() {
        let mut b = vec![vec![false; 2]; 3];
        b[0][0] = true;
        b[1][0] = true;
        b[2][0] = true;
        let mut board_test = Board {
            board: b,
            size: (3, 2),
        };
        let mut b = vec![vec![false; 2]; 3];
        b[1][0] = true;
        b[1][1] = true;
        let board_desired = Board {
            board: b,
            size: (3, 2),
        };
        board_test.next_mut_inplace();

        assert_eq!(
            board_desired, board_test,
            "desired: {:?}, actual: {:?}",
            board_desired, board_test
        )
    }

    #[test]
    fn board_mut_next_alive_three() {
        let mut b = vec![vec![false; 2]; 3];
        b[0][0] = true;
        b[1][0] = true;
        b[2][0] = true;
        b[1][1] = true;
        let mut board_test = Board {
            board: b,
            size: (3, 2),
        };
        let b = vec![vec![true; 2]; 3];
        let board_desired = Board {
            board: b,
            size: (3, 2),
        };
        board_test.next_mut_inplace();
        assert_eq!(
            board_desired, board_test,
            "desired: {:?}, actual: {:?}",
            board_desired, board_test
        )
    }

    #[test]
    fn board_mut_next_overcrowd() {
        let b = vec![vec![true; 3]; 3];
        let mut board_test = Board {
            board: b,
            size: (3, 3),
        };
        let mut b = vec![vec![false; 3]; 3];
        b[0][0] = true;
        b[2][0] = true;
        b[0][2] = true;
        b[2][2] = true;
        let board_desired = Board {
            board: b,
            size: (3, 3),
        };
        board_test.next_mut_inplace();

        assert_eq!(
            board_desired, board_test,
            "desired: {:?}, actual: {:?}",
            board_desired, board_test
        )
    }

    #[test]
    fn board_format() {
        let mut b = vec![vec![false; 3]; 3];
        b[0][0] = true;
        b[2][0] = true;
        b[0][2] = true;
        b[2][2] = true;
        let board = Board {
            board: b,
            size: (3, 3),
        };

        assert_eq!(
            "■ ■\n   \n■ ■",
            format!("{}", board),
            "expected {} but got {}",
            "■ ■\n   \n■ ■",
            format!("{}", board)
        )
    }
}
