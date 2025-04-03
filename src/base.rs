//! Core game logic for Battleship Classic mode.
//! Handles board states, ship placement, and basic attack mechanics.

/* ------ Import Used Libraries ------ */
 // Graphics library
use macroquad::prelude::*;
// Random library
use ::rand::prelude::*;
// A module I recompiled and made small fixes to, but did not write. Used for grid graphics and logic.
extern crate macroquad_grid_dex; 
use macroquad_grid_dex::Grid;

/*------ Constants ------ */
pub const GRID_SIZE: usize = 10; // Defines how many cells make up a grid

/*------ Enums and Structs ------ */
/// Represents possible states of a grid cell.
#[derive(Copy, Clone, PartialEq)] // Copy - Enables bitwise copying of the type, doesn't move ownership. Clone - Creates a deep copy of the value, can proform complex copying. PartialEq - Allows comparison of this type.
pub enum Cells {
    Empty, // Unknow or empty cell
    Occupied, // Cell containing part of a ship
    Hit, // Cell of a successful hit
    Miss, // Cell of a failed hit
    Reinforced, // Cell has extra protection (used in Twist mode, but you can't add to enums after making them so i had to add it here.)
}

/// Game board containing cell's and their states
pub struct Board {
    pub cells: [[Cells; GRID_SIZE]; GRID_SIZE] 
}

/// Orientation for ship placement.
#[derive(Clone)] // Clone - Creates a deep copy of the value, can proform complex copying.
pub enum Orientation {
    Horizontal, // The ship is horizontal 
    Verticle,   // The ship is verticle 
}

/// Types of ships and their respective lengths.
#[derive(Clone)] // Clone - Creates a deep copy of the value, can proform complex copying.
pub enum ShipType {
    Battleship,  // Ship size 4
    Cruiser,     // Ship size 3
    Submarine,   // Ship size 3
    Destroyer,   // Ship size 2
    Dreadnaught, // Ship size 5
}

/// Represents a ship on the board.
#[derive(Clone)] // Clone - Creates a deep copy of the value, can proform complex copying.
pub struct Ship {
    pub ship_type: ShipType,       // Tracks the type of ship and determines length
    pub positions: Vec<(usize, usize)>, // Grid coordinates the ship occupies
    pub orientation: Orientation,  // Orientation of the ship used for ship generation
}

/// Tracks which player's turn it is.
#[derive(PartialEq)] // PartialEq - Allows comparison of this type.
pub enum GameState {
    Player1,
    Player2,
    Else, // Transition state between turns, intended to allow for passing of the device to the next player.
}

/// Core player structure 
pub struct BasePlayer {
    pub board: Board, // Players own board to see where the opponent guesses and where your ships are.
    pub boardgrid: Grid, // Visual for the board.

    pub guess_board: Board, // Track of opponent's board and your guesses
    pub guessgrid: Grid, // Visual of guesses

    pub ships: Vec<Ship>, // Collection of placed ships
    pub ship_count: usize, // Remaining undestroyed ships
}

/* ------ Struct Implementations ------ */
/// Board Implementations
impl Board {
    /// Constructor function
    pub fn new() -> Self {
        Board {  
            cells: [[Cells::Empty; GRID_SIZE]; GRID_SIZE],
        }
    }
    /// Function to change cell based off of the provided celltype and grid position
    pub fn change_cell(&mut self, x: usize, y: usize, ctype: Cells, grid: &mut Grid) { 
        if self.cells[x][y] != Cells::Hit {
            match ctype {
                Cells::Empty => grid.color_cell(x, y, DARKGRAY),
                Cells::Occupied => grid.color_cell(x, y, GREEN),
                Cells::Hit => grid.color_cell(x, y, RED),
                Cells::Miss => grid.color_cell(x, y, GRAY),
                Cells::Reinforced => grid.color_cell(x,y,DARKGREEN),
            }
            self.cells[x][y] = ctype;
        }  
    }
}

/// Implementation for the base player handles all shared player functions
impl BasePlayer {
    /// Player constructor
    pub fn new() -> Self {
        let mut p = BasePlayer {
            board: Board::new(),
            boardgrid: Grid::new(400.0, 400.0, 10, 10, 1.0),

            guess_board: Board::new(),
            guessgrid: Grid::new(400.0, 400.0, 10, 10, 1.0),

            ships: Vec::new(),
            ship_count: 5,
        };

        //Place ships 
        p.place_ship(ShipType::Battleship, Orientation::Verticle); 
        p.place_ship(ShipType::Submarine, Orientation::Verticle);
        p.place_ship(ShipType::Cruiser, Orientation::Horizontal);
        p.place_ship(ShipType::Dreadnaught, Orientation::Verticle);
        p.place_ship(ShipType::Destroyer, Orientation::Horizontal);
        // Change grid offset and cell colour for your board
        p.boardgrid.set_x_offset(macroquad_grid_dex::Position::Pixels(150.));
        p.boardgrid.set_y_offset(macroquad_grid_dex::Position::Pixels(50.));
        p.boardgrid.set_cell_bg_color(BLACK);
        p.boardgrid.set_gap_color(GREEN);
        // Change grid offset and cell colour for the guess board
        p.guessgrid.set_x_offset(macroquad_grid_dex::Position::Pixels(screen_width()-100.));
        p.guessgrid.set_y_offset(macroquad_grid_dex::Position::Pixels(50.));
        p.guessgrid.set_cell_bg_color(BLACK);
        p.guessgrid.set_gap_color(GREEN);

        return p;
    }

