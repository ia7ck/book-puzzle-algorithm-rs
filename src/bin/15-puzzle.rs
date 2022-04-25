use std::fmt;
use std::fmt::Formatter;

const N: usize = 4;

#[derive(Debug, Copy, Clone)]
struct Value(u8);

impl Value {
    // (si, sj) から正しい位置までのマンハッタン距離
    fn move_cost(self, (si, sj): (usize, usize)) -> u32 {
        assert!(self.0 > 0);
        let (ti, tj) = (usize::from(self.0 - 1) / N, usize::from(self.0 - 1) % N);
        ((si.max(ti) - si.min(ti)) + (sj.max(tj) - sj.min(tj))) as u32
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:2}", self.0)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Dir {
    R,
    U,
    L,
    D,
}

impl Dir {
    fn reverse(self) -> Self {
        match self {
            Dir::R => Dir::L,
            Dir::U => Dir::D,
            Dir::L => Dir::R,
            Dir::D => Dir::U,
        }
    }
}

type B = [[Value; N]; N];

#[derive(Debug)]
struct Board {
    board: B,
    empty: (usize, usize),
    estimate: u32,
}

impl Board {
    fn new(board: [[u8; N]; N]) -> Self {
        let mut seen = vec![false; N * N];
        for i in 0..N {
            for j in 0..N {
                assert!(board[i][j] <= 15);
                seen[usize::from(board[i][j])] = true;
            }
        }
        for v in 0..(N * N) {
            assert!(seen[v]);
        }

        let mut board_v = [[Value(0); N]; N];
        let mut empty = (0, 0);
        for i in 0..N {
            for j in 0..N {
                board_v[i][j] = Value(board[i][j]);
                if board[i][j] == 0 {
                    empty = (i, j);
                }
            }
        }
        let mut board = Self {
            board: board_v,
            empty,
            estimate: 0,
        };
        board.estimate = board.estimate_all();
        board
    }

    fn move_cost(&self, (i, j): (usize, usize)) -> u32 {
        self.board[i][j].move_cost((i, j))
    }

    fn estimate_all(&self) -> u32 {
        let mut cost = 0;
        for i in 0..N {
            for j in 0..N {
                if (i, j) == self.empty {
                    continue;
                }
                cost += self.move_cost((i, j));
            }
        }
        cost as u32
    }

    // 空きマスを dir の方向にずらす
    fn slide(&mut self, dir: Dir) -> Result<(), ()> {
        let (i, j) = self.empty;
        match dir {
            Dir::R => {
                if j + 1 >= N {
                    return Err(());
                }
                self.estimate -= self.move_cost((i, j + 1));
                self.board[i].swap(j, j + 1);
                self.empty = (i, j + 1);
                self.estimate += self.move_cost((i, j));
            }
            Dir::U => {
                if i == 0 {
                    return Err(());
                }
                self.estimate -= self.move_cost((i - 1, j));
                let val = self.board[i - 1][j];
                self.board[i - 1][j] = self.board[i][j];
                self.board[i][j] = val;
                self.empty = (i - 1, j);
                self.estimate += self.move_cost((i, j));
            }
            Dir::L => {
                if j == 0 {
                    return Err(());
                }
                self.estimate -= self.move_cost((i, j - 1));
                self.board[i].swap(j - 1, j);
                self.empty = (i, j - 1);
                self.estimate += self.move_cost((i, j));
            }
            Dir::D => {
                if i + 1 >= N {
                    return Err(());
                }
                self.estimate -= self.move_cost((i + 1, j));
                let val = self.board[i + 1][j];
                self.board[i + 1][j] = self.board[i][j];
                self.board[i][j] = val;
                self.empty = (i + 1, j);
                self.estimate += self.move_cost((i, j));
            }
        }
        Ok(())
    }

    fn board(&self) -> B {
        self.board
    }
}

fn dfs(max_depth: usize, depth: usize, board: &mut Board, pre_dir: Dir, result: &mut Vec<B>) {
    if !result.is_empty() {
        return;
    }

    if board.estimate == 0 {
        result.push(board.board());
        return;
    }

    if depth >= max_depth {
        return;
    }

    for dir in [Dir::R, Dir::U, Dir::L, Dir::D] {
        if depth >= 1 && dir.reverse() == pre_dir {
            continue;
        }
        if let Ok(()) = board.slide(dir) {
            if depth + board.estimate as usize <= max_depth {
                dfs(max_depth, depth + 1, board, dir, result);
            }
            assert!(board.slide(dir.reverse()).is_ok());
            if !result.is_empty() {
                result.push(board.board());
                return;
            }
        }
    }
}

fn main() {
    // 図3-4
    #[rustfmt::skip]
    let mut board = Board::new([
        [ 5,  4,  7,  6],
        [15,  0, 13, 10],
        [ 2,  1,  8,  3],
        [12, 14, 11,  9],
    ]);

    for max_depth in 0..80 {
        let mut result = Vec::new();
        dfs(
            max_depth,
            0,
            &mut board,
            Dir::R, // dummy
            &mut result,
        );
        if !result.is_empty() {
            result.reverse();
            for (i, board) in result.iter().enumerate() {
                println!("{} th move:", i);
                for row in board {
                    let row: Vec<String> = row.iter().map(|val| format!("{}", val)).collect();
                    println!("{}", row.join(" "));
                }
                println!();
            }
            break;
        }
    }
}
