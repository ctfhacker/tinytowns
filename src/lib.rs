/// Board containing pieces holding the state of the current game
///
/// Below are the coordinates for each of the locations on the game board
///
/// + --- + --- + --- + --- +
/// | 0,0 | 0,1 | 0,2 | 0,3 |
/// | 0   | 1   | 2   | 3   |
/// + --- + --- + --- + --- +
/// | 1,0 | 1,1 | 1,2 | 1,3 |
/// | 4   | 5   | 6   | 7   |
/// + --- + --- + --- + --- +
/// | 2,0 | 2,1 | 2,2 | 3,3 |
/// | 8   | 9   | 10  | 1l  |
/// + --- + --- + --- + --- +
/// | 3,0 | 3,1 | 3,2 | 3,3 |
/// | 12  | 13  | 14  | 15  |
/// + --- + --- + --- + --- +
///
/// The formula to convert x,y into the index of the board itself is below:
///
/// row * 4 + column
///
///
///
use std::collections::HashSet;

const BOARD_WIDTH: u8 = 4;
const BOARD_HEIGHT: u8 = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Board {
    board: [Option<Piece>; BOARD_WIDTH as usize * BOARD_HEIGHT as usize],
}

impl Board {
    fn new() -> Board {
        Board { board: [None; 16] }
    }

    fn get_board_index(&self, x: u8, y: u8) -> u8 {
        x * BOARD_WIDTH + y
    }

    fn get_piece(&self, x: u8, y: u8) -> Option<Piece> {
        let index = self.get_board_index(x, y) as usize;
        self.board[index]
    }

    fn place_piece(&mut self, x: u8, y: u8, resource: Resource) -> Result<(), TinyTownsError> {
        if x > BOARD_WIDTH || y > BOARD_HEIGHT {
            return Err(TinyTownsError::OutOfBounds);
        }

        let index = self.get_board_index(x, y) as usize;

        if self.board[index].is_some() {
            return Err(TinyTownsError::Occupied);
        }

        self.board[index] = Some(Piece::Cube(resource));

        Ok(())
    }

    fn place_brick(&mut self, x: u8, y: u8) -> Result<(), TinyTownsError> {
        self.place_piece(x, y, Resource::Brick)
    }

    fn place_glass(&mut self, x: u8, y: u8) -> Result<(), TinyTownsError> {
        self.place_piece(x, y, Resource::Glass)
    }

    fn place_stone(&mut self, x: u8, y: u8) -> Result<(), TinyTownsError> {
        self.place_piece(x, y, Resource::Stone)
    }

    fn place_wheat(&mut self, x: u8, y: u8) -> Result<(), TinyTownsError> {
        self.place_piece(x, y, Resource::Wheat)
    }

    fn place_wood(&mut self, x: u8, y: u8) -> Result<(), TinyTownsError> {
        self.place_piece(x, y, Resource::Wood)
    }

