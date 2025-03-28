// Imports
use ::rand::prelude::{SliceRandom, IndexedRandom};
use macroquad::{audio, prelude::*};
extern crate macroquad_grid_dex;
use macroquad_grid_dex::Grid;

// Constants
const GRID_SIZE: usize = 10; 
const HAND_SIZE: usize = 3;
const DECK_SIZE: usize = 48;

/* -------- Structs and Enum -------- */
// Cells used to keep track of the state of a cell/coordinate on the board
#[derive(Copy, Clone, PartialEq)]
enum Cells {
    Empty,      // Nothing is on this cell or you don't know if something is there
    Occupied,   // The cell has a ship
    Hit,        // Hit a Ship
    Miss,       // Fired and missed
    Reinforced, // After been hit it goes down to occupied then it can be hit
}

// Board used to keep track of the four game boards 2 per player 
struct Board {
    cells: [[Cells; GRID_SIZE]; GRID_SIZE], // Track every cell in the board
}

// The orientation of the ships
#[derive(Clone)]
enum Orientation {
    Horizontal, // The ship is horizontal 
    Verticle,   // The ship is vertical 
}

// Used to keep track of what type of ship 
#[derive(Clone)]
enum ShipType {
    Battleship,  // Ship size 4
    Cruiser,     // Ship size 3
    Submarine,   // Ship size 3
    Destroyer,   // Ship size 2
    Dreadnaught, // Ship size 5
}

// Track ships, its positions and orientation
#[derive(Clone)]
struct Ship {
    ship_type: ShipType,       // Tracks the type of ship
    positions: Vec<(usize, usize)>, // Coordinates ship occupies
    orientation: Orientation,  // Orientation of the ship used for ship generation
}

// Tracks all types of actions 
#[derive(Copy, Clone)]
enum ActionType {
    Missle,     // Missle is the base battle ships fire ability
    Torpedo,    // Torpedo fires from a point on the x axis then shots upwards along the y axis
    Patrol,     // Allows the player to move a ship
    RadarScan,  // Reveals what is on the selected position and adjacent cells
    Reinforce,  // Gives a cell an extra life
}

// Keeps a vector of actions and used to randomly select them
struct Deck {
    deck_list: [ActionType; DECK_SIZE],
}

// Track all player related variables 
struct Player {
    board: Board,           // Players board
    boardgrid: Grid,        // Players grid (displayable version of board)
    guess_board: Board,     // Board where player guesses the enemies ships
    guessgrid: Grid,        // Displayable version of above
    hand: Vec<ActionType>,  // Stores the action cards that can be selected
    deck: Deck,             // Deck where action cards are drawn from 
    ships: Vec<Ship>,       // List of ships attached to the player
    ship_count: usize,      // Number of ships with at least one cell
}

/*-------- Impl for Structs -------- */
impl Board {
    // Board Constructor
    fn new() -> Self {
        Board {  
            cells: [[Cells::Empty; GRID_SIZE]; GRID_SIZE],
        }
    }
    
    // Change the inputed cell to the provided cell type
    fn change_cell(&mut self, x: usize, y: usize, ctype: Cells, grid: &mut Grid) { 
        if self.cells[x][y] != Cells::Hit {
            match ctype {
                Cells::Empty => grid.color_cell(x, y, DARKGRAY),
                Cells::Occupied => grid.color_cell(x, y, GREEN),
                Cells::Hit => grid.color_cell(x, y, RED),
                Cells::Miss => {
                    grid.set_cell_text(x, y, Some("0"));
                    grid.color_cell(x, y, GRAY);
                },
                Cells::Reinforced => grid.color_cell(x, y, DARKGREEN),
            }
            self.cells[x][y] = ctype;
        }  
    }
}

// Ship functions
impl Ship {
    // Ship constructor
    fn new() {
    }
}

// Deck functions
impl Deck {
    // Deck Constructor
    fn new() -> Self {
        Deck {
            deck_list: [ActionType::Missle; DECK_SIZE],
        }
    }

