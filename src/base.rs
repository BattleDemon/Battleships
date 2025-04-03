use macroquad::prelude::*;
use ::rand::prelude::*;
extern crate macroquad_grid_dex;
use macroquad_grid_dex::Grid;

pub const GRID_SIZE: usize = 10;

#[derive(Copy, Clone, PartialEq)]
pub enum Cells {
    Empty,
    Occupied,
    Hit,
    Miss,
    Reinforced,
}

pub struct Board {
    pub cells: [[Cells; GRID_SIZE]; GRID_SIZE]
}

#[derive(Clone)]
pub enum Orientation {
    Horizontal, // The ship is horizontal 
    Verticle,   // The ship is vertical 
}

#[derive(Clone)]
pub enum ShipType {
    Battleship,  // Ship size 4
    Cruiser,     // Ship size 3
    Submarine,   // Ship size 3
    Destroyer,   // Ship size 2
    Dreadnaught, // Ship size 5
}

#[derive(Clone)]
pub struct Ship {
    pub ship_type: ShipType,       // Tracks the type of ship
    pub positions: Vec<(usize, usize)>, // Coordinates ship occupies
    pub orientation: Orientation,  // Orientation of the ship used for ship generation
}

#[derive(PartialEq)]
pub enum GameState {
    Player1,
    Player2,
    Else,
}

pub struct BasePlayer {
    pub board: Board,
    pub boardgrid: Grid,

    pub guess_board: Board,
    pub guessgrid: Grid,

    pub ships: Vec<Ship>,
    pub ship_count: usize,
}

impl Board {
    pub fn new() -> Self {
        Board {  
            cells: [[Cells::Empty; GRID_SIZE]; GRID_SIZE],
        }
    }

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

impl BasePlayer {
    pub fn new() -> Self {
        let mut p = BasePlayer {
            board: Board::new(),
            boardgrid: Grid::new(400.0, 400.0, 10, 10, 1.0),

            guess_board: Board::new(),
            guessgrid: Grid::new(400.0, 400.0, 10, 10, 1.0),

            ships: Vec::new(),
            ship_count: 5,
        };

        p.place_ship(ShipType::Battleship, Orientation::Verticle);
        p.place_ship(ShipType::Submarine, Orientation::Verticle);
        p.place_ship(ShipType::Cruiser, Orientation::Horizontal);
        p.place_ship(ShipType::Dreadnaught, Orientation::Verticle);
        p.place_ship(ShipType::Destroyer, Orientation::Horizontal);

        p.boardgrid.set_x_offset(macroquad_grid_dex::Position::Pixels(150.));
        p.boardgrid.set_y_offset(macroquad_grid_dex::Position::Pixels(50.));
        p.boardgrid.set_cell_bg_color(BLACK);
        p.boardgrid.set_gap_color(GREEN);

        p.guessgrid.set_x_offset(macroquad_grid_dex::Position::Pixels(screen_width()-100.));
        p.guessgrid.set_y_offset(macroquad_grid_dex::Position::Pixels(50.));
        p.guessgrid.set_cell_bg_color(BLACK);
        p.guessgrid.set_gap_color(GREEN);

        return p;
    }

    pub fn fire_missile(&mut self, opponent: &mut BasePlayer, target_x: usize, target_y: usize) -> bool {
        let ocell = &mut opponent.board.cells[target_x][target_y];
        
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

    // Returns the index of the ship at the given coordinates, if any
    pub fn find_ship_at(&self, x: usize, y: usize) -> Option<usize> {
        self.ships.iter()
            .position(|ship| ship.positions.contains(&(x, y)))
    }

    // Checks if a specific ship is completely destroyed
    pub fn is_ship_destroyed(&self, ship_idx: usize) -> bool {
        self.ships[ship_idx].positions.iter()
            .all(|&(x, y)| self.board.cells[x][y] == Cells::Hit)
    }

    // Updates the ship count based on which ships are still alive
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