    /// Base attack / guess from the IRL game
    /// Returns true if attack hits a ship.
    pub fn fire_missile(&mut self, opponent: &mut BasePlayer, target_x: usize, target_y: usize) -> bool {
        let ocell = &mut opponent.board.cells[target_x][target_y]; // Create a local refrence to the cell
        
        match *ocell {
            Cells::Occupied => {
                opponent.board.change_cell(target_x, target_y, Cells::Hit, &mut opponent.boardgrid);
                self.guess_board.change_cell(target_x, target_y, Cells::Hit, &mut self.guessgrid);
                println!("Hit!");
                
                // Check if this hit destroyed a ship
                if let Some(ship_idx) = opponent.find_ship_at(target_x, target_y) {
                    if opponent.is_ship_destroyed(ship_idx) {
                        println!("Ship completely destroyed!");
                        opponent.update_ship_count();
                    }
                }
                true
            }
            _ => {
                opponent.board.change_cell(target_x, target_y, Cells::Miss, &mut opponent.boardgrid);
                self.guess_board.change_cell(target_x, target_y, Cells::Miss, &mut self.guessgrid);
                println!("Miss!");
                false
            }
        }
    }

    /// Converts mouse position to grid coordinates on guess board.
    /// Returns Some((x,y)) if the mouse is within the grid.
    pub fn get_clicked_cell(&self) -> Option<(usize, usize)> {
        let (mouse_x, mouse_y) = mouse_position();
        let grid_x_offset = 700.0;
        let grid_y_offset = 50.0;
        let cell_size = 40.0;
        let grid_size_px = cell_size * GRID_SIZE as f32;
    
        if mouse_x >= grid_x_offset && mouse_x < grid_x_offset + grid_size_px &&
           mouse_y >= grid_y_offset && mouse_y < grid_y_offset + grid_size_px {
            let x = ((mouse_y - grid_y_offset) / cell_size) as usize;
            let y = ((mouse_x - grid_x_offset) / cell_size) as usize;
            return Some((x, y));
        }
    
        None
    }

    /// Attemps to place a ship randomly on the board.
    /// Trues up to 100 times to find a valid placement.
    pub fn place_ship(&mut self, ship_type: ShipType, orientation: Orientation) -> Option<Ship> {
        let mut rng = ::rand::rng();
        let ship_length = match ship_type {
            ShipType::Battleship => 4,
            ShipType::Cruiser => 3,
            ShipType::Submarine => 3,
            ShipType::Destroyer => 2,
            ShipType::Dreadnaught => 5,
        };
        
        let possible_pos: Vec<usize> = (0..GRID_SIZE).collect();
        
        for _ in 0..100 {
            let tempx = possible_pos.choose(&mut rng);
            let tempy = possible_pos.choose(&mut rng);
            let x: usize = *tempx.unwrap();
            let y: usize = *tempy.unwrap();
    
            let mut directions = match orientation {
                Orientation::Horizontal => vec![(0, 1), (0, -1)],
                Orientation::Verticle => vec![(1, 0), (-1, 0)],
            };
            
            directions.shuffle(&mut rng);
            let (dx, dy) = directions[0];
            let mut positions = vec![];
            let mut fits = true;
            let mut temp_x = x;
            let mut temp_y = y;
    
            for _ in 0..ship_length {
                if temp_x >= GRID_SIZE || temp_y >= GRID_SIZE {
                    fits = false;
                    break;
                }
                positions.push((temp_x, temp_y));
                temp_x = (temp_x as isize + dx) as usize;
                temp_y = (temp_y as isize + dy) as usize;
            }
    
            if !fits {
                continue;
            }
    
            if positions.iter().any(|&(px, py)| self.board.cells[px][py] != Cells::Empty) {
                continue;
            }
    
            for &(sx, sy) in &positions {
                self.board.change_cell(sx, sy, Cells::Occupied, &mut self.boardgrid);
            }
    
            let ship = Ship {
                ship_type,
                positions,
                orientation,
            };
            
            self.ships.push(ship.clone());
            return Some(ship);
        }
    
        None
    }

    /// Returns the index of the ship at the given coordinates, if any
    pub fn find_ship_at(&self, x: usize, y: usize) -> Option<usize> {
        self.ships.iter()
            .position(|ship| ship.positions.contains(&(x, y)))
    }

    /// Checks if a specific ship is completely destroyed
    pub fn is_ship_destroyed(&self, ship_idx: usize) -> bool {
        self.ships[ship_idx].positions.iter()
            .all(|&(x, y)| self.board.cells[x][y] == Cells::Hit)
    }

    /// Updates the ship count based on which ships are still alive
    pub fn update_ship_count(&mut self) {
        self.ship_count = self.ships.iter()
            .filter(|ship| {
                // A ship is still alive if at least one of its cells isn't hit
                ship.positions.iter()
                    .any(|&(x, y)| self.board.cells[x][y] != Cells::Hit)
            })
            .count();
    }
}