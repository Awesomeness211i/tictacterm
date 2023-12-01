#![allow(non_snake_case)]
#[derive(Copy, Clone, PartialEq)]
pub enum BoardCell {
	X,
	O,
	None,
}
impl std::fmt::Display for BoardCell {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::X => write!(f, "X"),
			Self::O => write!(f, "O"),
			Self::None => write!(f, " "),
		}
	}
}

#[derive(Debug)]
pub enum BoardError {
	OutOfBounds,
	AlreadyChanged,
}
impl std::fmt::Display for BoardError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::OutOfBounds => write!(f, "Invalid index, max range of x and y is 2"),
			Self::AlreadyChanged => write!(f, "You can't change your or a previous player's answer"),
		}
	}
}

pub struct TicTacToeBoard {
	cells: [BoardCell; 9],
}
impl std::fmt::Display for TicTacToeBoard {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}|{}|{}\n-+-+-\n{}|{}|{}\n-+-+-\n{}|{}|{}\n", self.cells[0], self.cells[1], self.cells[2], self.cells[3], self.cells[4], self.cells[5], self.cells[6], self.cells[7], self.cells[8])
	}
}
impl std::default::Default for TicTacToeBoard {
	fn default() -> Self {
		Self {
			cells: [BoardCell::None; 9],
		}
	}
}
impl TicTacToeBoard {
	fn isFull(&self) -> bool {
		let mut boardFull = true;
		for cell in self.cells {
			if cell == BoardCell::None {
				boardFull = false;
				break;
			}
		}
		boardFull
	}

	fn checkVertical(&self, cellState: BoardCell, x: usize) -> bool {
		self.cells[x] == cellState && self.cells[x + 3] == cellState && self.cells[x + 6] == cellState
	}

	fn checkHorizontal(&self, cellState: BoardCell, y: usize) -> bool {
		let x = 3 * y;
		self.cells[x] == cellState && self.cells[x + 1] == cellState && self.cells[x + 2] == cellState
	}

	pub fn changeState(&mut self, cellState: BoardCell, (x, y): (usize, usize)) -> Result<GameState, BoardError> {
		if x >= 3 || y >= 3 {
			return Err(BoardError::OutOfBounds);
		}
		let index = x + 3 * y;
		match self.cells[index] {
			BoardCell::None => {
				self.cells[index] = cellState;
				if self.checkVertical(cellState, x) {
					return Ok(GameState::End(cellState));
				}
				if self.checkHorizontal(cellState, y) {
					return Ok(GameState::End(cellState));
				}
				// Check Diagonal 0-8
				if self.cells[0] == cellState && self.cells[4] == cellState && self.cells[8] == cellState {
					return Ok(GameState::End(cellState));
				}
				// Check Diagonal 2-6
				if self.cells[2] == cellState && self.cells[4] == cellState && self.cells[6] == cellState {
					return Ok(GameState::End(cellState));
				}
				// Check Draw
				if self.isFull() {
					return Ok(GameState::End(BoardCell::None));
				}
				Ok(GameState::InProgress)
			},
			_ => Err(BoardError::AlreadyChanged),
		}
	}
}

pub enum GameState {
	InProgress,
	// Reuse BoardCell since it contains only needed player win options
	End(BoardCell),
}
impl std::fmt::Display for GameState {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::InProgress => write!(f, "In progress game state"),
			Self::End(player) => {
				match player {
					BoardCell::X => write!(f, "Player 1 wins!"),
					BoardCell::O => write!(f, "Player 2 wins!"),
					BoardCell::None => write!(f, "The game was a draw"),
				}
			},
		}
	}
}

fn main() {
	let mut board = TicTacToeBoard::default();
	// Reuse BoardCell since it contains only needed player state options
	let mut playerState = BoardCell::X;
	let mut gameState = GameState::InProgress;
	let stdin = std::io::stdin();
	let mut userInput = String::with_capacity(3);
	while let GameState::InProgress = gameState {
		println!("{board}");
		userInput.clear();
		match playerState {
			BoardCell::X => println!("Player 1: "),
			BoardCell::O => println!("Player 2: "),
			_ => unreachable!(),
		}
		if stdin.read_line(&mut userInput).is_ok() {
			let args = userInput.split(' ').collect::<Vec<_>>();
			if args.len() != 2 {
				println!("Incorrect number of arguments");
				continue
			}
			let (x, y) = match (args[0].trim().parse::<usize>(), args[1].trim().parse::<usize>()) {
				(Ok(x), Ok(y)) => (x, y),
				(Err(e), Ok(_)) => {
					println!("{e}");
					continue
				},
				(Ok(_), Err(e)) => {
					println!("{e}");
					continue
				},
				(Err(e1), Err(e2)) => {
					println!("{e1}, {e2}");
					continue
				},
			};
			match board.changeState(playerState, (x, y)) {
				Ok(state) => {
					gameState = state;
				},
				Err(e) => {
					println!("{e}");
					continue
				},
			}
			playerState = match playerState {
				BoardCell::X => BoardCell::O,
				BoardCell::O => BoardCell::X,
				_ => unreachable!(),
			};
		}
	}
	println!("{gameState}");
}
