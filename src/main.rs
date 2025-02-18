use std::{collections::HashSet, str::FromStr};

fn main() {
    let input_file_name = "input.txt";
    let contents = std::fs::read_to_string(input_file_name).expect("Failed to read the input file");
    println!("{}", contents.len());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x , y }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct IllegalChar(char);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl TryFrom<char> for Direction {
    type Error = IllegalChar;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            '^' => Self::North,
            'v' => Self::South,
            '>' => Self::East,
            '<' => Self::West,
            _ => return Err(IllegalChar(c)), //if you return inside a match you are just jumping out of the surroundings and none of the other stuff/ ok will apply
        })
    }
}

struct Moves{
    moves: Vec<Direction>
}



impl FromStr for Moves {
    type Err = IllegalChar; //if you define a trait, you can define assoicted types with the trait, a bit like generic not something specifed from outside but something more intenal to the type, there are a number of traits that have assoated errors you must say what error it generated

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let moves = s.chars()
        .map(Direction::try_from)
        .collect::<Result<Vec<_>, IllegalChar>>()?; //if these are all ok, you get just the vector with the ok arount it- if any fail then instead of an ok it just returns the error

        Ok(Self {moves})
    } //this is how we parse from a string to the type we have requires a method from_str that takes a str and return a result tyo
    
}

pub struct VisitedHouses {
    visited_houses: HashSet<Pos>,
    current_position: Pos,
}

impl VisitedHouses {
    pub fn new() -> Self {
        let initial_position = Pos {x: 0, y: 0};
        let mut visited_houses = HashSet::new();
        visited_houses.insert(initial_position);

        Self {
            visited_houses,
            current_position: Pos {x: 0, y:  0},
        }
    }


pub fn num_visited_houses(&self) -> usize {
    self.visited_houses.len()
}

pub const fn current_pos(&self) -> Pos {
    self.current_position
}

pub fn perform_move (&mut self, direction: Direction) {
   let new_position =  match direction {
        Direction::North => Pos::new(self.current_pos().x, self.current_pos().y + 1),
        Direction::South => Pos::new(self.current_pos().x, self.current_pos().y - 1 ),
        Direction::East => Pos::new(self.current_pos().x + 1, self.current_pos().y ),
        Direction::West => Pos::new(self.current_pos().x - 1, self.current_pos().y ),
    };

    self.current_position = new_position;
    self.visited_houses.insert(new_position);
}

pub fn perform_moves(&mut self, moves: Moves) {
    for m in moves.moves {
        self.perform_move(m);
    }
}

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visited_houses_new() {
        let mut visited_houses = VisitedHouses::new();
        assert_eq!(visited_houses.num_visited_houses(), 1);
        assert_eq!(visited_houses.current_pos(), Pos::new(0, 0));
    }

    #[test]
    fn test_direction_try_from() {
        assert_eq!('^'.try_into(), Ok(Direction::North));
        assert_eq!('v'.try_into(), Ok(Direction::South));
        assert_eq!('<'.try_into(), Ok(Direction::West));
        assert_eq!('>'.try_into(), Ok(Direction::East));
        assert_eq!(Direction::try_from('x'), Err(IllegalChar('x')));
    }

    #[test]
    fn test_move_east() {
        let mut visited_houses = VisitedHouses::new();
        visited_houses.perform_move(Direction::East);
        assert_eq!(visited_houses.num_visited_houses(), 2);
        assert_eq!(visited_houses.current_pos(), Pos::new(1, 0));
    }

    #[test]
    fn test_square_moves() {
        let mut visited_houses = VisitedHouses::new();
        let moves = Moves::from_str("^>v<").unwrap();
        visited_houses.perform_moves(moves);
        assert_eq!(visited_houses.num_visited_houses(), 4);
        assert_eq!(visited_houses.current_pos(), Pos::new(0, 0));
    }

    #[test]
    fn test_up_down_moves() {
        let mut visited_houses = VisitedHouses::new();
        let moves = Moves::from_str("^v^v^v^v^v").unwrap();
        visited_houses.perform_moves(moves);
        assert_eq!(visited_houses.num_visited_houses(), 2);
        assert_eq!(visited_houses.current_pos(), Pos::new(0, 0));
    }

    #[test]
    fn test_aoc_input() {
        let mut visited_houses = VisitedHouses::new();
        let moves = Moves::from_str(include_str!("../input.txt").trim()).unwrap();
        visited_houses.perform_moves(moves);
        assert_eq!(visited_houses.num_visited_houses(), 2565);
    }
}
