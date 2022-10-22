use std::io;

pub fn input(propmt: &str) -> String {
    let stdin = io::stdin();
    let mut buff = String::new();

    loop {
        print!("{}", propmt);
        io::Write::flush(&mut io::stdout()).unwrap();
        stdin.read_line(&mut buff).unwrap();
        buff = buff.trim().to_string();
        if buff.len() != 0 {
            break;
        }
        println!("Please enter something");
        buff.clear();
    }
    buff.clone()
}

#[derive(PartialEq, Clone, Copy)]
pub enum Player {
    X,
    O,
}

impl Player {
    fn other(&self) -> Player {
        match *self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }

    fn to_string(&self) -> String {
        match *self {
            Player::X => "X".to_string(),
            Player::O => "O".to_string(),
        }
    }
}

pub struct Board {
    cells: [[Option<Player>; 3]; 3],
}

impl Board {
    fn new() -> Board {
        Board {
            cells: [[None, None, None], [None, None, None], [None, None, None]],
        }
    }

    pub fn from_array(cells: [[Option<Player>; 3]; 3]) -> Board {
        Board { cells }
    }
    
    fn ask(&mut self, player: Player) {
        let row: usize;
        let col: usize;
        println!("Enter row and column (e.g. 1 2)");
        loop {
            let buff = input("> ");
            let mut iter = buff.split_whitespace();

            let row_str = iter.next().unwrap();
            let col_str = iter.next();
            if col_str.is_none() {
                println!("Please enter two numbers");
                continue;
            } else {
                match (row_str.parse::<usize>(), col_str.unwrap().parse::<usize>()) {
                    (Ok(r), Ok(c)) => {
                        if r < 1 || r > 3 || c < 1 || c > 3 {
                            println!("Please enter a number between 1 and 3");
                            continue;
                        }
                        row = r - 1;
                        col = c - 1;
                        break;
                    }
                    _ => {
                        println!("Please enter numbers");
                        continue;
                    }
                }
            }
        }
        self.set(row, col, Some(player))
    }

    fn get(&self, row: usize, col: usize) -> Option<Player> {
        self.cells[row][col]
    }

    fn set(&mut self, row: usize, col: usize, player: Option<Player>) {
        self.cells[row][col] = player;
    }

    fn is_full(&self) -> bool {
        for row in 0..3 {
            for col in 0..3 {
                if self.get(row, col).is_none() {
                    return false;
                }
            }
        }
        true
    }

    fn free_cells(&self) -> Vec<(usize, usize)> {
        let mut cells = Vec::new();
        for row in 0..3 {
            for col in 0..3 {
                if self.get(row, col).is_none() {
                    cells.push((row, col));
                }
            }
        }
        cells
    }

    fn is_winner(&self, player: Player) -> bool {
        let p = Some(player);
        for i in 0..3 {
            if self.get(i, 0) == p && self.get(i, 1) == p && self.get(i, 2) == p {
                return true;
            }
            if self.get(0, i) == p && self.get(1, i) == p && self.get(2, i) == p {
                return true;
            }
        }
        if self.get(0, 0) == p && self.get(1, 1) == p && self.get(2, 2) == p {
            return true;
        }
        if self.get(0, 2) == p && self.get(1, 1) == p && self.get(2, 0) == p {
            return true;
        }
        false
    }

    fn print(&self) {
        for row in 0..3 {
            for col in 0..3 {
                match self.get(row, col) {
                    Some(player) => print!("{} ", player.to_string()),
                    None => print!("- "),
                }
            }
            println!("");
        }
    }
}

pub fn pvp() {
    let mut board = Board::new();
    let mut player = Player::X;

    while !board.is_full() {
        board.print();
        println!("{}'s turn", player.to_string());
        board.ask(player);
        if board.is_winner(player) {
            println!("{} wins!", player.to_string());
            break;
        }
        if board.is_full() {
            println!("Draw!");
            break;
        }
        player = player.other();
    }
}


/// minimax score for given board and player
fn minmax_score(board: &Board, player: Player, opposite: bool) -> i32 {
    if board.is_winner(player) {
        return if opposite { -1 } else { 1 };
    }
    if board.is_winner(player.other()) {
        return if opposite { 3 } else { -3 };
    }
    if board.is_full() {
        return if opposite { 2 } else { -2 };
    }
    -1
}


/// Returns the best move for the player in format (row, col)
/// Recursively calls itself to find the best move
pub fn minmax(board: &mut Board, mut current_player: Player, computer: Player, _entered_recursion: bool) -> (i32, (usize, usize)) {
    current_player = current_player.other();

    let mut best_score = if board.get(1, 1).is_none() { 3 } else { -1 };
    let mut best_move = if board.get(1, 1).is_none() {
        (1, 1)
    } else {
        if let Some((row, col)) = board.free_cells().first() {
            (*row, *col)
        } else {
            return (0, (0, 0));
        }
    };

    // making all the way down to the bottom of the game field via for loops
    for row in 0..3 {
        for col in 0..3 {
            // if we have a free cell, then go try to find the best move
            if board.get(row, col).is_none() {
                // evaluate score of the move by making a move
                // then recursively calling minmax to find the move score
                // after that, undo the move and go to the next free cell
                board.set(row, col, Some(current_player));

                // here we are summing up the scores of all best moves from here
                let mut score = minmax_score(board, current_player, computer == current_player);

                // if we are at the bottom of the recursion, then we are at the end of the game
                // and we can return the score of the move
                if score != -1 {
                    board.set(row, col, None);
                    return (score, (row, col));
                }

                // if we are not at the bottom of the recursion, then we are not at the end of the game
                // and we need to find the best move from here
                score = score + minmax(board, current_player, computer, true).0;

                board.set(row, col, None);

                if score > best_score {
                    best_score = score;
                    best_move = (row, col);
                }
            }
        }
    }
    (best_score, best_move)
}


pub fn pve() {
    let mut board = Board::new();
    let mut player = Player::X;
    let computer: Player;

    // let user choose if he wants to be X or O
    println!("Choose your player: X or O");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "X" => {
                computer = Player::O;
                break;
            },
            "O" => {
                computer = Player::X;
                break;
            },
            _ => println!("Invalid input"),
        }
    }

    while !board.is_full() {
        if board.is_winner(player) {
            println!("{} wins!", player.to_string());
            return;
        }
        board.print();
        println!("{}'s turn", player.to_string());
        if player != computer {
            board.ask(player);
        } else {
            let (_, (row, col)) = minmax(&mut board, player, player, false);
            board.set(row, col, Some(player));
        }
        player = player.other();
    }
    println!("Draw!");
}