    fn place_building(&mut self, x: u8, y: u8, building: Building) -> Result<(), TinyTownsError> {
        if x > BOARD_WIDTH || y > BOARD_HEIGHT {
            return Err(TinyTownsError::OutOfBounds);
        }

        let index = self.get_board_index(x, y) as usize;

        if self.board[index].is_some() {
            return Err(TinyTownsError::Occupied);
        }

        self.board[index] = Some(Piece::Structure(building));

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Piece {
    Cube(Resource),
    Structure(Building),
}

/// All resources available for a space to have
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Resource {
    Brick,
    Glass,
    Stone,
    Wheat,
    Wood,
}

/// All bulldings in the game
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Building {
    Well,
    Theater,
    TradingPost,
}

impl Building {
    fn pattern(&self) -> Pattern {
        match self {
            // W S
            Building::Well => Pattern::new(vec![vec![Some(Resource::Wood), Some(Resource::Stone)]]),
            //    S
            //  W G W
            Building::Theater => Pattern::new(vec![
                vec![None, Some(Resource::Stone), None],
                vec![
                    Some(Resource::Wood),
                    Some(Resource::Glass),
                    Some(Resource::Wood),
                ],
            ]),
            // S W
            // S W B
            Building::TradingPost => Pattern::new(vec![
                vec![Some(Resource::Stone), Some(Resource::Wood), None],
                vec![
                    Some(Resource::Stone),
                    Some(Resource::Wood),
                    Some(Resource::Brick),
                ],
            ]),
        }
    }

    /// Returns all possible rotations for a given pattern
    fn rotations(&self) -> HashSet<Pattern> {
        let pattern = self.pattern();

        let mut patterns = HashSet::new();

        patterns.insert(Pattern::new(
            pattern
                .coords
                .iter()
                .map(|x| x.to_owned())
                .collect::<Vec<_>>(),
        ));

        let horizontal_flip: Vec<Vec<Option<Resource>>> = pattern
            .coords
            .iter()
            .map(|x| x.to_owned())
            .rev()
            .collect::<Vec<_>>();
        patterns.insert(Pattern::new(horizontal_flip));

        let vertical_flip = pattern
            .coords
            .iter()
            .map(|line| line.iter().rev().map(|x| x.to_owned()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        println!("vertical_flip: {:?}", vertical_flip);
        patterns.insert(Pattern::new(vertical_flip));

        let horizontal_vertical_flip = pattern
            .coords
            .iter()
            .rev()
            .map(|line| line.iter().rev().map(|x| x.to_owned()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        println!("horizontal_vertical_flip: {:?}", horizontal_vertical_flip);
        patterns.insert(Pattern::new(horizontal_vertical_flip));

        let mut rotation = Vec::new();
        for width in 0..pattern.width {
            let mut new_row = Vec::new();
            for height in 0..pattern.height {
                new_row.push(pattern.coords[height as usize][width as usize]);
            }
            rotation.push(new_row);
        }

        // Now for the 90 degree rotation options

        patterns.insert(Pattern::new(
            rotation.iter().map(|x| x.to_owned()).collect::<Vec<_>>(),
        ));

        let rotation_horizontal_flip = rotation
            .iter()
            .map(|x| x.to_owned())
            .rev()
            .collect::<Vec<_>>();
        patterns.insert(Pattern::new(rotation_horizontal_flip));

        let rotation_vertical_flip = rotation
            .iter()
            .map(|x| x.to_owned())
            .map(|line| line.iter().rev().map(|x| x.to_owned()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        patterns.insert(Pattern::new(rotation_vertical_flip));

        let rotation_horizontal_vertical_flip = rotation
            .iter()
            .map(|x| x.to_owned())
            .rev()
            .map(|line| line.iter().rev().map(|x| x.to_owned()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        patterns.insert(Pattern::new(rotation_horizontal_vertical_flip));

        patterns
    }

    fn patterns(&self) -> Vec<Vec<((u8, u8), Option<Resource>)>> {
        let mut possibles = Vec::new();
        for pattern in self.rotations() {
            println!("self.rotation: {:?}", pattern);
            let mut flattened_resources = pattern.coords.iter().flatten().cycle();

            // Build the total possible array values where this pattern will fit
            for curr_x in 0..BOARD_WIDTH {
                // If our pattern width would exceed the bounds of the board, break
                if curr_x + (pattern.width - 1) >= BOARD_WIDTH {
                    break;
                }
                for curr_y in 0..BOARD_HEIGHT {
                    // If our pattern height would exceed the bounds of the board, break
                    if curr_y + (pattern.height - 1) >= BOARD_HEIGHT {
                        break;
                    }

                    let mut possible = Vec::new();
                    for tmp_y in curr_y..curr_y + pattern.height {
                        for tmp_x in curr_x..curr_x + pattern.width {
                            possible.push(((tmp_x, tmp_y), *flattened_resources.next().unwrap()));
                        }
                    }
                    possibles.push(possible);
                }
            }
        }

        possibles
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Pattern {
    coords: Vec<Vec<Option<Resource>>>,
    width: u8,
    height: u8,
}

impl Pattern {
    fn new(coords: Vec<Vec<Option<Resource>>>) -> Pattern {
        let width = coords.iter().map(|line| line.len()).max().unwrap() as u8;
        let height = coords.len() as u8;
        Pattern {
            coords,
            width,
            height,
        }
    }
}

impl std::fmt::Debug for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Resource::Wood => write!(f, "Wd"),
            Resource::Glass => write!(f, "Gs"),
            Resource::Wheat => write!(f, "Wt"),
            Resource::Brick => write!(f, "Bk"),
            Resource::Stone => write!(f, "St"),
        }
    }
}

impl std::fmt::Debug for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for line in &self.coords {
            for resource in line {
                write!(f, "{:?} ", resource);
            }
            writeln!(f, "");
        }
        Ok(())
    }
}

/// Errors associated with Tiny Towns
#[derive(Clone, Copy, PartialEq, Eq)]
enum TinyTownsError {
    /// Attempting to place a resource in an occupied location
    Occupied,

    /// Given coordinates are out of bounds of the board
    OutOfBounds,
}

fn print_legend() {
    println!("x: 0     1     2     3");
    println!("y + --- + --- + --- + --- +");
    println!("  | 0,0 | 1,0 | 2,0 | 3,0 |");
    println!("0 | 0   | 1   | 2   | 3   |");
    println!("  + --- + --- + --- + --- +");
    println!("  | 0,1 | 1,1 | 2,1 | 3,1 |");
    println!("1 | 4   | 5   | 6   | 7   |");
    println!("  + --- + --- + --- + --- +");
    println!("  | 0,2 | 1,2 | 2,2 | 3,2 |");
    println!("2 | 8   | 9   | 10  | 1l  |");
    println!("  + --- + --- + --- + --- +");
    println!("  | 0,3 | 1,3 | 2,3 | 3,3 |");
    println!("3 | 12  | 13  | 14  | 15  |");
    println!("  + --- + --- + --- + --- +");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn board_init_board() {
        let board = Board::new();
        assert!(board.board == [None; 16]);
    }

    #[test]
    fn add_brick() {
        let mut board = Board::new();
        board.place_brick(0, 0);
        assert!(board.get_piece(0, 0) == Some(Piece::Cube(Resource::Brick)));
    }

    #[test]
    fn add_glass() {
        let mut board = Board::new();
        board.place_glass(0, 0);
        assert!(board.get_piece(0, 0) == Some(Piece::Cube(Resource::Glass)));
    }

    #[test]
    fn add_stone() {
        let mut board = Board::new();
        board.place_stone(0, 0);
        assert!(board.get_piece(0, 0) == Some(Piece::Cube(Resource::Stone)));
    }

    #[test]
    fn add_wheat() {
        let mut board = Board::new();
        board.place_wheat(0, 0);
        assert!(board.get_piece(0, 0) == Some(Piece::Cube(Resource::Wheat)));
    }

    #[test]
    fn add_wood() {
        let mut board = Board::new();
        board.place_wood(0, 0);
        assert!(board.get_piece(0, 0) == Some(Piece::Cube(Resource::Wood)));
    }

    #[test]
    fn error_occupied() {
        let mut board = Board::new();
        board.place_wood(0, 0);
        assert!(board.place_brick(0, 0) == Err(TinyTownsError::Occupied));
    }

    #[test]
    fn error_out_of_bounds_x() {
        let mut board = Board::new();
        assert!(board.place_wood(0, 100) == Err(TinyTownsError::OutOfBounds));
    }

    #[test]
    fn error_out_of_bounds_y() {
        let mut board = Board::new();
        assert!(board.place_wood(100, 0) == Err(TinyTownsError::OutOfBounds));
    }

    #[test]
    fn add_well() {
        let mut board = Board::new();
        board.place_building(0, 0, Building::Well);
        assert!(board.get_piece(0, 0) == Some(Piece::Structure(Building::Well)));
    }

    #[test]
    fn test_rotations_easy() {
        assert_eq!(Building::Well.rotations().len(), 4);
        assert_eq!(Building::TradingPost.rotations().len(), 8);
    }
}
