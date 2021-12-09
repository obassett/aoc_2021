#[derive(Debug, Clone, Copy)]
struct BoardEntry {
    value: i32,
    marked: bool,
}

impl BoardEntry {
    pub fn new(value: i32) -> Self {
        Self {
            value: value,
            marked: false,
        }
    }
    pub fn mark(&mut self) {
        self.marked = true;
    }

    pub fn is_marked(&self) -> bool {
        self.marked
    }
}

#[derive(Debug, Clone, Copy)]
struct BoardRow {
    entries: [BoardEntry; 5], // Should only contain 5 entries per row
}

impl BoardRow {
    fn new(entries: [BoardEntry; 5]) -> Self {
        Self { entries: entries }
    }
}

#[derive(Debug, Clone, Copy)]
struct GameBoard {
    rows: [BoardRow; 5],
    won: bool,
    winning_number: i32,
}

impl GameBoard {
    fn new(rows: [BoardRow; 5]) -> Self {
        Self {
            rows: rows,
            won: false,
            winning_number: 0,
        }
    }

    // Marks number on the board)
    fn call_number(&mut self, number: i32) {
        if !self.won {
            for row in &mut self.rows {
                for entry in &mut row.entries {
                    if entry.value == number {
                        entry.mark();
                    }
                }
            }
        }
    }

    fn has_won(&mut self, number: i32) -> bool {
        if self.won {
            return true;
        }

        let mut win_state: bool = false;
        // Check Columns - returns true if there are 5 marked numbers in the column
        for i in 0..5 {
            let mut marked_count = 0;
            for row in &self.rows {
                if row.entries[i].is_marked() {
                    marked_count += 1;
                }
            }
            if marked_count == 5 {
                win_state = true;
            }
        }

        // Check Rows - returns true if there are 5 marked numbers in at least one row or returns false
        if !win_state {
            win_state = &self
                .rows
                .iter()
                .filter(|row| row.entries.iter().filter(|entry| entry.is_marked()).count() == 5)
                .count()
                >= &1;
        }

        if win_state {
            self.won = win_state;
            self.winning_number = number;
        }

        return self.won;
    }

    // Sum unmarked numbers * last drawn number
    fn calculate_score(&self) -> i32 {
        let mut unmarked_tally = 0;
        for row in &self.rows {
            for entry in &row.entries {
                if !entry.is_marked() {
                    unmarked_tally += entry.value;
                }
            }
        }
        unmarked_tally * self.winning_number
    }
}

struct Game {
    boards: Vec<GameBoard>,
    numbers_to_call: Vec<i32>,
}

impl Game {
    fn new(boards: Vec<GameBoard>, numbers_to_call: Vec<i32>) -> Self {
        Self {
            boards: boards,
            numbers_to_call: numbers_to_call,
        }
    }
}

