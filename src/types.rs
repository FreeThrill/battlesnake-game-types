//! various types that are useful for working with battlesnake
use crate::wire_representation::Game;
use std::collections::HashMap;
use std::fmt;
use std::time::Duration;

/// Represents the snake IDs for a given game. This should be established once on the `/start` request and then
/// stored, so that `SnakeIds` are stable throughout the game.
pub type SnakeIDMap = HashMap<String, SnakeId>;

/// A vector with which to do positional math
#[derive(Debug, Clone, Copy)]
pub struct Vector {
    /// x position
    pub x: i64,
    /// y position
    pub y: i64,
}

/// Represents a move
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Move {
    #[allow(missing_docs)]
    Left,
    #[allow(missing_docs)]
    Down,
    #[allow(missing_docs)]
    Up,
    #[allow(missing_docs)]
    Right,
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Move::Left => write!(f, "left"),
            Move::Right => write!(f, "right"),
            Move::Up => write!(f, "up"),
            Move::Down => write!(f, "down"),
        }
    }
}

impl Move {
    /// convert this move to a vector
    pub fn to_vector(self) -> Vector {
        match self {
            Move::Left => Vector { x: -1, y: 0 },
            Move::Right => Vector { x: 1, y: 0 },
            Move::Up => Vector { x: 0, y: 1 },
            Move::Down => Vector { x: 0, y: -1 },
        }
    }

    /// returns a vec of all possible moves
    pub fn all() -> Vec<Move> {
        vec![Move::Up, Move::Down, Move::Left, Move::Right]
    }

    /// converts this move to a usize index. indices are the same order as the `Move::all()` method
    pub fn as_index(&self) -> usize {
        match self {
            Move::Up => 0,
            Move::Down => 1,
            Move::Left => 2,
            Move::Right => 3,
        }
    }

    #[allow(dead_code)]
    /// checks if a given move is not opposibe this move. e.g. Up is not opposite to Left, but is opposite to Down
    pub fn is_not_opposite(&self, other: &Move) -> bool {
        !matches!(
            (self, other),
            (Move::Up, Move::Down)
                | (Move::Down, Move::Up)
                | (Move::Left, Move::Right)
                | (Move::Right, Move::Left)
        )
    }
}

/// token to represent a snake id
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SnakeId(pub u8);

/// builds a snake ID map for a given game, mapping snakes to
/// integers. The snake in "you" is always ID 0. Instead of
/// calling this on every game you are given, you should call
/// this function once per game at the start, and store the result
/// that way you can stabally have integer IDs for a given snake
/// throughout a game
pub fn build_snake_id_map(g: &Game) -> SnakeIDMap {
    let mut hm = HashMap::new();
    hm.insert(g.you.id.clone(), SnakeId(0));
    let mut i = 1;
    for snake in g.board.snakes.iter() {
        if snake.id != g.you.id {
            hm.insert(snake.id.clone(), SnakeId(i));
            i += 1;
        }
    }

    hm
}

/// A game for which one can get the snake ids
pub trait SnakeIDGettableGame {
    #[allow(missing_docs)]
    type SnakeIDType;
    #[allow(missing_docs)]
    fn get_snake_ids(&self) -> Vec<Self::SnakeIDType>;
}

/// Instruments to be used with simulation
pub trait SimulatorInstruments: std::fmt::Debug {
    #[allow(missing_docs)]
    fn observe_simulation(&self, duration: Duration);
}

/// A game for which "you" is determinable
pub trait YouDeterminableGame: std::fmt::Debug {
    #[allow(missing_docs)]
    type SnakeIDType;

    /// determines for a given game if a given snake id is you.
    fn is_you(&self, snake_id: &Self::SnakeIDType) -> bool;

    /// get the id for you for a given game
    fn you_id(&self) -> &Self::SnakeIDType;
}

/// A game which can have it's winner determined
pub trait VictorDeterminableGame: std::fmt::Debug {
    #[allow(missing_docs)]
    type SnakeIDType;
    #[allow(missing_docs)]
    fn is_over(&self) -> bool;

    /// get the winner for a given game, will return None in the case of a draw, or if the game is not over
    fn get_winner(&self) -> Option<Self::SnakeIDType>;
}

/// a game for which future states can be simulated
pub trait SimulableGame<T: SimulatorInstruments>: std::fmt::Debug + Sized {
    #[allow(missing_docs)]
    type SnakeIDType;
    /// simulates all possible future games for a given game returning the snake ids, moves that
    /// got to a given state, plus that state
    fn simulate(
        &self,
        instruments: &T,
        snake_ids: Vec<Self::SnakeIDType>,
    ) -> Vec<(Vec<(Self::SnakeIDType, Move)>, Self)> {
        let moves_to_simulate = Move::all();
        let build = snake_ids
            .into_iter()
            .map(|s| (s, moves_to_simulate.clone()))
            .collect::<Vec<_>>();
        self.simulate_with_moves(instruments, build)
    }
    /// simulates the next possible states for a a game with a given set of snakes and moves, producing a list of the new games,
    /// along with the moves that got to that position
    fn simulate_with_moves(
        &self,
        instruments: &T,
        snake_ids_and_moves: Vec<(Self::SnakeIDType, Vec<Move>)>,
    ) -> Vec<(Vec<(Self::SnakeIDType, Move)>, Self)>;
}

/// a game for which random reasonable moves for a given snake can be determined. e.g. do not collide with yourself
pub trait RandomReasonableMovesGame {
    #[allow(missing_docs)]
    type SnakeIDType;
    #[allow(missing_docs)]
    fn random_reasonable_move_for_each_snake(&self) -> Vec<(Self::SnakeIDType, Move)>;
}