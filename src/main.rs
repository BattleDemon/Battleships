// Imports
use ::rand::prelude::{SliceRandom, IndexedRandom};
use macroquad::{audio, prelude::*};
extern crate macroquad_grid_dex;
use macroquad_grid_dex::Grid;

// Constants
const GRID_SIZE: usize = 10; 
const HAND_SIZE: usize = 3;
const DECK_SIZE: usize = 48;

// Sound Effects Constants (Bytes)
const REINFORCE_SOUND: &[u8] = include_bytes!("Sound/Reinforce(new version).wav");
const SONAR_SOUND: &[u8] = include_bytes!("Sound/Sonar(new version).wav");
const MISSLE_SOUND: &[u8] = include_bytes!("Sound/Sound Effect - Missile Launch.wav");
const SPLASH_SOUND: &[u8] = include_bytes!("Sound/Splash(new version).wav");
const TORPEDO_SOUND: &[u8] = include_bytes!("Sound/Torpedo(new version).wav");

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
#[derive(Copy, Clone, PartialEq)]
enum ActionType {
    Missle,     // Missle is the base battle ships fire ability
    Torpedo,    // Torpedo fires from a point on the x axis then shots upwards along the y axis
    Patrol,     // Allows the player to move a ship
    RadarScan,  // Reveals what is on the selected position and adjacent cells
    Reinforce,  // Gives a cell an extra life
}

// Keeps a vector of actions and used to randomly select them
struct Deck {
    deck_list: Vec<ActionType>,  
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
    patrol_mode: bool,
    patrol_ship: Option<usize>, // index of ship being moved
    patrol_frames: usize,       // frames remaining to wait for input
}

#[derive(PartialEq)]
enum GameState {
    Player1,
    Player2,
    Else,
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
                Cells::Miss => grid.color_cell(x, y, GRAY),
                Cells::Reinforced => grid.color_cell(x, y, DARKGREEN),
            }
            self.cells[x][y] = ctype;
        }  
    }
}

// Deck functions
impl Deck {
    // Deck Constructor
    fn new() -> Self {
        Deck {
            deck_list: Vec::with_capacity(DECK_SIZE),
        }
    }

    fn build(&mut self) {
        // Clear any existing cards
        self.deck_list.clear();
        
        // Add Missile cards (first 16 cards)
        for _ in 0..16 {
            self.deck_list.push(ActionType::Missle);
        }
        
        // Add Torpedo cards (next 9 cards)
        for _ in 0..9 {
            self.deck_list.push(ActionType::Torpedo);
        }
        
        // Add Patrol cards (next 8 cards)
        for _ in 0..8 {
            self.deck_list.push(ActionType::Patrol);
        }
        
        // Add Reinforce cards (next 7 cards)
        for _ in 0..7 {
            self.deck_list.push(ActionType::Reinforce);
        }
        
        // Add RadarScan cards (last 8 cards)
        for _ in 0..8 {
            self.deck_list.push(ActionType::RadarScan);
        }
    }

    fn shuffle(&mut self) {
        let mut rng = ::rand::rng();
        self.deck_list.shuffle(&mut rng);
    }
}