    fn build(&mut self) {
        let mut deck_pos = 17;

        loop {
            if deck_pos <= 26 {
                self.deck_list[deck_pos] = ActionType::Torpedo;
            } else if deck_pos <= 34 {
                self.deck_list[deck_pos] = ActionType::Patrol;
            } else if deck_pos <= 41 {
                self.deck_list[deck_pos] = ActionType::Reinforce;
            } else if deck_pos <= 47 {
                self.deck_list[deck_pos] = ActionType::RadarScan;
            } else if deck_pos == 48 {
                break;
            }
            deck_pos += 1;
        }  
    }

    fn shuffle(&mut self) {
        let mut rng = ::rand::rng();
        self.deck_list.shuffle(&mut rng);
    }

    fn draw_card() {
    }
}

/*-------- Player Implementations -------- */
impl Player {
    // Player Constructor
    fn new() -> Self {
        Player {
            board: Board::new(),
            boardgrid: Grid::new(400.0, 400.0, 10, 10, 1.0),
            guess_board: Board::new(),
            guessgrid: Grid::new(400.0, 400.0, 10, 10, 1.0),
            hand: Vec::new(),
            deck: Deck::new(),
            ships: Vec::new(),
            ship_count: 5,
        }
    }

    fn fire_missile(&mut self, opponent: &mut Player, target_x: usize, target_y: usize) -> bool {
        let ocell = &mut opponent.board.cells[target_x][target_y];
    
        match *ocell {
            Cells::Reinforced => {
                println!("Reinforced hit! Cell downgraded to Occupied.");
                opponent.board.change_cell(target_x, target_y, Cells::Occupied, &mut opponent.boardgrid);
                self.guess_board.change_cell(target_x, target_y, Cells::Occupied, &mut self.guessgrid);
                true
            }
            Cells::Occupied => {
                opponent.board.change_cell(target_x, target_y, Cells::Hit, &mut opponent.boardgrid);
                self.guess_board.change_cell(target_x, target_y, Cells::Hit, &mut self.guessgrid);
                println!("Hit!");
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

    fn fire_torpedo(&mut self, opponent: &mut Player, target_y: usize) -> bool {
        let mut x = GRID_SIZE - 1;
        let mut hit_something = false;
    
        while x < GRID_SIZE {
            match opponent.board.cells[x][target_y] {
                Cells::Reinforced => {
                    opponent.board.change_cell(x, target_y, Cells::Occupied, &mut opponent.boardgrid);
                    println!("Torpedo hit a reinforced cell! Protection removed.");
                    hit_something = true;
                    break;
                }
                Cells::Occupied => {
                    self.guess_board.change_cell(x, target_y, Cells::Hit, &mut self.guessgrid);
                    opponent.board.change_cell(x, target_y, Cells::Hit, &mut opponent.boardgrid);
                    println!("Torpedo hit!");
                    hit_something = true;
                    break;
                }
                Cells::Hit => {
                    println!("Torpedo stopped! Already hit here.");
                    break;
                }
                _ => {
                    self.guess_board.change_cell(x, target_y, Cells::Miss, &mut self.guessgrid);
                    opponent.board.change_cell(x, target_y, Cells::Miss, &mut opponent.boardgrid);
                    println!("Torpedo missed!");
                }
            }
    
            if x == 0 { break; }
            x -= 1;
        }
    
        hit_something
    }
    
    fn get_torpedo_target_column(&self) -> Option<usize> {
        let (mouse_x, mouse_y) = mouse_position();
        let grid_x_offset = 700.0;
        let grid_y_offset = 50.0;
        let cell_size = 40.0;
        let grid_size_px = cell_size * GRID_SIZE as f32;

        if mouse_x >= grid_x_offset && mouse_x < grid_x_offset + grid_size_px &&
           mouse_y >= grid_y_offset && mouse_y < grid_y_offset + grid_size_px {
            let y = ((mouse_x - grid_x_offset) / cell_size) as usize;
            return Some(y);
        }

        None
    }

    fn radar_scan(&mut self, opponent: &mut Player, target_x: usize, target_y: usize) {
        let offsets = [(0, 0), (0, 1), (0, -1), (1, 0), (-1, 0)];
    
        for &(dx, dy) in &offsets {
            let nx = target_x as isize + dx;
            let ny = target_y as isize + dy;
    
            if nx >= 0 && nx < GRID_SIZE as isize && ny >= 0 && ny < GRID_SIZE as isize {
                let ux = nx as usize;
                let uy = ny as usize;
                let cell = opponent.board.cells[ux][uy];
    
                self.guess_board.change_cell(ux, uy, cell, &mut self.guessgrid);
            }
        }
        println!("Radar scan complete!");
    }

    fn check_hit(&self, target_x: usize, target_y: usize) -> bool {
        self.guess_board.cells[target_x][target_y] == Cells::Occupied
    }

    fn reinforce(&mut self, target_x: usize, target_y: usize) -> bool {
        let current_state = self.board.cells[target_x][target_y];
        
        match current_state {
            Cells::Occupied => {
                self.board.change_cell(target_x, target_y, Cells::Reinforced, &mut self.boardgrid);
                println!("Cell at ({}, {}) reinforced!", target_x, target_y);
                true
            }
            Cells::Reinforced => {
                println!("Cell at ({}, {}) is already reinforced.", target_x, target_y);
                false
            }
            _ => {
                println!("Cannot reinforce cell at ({}, {}). It must be occupied.", target_x, target_y);
                false
            }
        }
    }

    fn get_clicked_cell(&self) -> Option<(usize, usize)> {
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

    fn get_clicked_cell_on_own_board(&self) -> Option<(usize, usize)> {
        let (mouse_x, mouse_y) = mouse_position();
        let grid_x_offset = 150.0;
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

    fn place_ship(&mut self, ship_type: ShipType, orientation: Orientation) -> Option<Ship> {
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
}

/*-------- Main -------- */
#[macroquad::main("Battleships")]
async fn main() {
    request_new_screen_size(1280., 720.);

    let REINFORCE_SOUND = audio::load_sound("src/Sound/Welding sound Effects.wav").await.unwrap();
    let TORPEDO_SOUND = audio::load_sound("src/Sound/Torpedo - Free Sound Effect.wav").await.unwrap();
    let SONAR_SOUND = audio::load_sound("src/Sound/Submarine Sonar Ping Sound - Sonar Sound Effect.wav").await.unwrap();
    let SPLASH_SOUND = audio::load_sound("src/Sound/Splash Sound Effect.wav").await.unwrap();
    let MISSLE_SOUND = audio::load_sound("src/Sound/Sound Effect - Missile Launch.wav").await.unwrap();

    let mut player1 = Player::new();
    player1.deck.build();
    player1.deck.shuffle();

    let mut opponent = Player::new();
    opponent.deck.build();
    opponent.deck.shuffle();

    player1.place_ship(ShipType::Battleship, Orientation::Verticle);
    player1.place_ship(ShipType::Submarine, Orientation::Verticle);
    player1.place_ship(ShipType::Cruiser, Orientation::Horizontal);
    player1.place_ship(ShipType::Dreadnaught, Orientation::Verticle);
    player1.place_ship(ShipType::Destroyer, Orientation::Horizontal);

    player1.boardgrid.set_x_offset(macroquad_grid_dex::Position::Pixels(150.));
    player1.boardgrid.set_y_offset(macroquad_grid_dex::Position::Pixels(50.));
    player1.boardgrid.set_cell_bg_color(BLACK);
    player1.boardgrid.set_gap_color(GREEN);

    player1.guessgrid.set_x_offset(macroquad_grid_dex::Position::Pixels(screen_width()-100.));
    player1.guessgrid.set_y_offset(macroquad_grid_dex::Position::Pixels(50.));
    player1.guessgrid.set_cell_bg_color(BLACK);
    player1.guessgrid.set_gap_color(GREEN);

    opponent.boardgrid.set_x_offset(macroquad_grid_dex::Position::Pixels(150.));
    opponent.boardgrid.set_y_offset(macroquad_grid_dex::Position::Pixels(50.));
    opponent.boardgrid.set_cell_bg_color(BLACK);
    opponent.boardgrid.set_gap_color(GREEN);

    opponent.guessgrid.set_x_offset(macroquad_grid_dex::Position::Pixels(screen_width()-100.));
    opponent.guessgrid.set_y_offset(macroquad_grid_dex::Position::Pixels(50.));
    opponent.guessgrid.set_cell_bg_color(BLACK);
    opponent.guessgrid.set_gap_color(GREEN);

    opponent.place_ship(ShipType::Battleship, Orientation::Verticle);
    opponent.place_ship(ShipType::Submarine, Orientation::Verticle);
    opponent.place_ship(ShipType::Cruiser, Orientation::Horizontal);
    opponent.place_ship(ShipType::Dreadnaught, Orientation::Verticle);
    opponent.place_ship(ShipType::Destroyer, Orientation::Horizontal);

    let mut player1_turn = true;
    let mut player_acted = false;
    
    loop {
        clear_background(BLACK);

        if player1_turn {
            player1.boardgrid.draw();
            player1.guessgrid.draw();
        } else {
            opponent.boardgrid.draw();
            opponent.guessgrid.draw();
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            if !player_acted {
                if player1_turn {
                    if let Some((x, y)) = player1.get_clicked_cell() {
                        let hit = player1.fire_missile(&mut opponent, x, y);
                        println!("Missile {}", if hit { "hit!" } else { "missed." });
                        player_acted = true;
                        if hit {
                            audio::play_sound_once(&MISSLE_SOUND);
                        }else {
                            audio::play_sound_once(&SPLASH_SOUND);
                        }
                    }
                } else {
                    if let Some((x, y)) = opponent.get_clicked_cell() {
                        let hit = opponent.fire_missile(&mut player1, x, y);
                        println!("Missile {}", if hit { "hit!" } else { "missed." });
                        player_acted = true;
                        if hit {
                            audio::play_sound_once(&MISSLE_SOUND);
                        }else {
                            audio::play_sound_once(&SPLASH_SOUND);
                        }
                    }
                }
            } else {
                println!("Already used your action this turn!!");
            }
        }
        
        if is_key_pressed(KeyCode::T) {
            if !player_acted {
                audio::play_sound_once(&TORPEDO_SOUND);
                if player1_turn {
                    if let Some(target_x) = player1.get_torpedo_target_column() {
                        let hit = player1.fire_torpedo(&mut opponent, target_x);
                        println!("Torpedo {}", if hit { "hit!" } else { "missed." });
                        player_acted = true;
                        if hit {
                            // Put a explosion sound effect
                        }
                    }
                } else {
                    if let Some(target_x) = opponent.get_torpedo_target_column() {
                        let hit = opponent.fire_torpedo(&mut player1, target_x);
                        println!("Torpedo {}", if hit { "hit!" } else { "missed." });
                        player_acted = true;
                        if hit {
                            // Put a explosion sound effect
                        }
                    }
                }
            } else {
                println!("Already used your action this turn!!");
            }
        }
        
        if is_key_pressed(KeyCode::R) {
            if !player_acted {
                if player1_turn {
                    if let Some((x, y)) = player1.get_clicked_cell_on_own_board() {
                        let success = player1.reinforce(x, y);
                        println!("Reinforcement {}", if success { "successful!" } else { "failed." });
                        player_acted = true;
                        if success {
                            audio::play_sound_once(&REINFORCE_SOUND);
                        }
                    }
                } else {
                    if let Some((x, y)) = opponent.get_clicked_cell_on_own_board() {
                        let success = opponent.reinforce(x, y);
                        println!("Reinforcement {}", if success { "successful!" } else { "failed." });
                        player_acted = true;
                        if success {
                            audio::play_sound_once(&REINFORCE_SOUND);
                        }
                    }
                }
            } else {
                println!("Already used your action this turn!!");
            }
        }

        if is_key_pressed(KeyCode::S) {
            if !player_acted {
                audio::play_sound_once(&SONAR_SOUND);
                if player1_turn {
                    if let Some((x, y)) = player1.get_clicked_cell() {
                        player1.radar_scan(&mut opponent, x, y);
                        player_acted = true;
                    }
                } else {
                    if let Some((x, y)) = player1.get_clicked_cell() {
                        opponent.radar_scan(&mut player1, x, y);
                        player_acted = true;
                    }
                }
            } else {
                println!("Already used your action this turn!!");
            }
        }

        if is_key_pressed(KeyCode::Space) {
            if player_acted {
                println!("Player changed");
                player1_turn = !player1_turn;
                player_acted = false;
            }
        }

        next_frame().await;
    }
}