pub fn part2(input: &str) -> Option<i32> {
    // Process Input into Numbers to be drawn and boards
    let game = parse_input(input);
    let mut indexes_to_remove: Vec<usize> = Vec::new();
    let mut last_board_to_win: Option<GameBoard> = None;

    match game {
        Ok(mut game) => {
            // Call Numbers
            for number in game.numbers_to_call {
                for (i, board) in &mut game.boards.iter_mut().enumerate() {
                    // Mark Boards
                    board.call_number(number);
                    // Check for Victory
                    if board.has_won(number) {
                        // If victory calculate and return score - Sum unmarked numbers * last drawn number
                        last_board_to_win = Some(board.clone());
                        indexes_to_remove.push(i.to_owned());
                    }
                }
                // Remove winning boards
                for index in indexes_to_remove.iter().rev() {
                    // println!("Should Remove this index{:?}", index);
                    // println!("Which is this board{:?}", game.boards[*index]);
                    game.boards.remove(*index);
                }
                indexes_to_remove.clear();
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    // If no victory return None
    match last_board_to_win {
        Some(board) => Some(board.calculate_score()),
        None => None,
    }
}

pub fn part1(input: &str) -> Option<i32> {
    // Process Input into Numbers to be drawn and boards
    let game = parse_input(input);

    match game {
        Ok(mut game) => {
            // Call Numbers
            for number in game.numbers_to_call {
                for board in &mut game.boards {
                    // Mark Boards
                    board.call_number(number);
                    // Check for Victory
                    if board.has_won(number) {
                        // If victory calculate and return score - Sum unmarked numbers * last drawn number
                        return Some(board.calculate_score());
                    }
                }
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    // If no victory return None
    None
}

fn parse_input(input: &str) -> Result<Game, &str> {
    // line 1 -> numbers to be called.
    // line 3-8 -> board rows
    // line 10-15 -> board rows etc
    // let mut numbers_to_call: Vec<i32> = Vec::new();
    let mut numbers_to_call: Vec<i32> = Vec::new();
    let mut temp_games_holder: Vec<GameBoard> = Vec::new();
    let mut temp_board_holder: Vec<BoardRow> = Vec::new();

    for (i, line) in input.lines().enumerate() {
        if i == 0 {
            // Numbers to be called
            numbers_to_call = line.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
        } else {
            if line.len() == 0 {
                // Board Seperator
                match temp_board_holder.len() {
                    0 => {}
                    1..=4 => Err("Board is not full")?,
                    5 => {
                        temp_games_holder.push(GameBoard::new(
                            temp_board_holder.clone().try_into().unwrap(),
                        ));
                        temp_board_holder.clear();
                    }
                    _ => {
                        unreachable!()
                    }
                }
            } else {
                // Board row
                let mut temp_row_holder = Vec::new();

                line.trim()
                    .split(" ")
                    .map(|s| s.trim().parse::<i32>())
                    .for_each(|number| {
                        match number {
                            Ok(number) => {
                                temp_row_holder.push(BoardEntry::new(number));
                            }
                            Err(_e) => {}
                        }
                        // temp_row_holder.push(BoardEntry::new(number));
                    });
                temp_board_holder.push(BoardRow::new(temp_row_holder.try_into().unwrap()));
            }
        }
    }
    // Handle last board
    match temp_board_holder.len() {
        0 => {}
        5 => {
            temp_games_holder.push(GameBoard::new(
                temp_board_holder.clone().try_into().unwrap(),
            ));
            temp_board_holder.clear();
        }
        _ => Err("Board is not full")?,
    }

    let game = Game::new(temp_games_holder, numbers_to_call);
    return Ok(game);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: String = String::from(
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\
            \n\
            22 13 17 11  0\n\
             8  2 23  4 24\n\
            21  9 14 16  7\n\
             6 10  3 18  5\n\
             1 12 20 15 19\n\
            \n\
             3 15  0  2 22\n\
             9 18 13 17  5\n\
            19  8  7 25 23\n\
            20 11 10 24  4\n\
            14 21 16 12  6\n\
            \n\
            14 21 17 24  4\n\
            10 16 15  9 19\n\
            18  8 23 26 20\n\
            22 11 13  6  5\n\
             2  0 12  3  7\n\
        ",
        );

        let result = part1(&input);
        let expected_result = Some(4512);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_part2() {
        let input: String = String::from(
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\
            \n\
            22 13 17 11  0\n\
             8  2 23  4 24\n\
            21  9 14 16  7\n\
             6 10  3 18  5\n\
             1 12 20 15 19\n\
            \n\
             3 15  0  2 22\n\
             9 18 13 17  5\n\
            19  8  7 25 23\n\
            20 11 10 24  4\n\
            14 21 16 12  6\n\
            \n\
            14 21 17 24  4\n\
            10 16 15  9 19\n\
            18  8 23 26 20\n\
            22 11 13  6  5\n\
             2  0 12  3  7\n\
        ",
        );

        let result = part2(&input);
        let expected_result = Some(1924);
        assert_eq!(result, expected_result);
    }
}

// You're already almost 1.5km (almost a mile) below the surface of the ocean, already so deep that you can't see any sunlight. What you can see, however, is a giant squid that has attached itself to the outside of your submarine.

// Maybe it wants to play bingo?

// Bingo is played on a set of boards each consisting of a 5x5 grid of numbers. Numbers are chosen at random, and the chosen number is marked on all boards on which it appears. (Numbers may not appear on all boards.) If all numbers in any row or any column of a board are marked, that board wins. (Diagonals don't count.)

// The submarine has a bingo subsystem to help passengers (currently, you and the giant squid) pass the time. It automatically generates a random order in which to draw numbers and a random set of boards (your puzzle input). For example:

// 7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

// 22 13 17 11  0
//  8  2 23  4 24
// 21  9 14 16  7
//  6 10  3 18  5
//  1 12 20 15 19

//  3 15  0  2 22
//  9 18 13 17  5
// 19  8  7 25 23
// 20 11 10 24  4
// 14 21 16 12  6

// 14 21 17 24  4
// 10 16 15  9 19
// 18  8 23 26 20
// 22 11 13  6  5
//  2  0 12  3  7
// After the first five numbers are drawn (7, 4, 9, 5, and 11), there are no winners, but the boards are marked as follows (shown here adjacent to each other to save space):

// 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
//  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
// 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
//  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
//  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
// After the next six numbers are drawn (17, 23, 2, 0, 14, and 21), there are still no winners:

// 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
//  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
// 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
//  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
//  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
// Finally, 24 is drawn:

// 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
//  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
// 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
//  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
//  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
// At this point, the third board wins because it has at least one complete row or column of marked numbers (in this case, the entire top row is marked: 14 21 17 24 4).

// The score of the winning board can now be calculated. Start by finding the sum of all unmarked numbers on that board; in this case, the sum is 188. Then, multiply that sum by the number that was just called when the board won, 24, to get the final score, 188 * 24 = 4512.

// To guarantee victory against the giant squid, figure out which board will win first. What will your final score be if you choose that board?