/*-------- Player Implementations -------- */
impl Player {
    // Player Constructor
    fn new() -> Self {
        let mut p = Player {
            board: Board::new(),
            boardgrid: Grid::new(400.0, 400.0, 10, 10, 1.0),
            guess_board: Board::new(),
            guessgrid: Grid::new(400.0, 400.0, 10, 10, 1.0),
            hand: Vec::new(),
            deck: Deck::new(),
            ships: Vec::new(),
            ship_count: 5,
            patrol_mode: false,
            patrol_ship: None,
            patrol_frames: 0,
        };

        p.deck.build();
        p.deck.shuffle();
        p.draw_hand();
        
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

    fn fire_torpedo(&mut self, opponent: &mut Player, target_y: usize) -> bool {
        let mut x = GRID_SIZE - 1;
        let mut hit_something = false;
    
        while x < GRID_SIZE {
            match opponent.board.cells[x][target_y] {
                Cells::Reinforced => {
                    self.guess_board.change_cell(x, target_y, Cells::Occupied, &mut self.guessgrid);
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
                    
                    // Check if this hit destroyed a ship
                    if let Some(ship_idx) = opponent.find_ship_at(x, target_y) {
                        if opponent.is_ship_destroyed(ship_idx) {
                            println!("Ship completely destroyed by torpedo!");
                            opponent.update_ship_count();
                        }
                    }
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

    fn start_patrol(&mut self, x: usize, y: usize) -> bool {
        // Find the ship at this position
        if let Some(ship_idx) = self.ships.iter().position(|ship| ship.positions.contains(&(x, y))) {
            let ship = &self.ships[ship_idx];
            
            // Check if any part of the ship is hit
            let has_hit = ship.positions.iter().any(|&(px, py)| {
                self.board.cells[px][py] == Cells::Hit
            });
    
            if has_hit {
                println!("Cannot move a ship that has been hit!");
                self.hand.push(ActionType::Patrol); // Return card to hand
                return false;
            }
    
            self.patrol_mode = true;
            self.patrol_ship = Some(ship_idx);
            self.patrol_frames = 30; // About 0.5 seconds at 60 FPS
            
            // Highlight ship
            for &(px, py) in &self.ships[ship_idx].positions {
                self.boardgrid.color_cell(px, py, YELLOW);
            }
            true
        } else {
            self.hand.push(ActionType::Patrol); // Return card to hand
            false
        }
    }

    fn try_patrol_move(&mut self, dir_x: isize, dir_y: isize) -> bool {
        if let Some(ship_idx) = self.patrol_ship {
            let ship = &mut self.ships[ship_idx];

            // Track which positions were reinforced
            let mut reinforced_positions = Vec::new();
            for &(x, y) in &ship.positions {
                if self.board.cells[x][y] == Cells::Reinforced {
                    reinforced_positions.push((x, y));
                }
            }

            // Calculate new positions
            let mut new_positions = Vec::new();
            for &(x, y) in &ship.positions {
                let new_x = (x as isize + dir_x) as usize;
                let new_y = (y as isize + dir_y) as usize;

                // Check bounds
                if new_x >= GRID_SIZE || new_y >= GRID_SIZE {
                    return false;
                }

                // Check if new position is already occupied (by another ship)
                if self.board.cells[new_x][new_y] == Cells::Occupied || 
                   self.board.cells[new_x][new_y] == Cells::Reinforced {
                    // Check if this is part of our own ship
                    if !ship.positions.contains(&(new_x, new_y)) {
                        return false;
                    }
                }

                new_positions.push((new_x, new_y));
            }

            // Clear old positions - change back to BLACK (default)
            for &(x, y) in &ship.positions {
                self.boardgrid.color_cell(x, y, BLACK);  // Set to black instead of changing cell type
                self.board.cells[x][y] = Cells::Empty;   // Still mark as empty in the backend
            }

            // Update ship positions
            ship.positions = new_positions;

            // Mark new positions as occupied or reinforced
            for &(x, y) in &ship.positions {
                // Check if this position was reinforced in the old location
                let was_reinforced = reinforced_positions.contains(&(
                    (x as isize - dir_x) as usize,
                    (y as isize - dir_y) as usize
                ));
                
                if was_reinforced {
                    self.board.change_cell(x, y, Cells::Reinforced, &mut self.boardgrid);
                } else {
                    self.board.change_cell(x, y, Cells::Occupied, &mut self.boardgrid);
                }
            }

            // Clean up patrol state
            self.cancel_patrol(false);
            true
        } else {
            false
        }
    }

    fn cancel_patrol(&mut self, return_to_hand: bool) {
        if let Some(ship_idx) = self.patrol_ship {
            // Remove highlight and reset to proper colors
            for &(x, y) in &self.ships[ship_idx].positions {
                match self.board.cells[x][y] {
                    Cells::Occupied => self.boardgrid.color_cell(x, y, GREEN),
                    Cells::Reinforced => self.boardgrid.color_cell(x, y, DARKGREEN),
                    Cells::Empty => self.boardgrid.color_cell(x, y, BLACK),
                    _ => {}
                }
            }
        }
        
        // Return Patrol card to hand if specified
        if return_to_hand {
            self.hand.push(ActionType::Patrol);
        }
        
        self.patrol_mode = false;
        self.patrol_ship = None;
        self.patrol_frames = 0;
    }


    fn update_patrol(&mut self) {
        if self.patrol_mode && self.patrol_frames > 0 {
            self.patrol_frames -= 1;
            
            if self.patrol_frames == 0 {
                self.cancel_patrol(true); // Return to hand
                println!("Patrol move timed out - card returned to hand");
            }
        }
    }
    fn draw_card(&mut self) -> Option<ActionType> {
        self.deck.deck_list.pop()
    }

    fn draw_hand(&mut self) {
        while self.hand.len() < HAND_SIZE {
            if let Some(card) = self.draw_card() {
                self.hand.push(card);
            } else {
                break; // No more cards in deck
            }
        }
    }

    fn has_card(&self, action_type: ActionType) -> bool {
        self.hand.contains(&action_type)
    }

    fn use_card(&mut self, action_type: ActionType) -> bool {
        if let Some(pos) = self.hand.iter().position(|&x| x == action_type) {
            self.hand.remove(pos);
            true
        } else {
            false
        }
    }

   // Returns the index of the ship at the given coordinates, if any
    fn find_ship_at(&self, x: usize, y: usize) -> Option<usize> {
        self.ships.iter()
            .position(|ship| ship.positions.contains(&(x, y)))
    }

    // Checks if a specific ship is completely destroyed
    fn is_ship_destroyed(&self, ship_idx: usize) -> bool {
        self.ships[ship_idx].positions.iter()
            .all(|&(x, y)| self.board.cells[x][y] == Cells::Hit)
    }

    // Updates the ship count based on which ships are still alive
    fn update_ship_count(&mut self) {
        self.ship_count = self.ships.iter()
            .filter(|ship| {
                // A ship is still alive if at least one of its cells isn't hit
                ship.positions.iter()
                    .any(|&(x, y)| self.board.cells[x][y] != Cells::Hit)
            })
            .count();
    }

}

fn draw_hand_to_screen(hand: &[ActionType], x: f32, y: f32) {
    for (i, card) in hand.iter().enumerate() {
        let card_x = x + (i as f32 * 70.0);
        let color = match card {
            ActionType::Missle => RED,
            ActionType::Torpedo => BLUE,
            ActionType::Patrol => YELLOW,
            ActionType::RadarScan => PURPLE,
            ActionType::Reinforce => GREEN,
        };
        
        draw_rectangle(card_x, y, 70.0, 100.0, color);
        draw_text(
            match card {
                ActionType::Missle => "Missile",
                ActionType::Torpedo => "Torpedo",
                ActionType::Patrol => "Patrol",
                ActionType::RadarScan => "Radar",
                ActionType::Reinforce => "Reinforce",
            },
            card_x + 2.0,
            y + 40.0,
            20.0,
            BLACK,
        );
    }
}

/*-------- Main -------- */
#[macroquad::main("Battleships")]
async fn main() {
    request_new_screen_size(1280., 720.);

    // Load sound effects from data
    let reinforce_sound: audio::Sound = audio::load_sound_from_bytes(REINFORCE_SOUND).await.unwrap();
    let torpedo_sound: audio::Sound = audio::load_sound_from_bytes(TORPEDO_SOUND).await.unwrap();
    let sonar_sound: audio::Sound = audio::load_sound_from_bytes(SONAR_SOUND).await.unwrap();
    let splash_sound: audio::Sound = audio::load_sound_from_bytes(SPLASH_SOUND).await.unwrap();
    let missle_sound: audio::Sound = audio::load_sound_from_bytes(MISSLE_SOUND).await.unwrap();

    // Create Player 1 
    let mut player1: Player = Player::new();

    // Create Player 2 (opponent)
    let mut opponent: Player = Player::new();

    let mut player1_turn: bool = true;
    let mut game_state: GameState = GameState::Player1;
    let mut player_acted: bool = false;
    let mut turncounter: f64 = 1.0;
    let mut player_won:i32 = 0;

    let mut twist: bool = true;

    loop {
        clear_background(BLACK);

        draw_text("Please Press SPACE to play with the twist on",(screen_width()/2.0)-350.0,(screen_height()/2.0)-30.0,40.0,WHITE);
        draw_text("Press ENTER to play without the twist",(screen_width()/2.0)-320.0,(screen_height()/2.0)-75.0,40.0,WHITE);

        if is_key_pressed(KeyCode::Space) {
            break;
        }

        if is_key_pressed(KeyCode::Enter) {
            twist = false;
            break;
        }

        next_frame().await;
    }
    
    loop {

        clear_background(BLACK);

        if game_state == GameState::Player1 {
            player1.boardgrid.draw();
            player1.guessgrid.draw();
            player1.update_patrol();
            draw_text("Player 1's turn", (screen_width()/2.0)-100.0, 45.0, 30.0, WHITE);
            if twist == true 
           { draw_hand_to_screen(&player1.hand, (screen_width()/2.0)-120.0, 500.0);}
        } else if game_state == GameState::Player2 {
            opponent.boardgrid.draw();
            opponent.guessgrid.draw();
            opponent.update_patrol();
            draw_text("Player 2's turn", (screen_width()/2.0)-120.0, 45.0, 30.0, WHITE);
            if twist == true
            {draw_hand_to_screen(&opponent.hand, (screen_width()/2.0)-100.0, 500.0);}
        } else {
            draw_text("Press Space to change player",(screen_width()/2.0)-350.0,(screen_height()/2.0)-30.0,60.0,WHITE);
        }

        if twist == true {

            if is_mouse_button_pressed(MouseButton::Left) {
                if !player_acted {
                    if game_state == GameState::Player1 {
                        if player1.use_card(ActionType::Missle) {
                            if let Some((x, y)) = player1.get_clicked_cell() {
                                let hit = player1.fire_missile(&mut opponent, x, y);
                                println!("Missile {}", if hit { "hit!" } else { "missed." });
                                player_acted = true;
                                if hit {
                                    audio::play_sound_once(&missle_sound);
                                }else {
                                    audio::play_sound_once(&splash_sound);
                                }
                            } else {
                                player1.hand.push(ActionType::Missle);
                            }
                        } else {
                            println!("You can't use that action since it isn't in your hand!");
                        }
                    } else {
                        if opponent.use_card(ActionType::Missle) {
                            if let Some((x, y)) = opponent.get_clicked_cell() {
                                let hit: bool = opponent.fire_missile(&mut player1, x, y);
                                println!("Missile {}", if hit { "hit!" } else { "missed." });
                                player_acted = true;
                                if hit {
                                    audio::play_sound_once(&missle_sound);
                                }else {
                                    audio::play_sound_once(&splash_sound);
                                }
                            }else {
                                opponent.hand.push(ActionType::Missle);
                            }
                        } else {
                            println!("You can't use that action since it isn't in your hand!");
                        }
                    }
                } else {
                    println!("Already used your action this turn!!");
                }
            }

            if is_key_pressed(KeyCode::T) {
                if !player_acted {
                    audio::play_sound_once(&torpedo_sound);
                    if game_state == GameState::Player1 {
                        if player1.use_card(ActionType::Torpedo){
                            if let Some(target_x) = player1.get_torpedo_target_column() {
                                let hit: bool = player1.fire_torpedo(&mut opponent, target_x);
                                println!("Torpedo {}", if hit { "hit!" } else { "missed." });
                                player_acted = true;
                                if hit {
                                    // Put a explosion sound effect
                                }
                            }else{
                                player1.hand.push(ActionType::Torpedo);
                            }
                        } else {
                            println!("You can't use that action since it isn't in your hand!");
                        }
                    } else {
                        if opponent.use_card(ActionType::Torpedo){
                            if let Some(target_x) = opponent.get_torpedo_target_column() {
                                let hit: bool = opponent.fire_torpedo(&mut player1, target_x);
                                println!("Torpedo {}", if hit { "hit!" } else { "missed." });
                                player_acted = true;
                                if hit {
                                    // Put a explosion sound effect
                                }
                            }else{
                                opponent.hand.push(ActionType::Torpedo);
                            }
                        } else {
                            println!("You can't use that action since it isn't in your hand!");
                        }
                    }
                } else {
                    println!("Already used your action this turn!!");
                }
            }
            
            if is_key_pressed(KeyCode::R) {
                if !player_acted {
                    if game_state == GameState::Player1 {
                        if player1.use_card(ActionType::Reinforce) {
                            if let Some((x, y)) = player1.get_clicked_cell_on_own_board() {
                                let success: bool = player1.reinforce(x, y);
                                println!("Reinforcement {}", if success { "successful!" } else { "failed." });
                                player_acted = true;
                                if success {
                                    audio::play_sound_once(&reinforce_sound);
                                } else {
                                    player1.hand.push(ActionType::Reinforce);
                                }
                            } else {
                                player1.hand.push(ActionType::Reinforce);
                            }
                        } else {
                            println!("You can't use that action since it isn't in your hand!");
                        }
                    } else {
                        if opponent.use_card(ActionType::Reinforce) {
                            if let Some((x, y)) = opponent.get_clicked_cell_on_own_board() {
                                let success: bool = opponent.reinforce(x, y);
                                println!("Reinforcement {}", if success { "successful!" } else { "failed." });
                                player_acted = true;
                                if success {
                                    audio::play_sound_once(&reinforce_sound);
                                } else {
                                    opponent.hand.push(ActionType::Reinforce);
                                }
                            }else{
                                opponent.hand.push(ActionType::Reinforce);
                            }
                        } else {
                            println!("You can't use that action since it isn't in your hand!");
                        }
                    }
                } else {
                    println!("Already used your action this turn!!");
                }
            }

            if is_key_pressed(KeyCode::S) {
                if !player_acted {
                    if game_state == GameState::Player1 {
                        if player1.use_card(ActionType::RadarScan) {
                            if let Some((x, y)) = player1.get_clicked_cell() {
                                player1.radar_scan(&mut opponent, x, y);
                                player_acted = true;
                                audio::play_sound_once(&sonar_sound);
                            }else{
                                player1.hand.push(ActionType::RadarScan);
                            }
                        } else {
                            println!("You can't use that action since it isn't in your hand!");
                        }
                    } else {
                        if opponent.use_card(ActionType::RadarScan){
                            if let Some((x, y)) = player1.get_clicked_cell() {
                                opponent.radar_scan(&mut player1, x, y);
                                player_acted = true;
                                audio::play_sound_once(&sonar_sound);
                            }else{
                                opponent.hand.push(ActionType::RadarScan);
                            }
                        } else {
                            println!("You can't use that action since it isn't in your hand!");
                        }
                    }
                } else {
                    println!("Already used your action this turn!!");
                }
            }

            if is_key_pressed(KeyCode::P) && !player_acted {
                if game_state == GameState::Player1 && !player1.patrol_mode {
                    if player1.use_card(ActionType::Patrol) {
                        if let Some((x, y)) = player1.get_clicked_cell_on_own_board() {
                            let started = player1.start_patrol(x, y);
                            if !started {
                                // start_patrol already returns the card if it fails
                                println!("Couldn't start patrol - card returned to hand");
                            }
                        } else {
                            player1.hand.push(ActionType::Patrol);
                            println!("No ship selected - card returned to hand");
                        }
                    }
                } else if !player1_turn && !opponent.patrol_mode {
                    if opponent.use_card(ActionType::Patrol) {
                        if let Some((x, y)) = opponent.get_clicked_cell_on_own_board() {
                            let started = opponent.start_patrol(x, y);
                            if !started {
                                // start_patrol already returns the card if it fails
                                println!("Couldn't start patrol - card returned to hand");
                            }
                        } else {
                            opponent.hand.push(ActionType::Patrol);
                            println!("No ship selected - card returned to hand");
                        }
                    }
                }
            }
            
            // Add arrow key handling (before the Space key check):
            if !player_acted {
                let (current_player, current_opponent) = if game_state == GameState::Player1 {
                    (&mut player1, &mut opponent)
                } else {
                    (&mut opponent, &mut player1)
                };
            
                if current_player.patrol_mode {
                    let dir = if is_key_pressed(KeyCode::Up) {
                        Some((-1, 0))
                    } else if is_key_pressed(KeyCode::Down) {
                        Some((1, 0))
                    } else if is_key_pressed(KeyCode::Left) {
                        Some((0, -1))
                    } else if is_key_pressed(KeyCode::Right) {
                        Some((0, 1))
                    } else {
                        None
                    };
            
                    if let Some((dir_x, dir_y)) = dir {
                        let success = current_player.try_patrol_move(dir_x, dir_y);
                        println!("Patrol move {}", if success { "successful!" } else { "failed." });
                        player_acted = success;
                    } 
                }
            }
        } else {
            if is_mouse_button_pressed(MouseButton::Left) {
                if !player_acted {
                    if game_state == GameState::Player1 {
                        if let Some((x, y)) = player1.get_clicked_cell() {
                            let hit = player1.fire_missile(&mut opponent, x, y);
                            println!("Missile {}", if hit { "hit!" } else { "missed." });
                            player_acted = true;
                            if hit {
                                audio::play_sound_once(&missle_sound);
                            }else {
                                audio::play_sound_once(&splash_sound);
                            }
                        } else {
                            player1.hand.push(ActionType::Missle);
                        } 
                    } else {
                        if let Some((x, y)) = opponent.get_clicked_cell() {
                            let hit: bool = opponent.fire_missile(&mut player1, x, y);
                            println!("Missile {}", if hit { "hit!" } else { "missed." });
                            player_acted = true;
                            if hit {
                                audio::play_sound_once(&missle_sound);
                            }else {
                                audio::play_sound_once(&splash_sound);
                            }
                        }else {
                            opponent.hand.push(ActionType::Missle);
                        }
                    
                    }
                } else {
                    println!("Already used your action this turn!!");
                }
            }
        }



        if is_key_pressed(KeyCode::Space) {
            if player_acted {
                println!("Player changed");
                println!(" ");
                if game_state == GameState::Player1 {
                    let newcard = player1.draw_card().unwrap();
                    player1.hand.push(newcard);
                } else {
                    let newcard = opponent.draw_card().unwrap();
                    opponent.hand.push(newcard);
                }
                game_state = GameState::Else;
                player_acted = false;
                turncounter += 1.0;
            } else {
                if game_state == GameState::Else {
                    if player1_turn {
                        game_state = GameState::Player2;
                        player1_turn = !player1_turn;
                    } else {
                        game_state = GameState::Player1;
                        player1_turn = !player1_turn;
                    }
                }
            }
        }
        
        // For Testing
        if is_key_pressed(KeyCode::K){
            player1.ship_count = 0;
        }
        if is_key_pressed(KeyCode::J){
            opponent.ship_count = 0;
        }

        // Win check
        if player1.ship_count == 0 {
            player_won = -1;
            break;
        }else if opponent.ship_count == 0 {
            player_won = 1;
            break;
        }
        
        let temp_turncounter = (turncounter/2.0).floor();
        
        draw_text(format!("Turn: {}", temp_turncounter).as_str(),75.0,45.0,30.0,WHITE);
        
        next_frame().await;
    }

    turncounter = turncounter /2.0;
    turncounter = turncounter.floor();
    loop {
        if player_won == -1 {
            clear_background(BLACK);
            draw_text("Player 2 Won!!", (screen_width()/2.0)-200.0, screen_height()/2.0, 60.0, WHITE);
            
        }
        else if player_won == 1 {
            clear_background(BLACK);
            draw_text("Player 1 Won!!", (screen_width()/2.0)-200.0, screen_height()/2.0, 60.0, WHITE);
        }

        if is_key_pressed(KeyCode::Space){
            break;
        }

        draw_text(format!("After {} turns",turncounter).as_str(),(screen_width()/2.0)-180.0,(screen_height()/2.0)+50.0,30.0,WHITE);
        
        next_frame().await;
    }
}