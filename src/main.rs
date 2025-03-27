// Imports
use ::rand::prelude::{SliceRandom,IndexedRandom};
use macroquad::{audio, prelude::*};
extern crate macroquad_grid_dex;
use macroquad_grid_dex::Grid;

// Constants
const GRID_SIZE:usize = 10; 
const HAND_SIZE:usize = 3;
const DECK_SIZE: usize = 48;

/* -------- Structs and Enum -------- */
// Cells used to keep track of the state of a cell/coordinate on the board
#[derive(Copy, Clone, PartialEq)]
enum Cells {
    Empty, // Nothig is on this cell or you don't know if something is there
    Occupied, // The cell has a ship
    Hit, // Hit a Ship
    Miss, // Fired and missed
}

// Board used to keep track of the four game boards 2 per player 
struct Board {
    cells:[[Cells; GRID_SIZE]; GRID_SIZE], // Track every cell in the board
}

// The orientation of the ships
#[derive(Clone)]
enum Orientation {
    Horizontal, // the ship is horizontal 
    Verticle, // the ship is verticle 
}

// Used to keep track of what type of ship 
#[derive(Clone)]
enum ShipType {
    Battleship, // ship size 4
    Cruiser, // ship size 3
    Submarine, // ship size 3
    Destroyer, // ship size 2
    Dreadnaught, // ship size 5
}

// Track ships, its positions and orientation
#[derive(Clone)]
struct Ship {
    ship_type: ShipType, // Tracks the type of ship. see above
    positions: Vec<(usize,usize)>,// Cords ship occupies
    orientation: Orientation, // Orientation of the ship used for ship generation
}

// Tracks all types of actions 
#[derive(Copy, Clone)]
enum ActionType {
    Missle, // Missle is the base battle ships fire ability
    Torpedo, // Torpedo fires from a point on the x axis then shots upwards along the y axis until it hits a occupied or already hit cell or leaves the grid
    Patrol, // Allows the player to move a ship with all of its cells (no cell has been hit) one direction in the x or y axis (intended to use when seen by radar)
    RadarScan, // reveals what is on the selected position and those adjacent to it 
    Reinforce, // unsure if i will add this but it would give a cell an extra life 
}

// Keeps a vector of actions and used to randomly select them
struct Deck {
    deck_list: [ActionType; DECK_SIZE]
    // deck contents
    // missle the most common ( 20/48 )
    // torpedo ( 6/48 ) 
    // patrol ( 8/48 )
    // Radarscan ( 7/48 )
    // Reinforce ( 7/48 )
}

// Track all player related variables 
struct Player {
    board: Board, // Players board
    boardgrid: Grid, // Players grid (displayable version of board)
    guess_board: Board, // board where player guesses the enemies ships
    guessgrid: Grid, // displayable version of above
    hand: Vec<ActionType>, // stores the action cards that can be selected to use once each turn
    deck: Deck, // Deck where action cards are drawn from 
    ships: Vec<Ship>, // list of ships attached to the player
    ship_count: usize, // number of ships with at least one cell (if 0 then game over you lost)
}

/*-------- Impl for Structs -------- */
// think of impl as been the functions inside a python class
// Board Functions 
impl Board {
    // Board Constructor
    fn new() -> Self {
        Board {  
            cells: [[Cells::Empty; GRID_SIZE]; GRID_SIZE], // Initialise all cells as Empty
        }
    }
    
    // Change the inputed cell to the provided cell type
    fn change_cell(&mut self, x:usize,y:usize,ctype:Cells,grid:&mut Grid) { 

        if self.cells[x][y] != Cells::Hit { // Stops the cell from changing if the cell on the board is already hit i will have to make a way around this to add patrol in the future
            match ctype {
                Cells::Empty => grid.color_cell(x,y ,DARKGRAY), // if the provided cell is empty it will make the displayed grid cell dark grey
                Cells::Occupied => grid.color_cell(x,y,GREEN), // if the provided cell is occupied it will display as green
                Cells::Hit => grid.color_cell(x, y, RED), // if the provided cell is hit it will display as red
                Cells::Miss => {grid.set_cell_text(x,y, Some("0")); // if the provided cell is miss it will show as grey with a 0 in it
                                grid.color_cell(x,y,GRAY); },
            }
            self.cells[x][y] = ctype; // changed the cell in the board to the provided cell
        }  
    }
}

// Ship functions
impl Ship {
    // ship constructor
    fn new(){
    }
}

// Deck functions
impl Deck {
    // Deck Constructor
    fn new() -> Self {
        Deck {
            deck_list: [ActionType::Missle; DECK_SIZE], // make a vector of size deck size
        }
    }

    fn build(&mut self) { // changes the values of the elements in the vector to its respective action card based of of the deck probability in the deck struct 
        let mut deck_pos = 17; // first card not a missle

        loop {
            if deck_pos <= 26 {
                self.deck_list[deck_pos] = ActionType::Torpedo; // makes 10 torpedos
            }
            else if deck_pos <= 34 {
                self.deck_list[deck_pos] = ActionType::Patrol; // makes 6 patrols
            }
            else if deck_pos <= 41 {
                self.deck_list[deck_pos] = ActionType::Reinforce; // makes 6 reinforces
            }
            else if deck_pos <= 47 {
                self.deck_list[deck_pos] = ActionType::RadarScan; // makes 6 radar scans
            }
            else if deck_pos == 48 {
                break; // breaks out of the loop
            }
            deck_pos += 1; // increase the deck pos then runs the loop
        }  
    }

    fn shuffle(&mut self,){
        // randomly select a permutation of the deck 
        let mut rng = ::rand::rng();
        self.deck_list.shuffle(&mut rng); // randomly selct a permutation of the deck list vector 
    }

    fn draw_card() {

    }
}

/*-------- Player Implementations -------- */
impl Player {
    // Player Constructor
    fn new() -> Self {
        Player{
            board: Board::new(), // creates an instance of the board struct
            boardgrid: Grid::new(400.0,400.0,10,10,1.0), // creates a grid with the provided values
            guess_board: Board::new(), // creates an instance of the board struct
            guessgrid: Grid::new(400.0,400.0,10,10,1.0), // creates a grid with the provided values
            hand: Vec::new(), // Vector to store what actions are in the hand
            deck: Deck::new(), // creates an instance of the deck struct
            ships: Vec::new(), // creates a vectore of ship structs
            ship_count: 5,
        }
    }

    // Base fire type used in every battleship game
    fn fire_missile(&mut self, opponent: &mut Player , target_x: usize, target_y: usize) {
        // create local mutable cell for your opponent 
        let ocell = &mut opponent.board.cells[target_x][target_y]; // refrence to cell cords provided

        // Check if your opponents cell is occupied if so then muts it to be a hit
        if *ocell == Cells::Occupied {
            // if cell is occupied then it will change it to a hit for the player and opponent
            self.guess_board.change_cell(target_x, target_y, Cells::Hit, &mut self.guessgrid); 
            opponent.board.change_cell(target_x,target_y,Cells::Hit,&mut opponent.boardgrid);
            println!("Hit!");
        } else { // muts it to be a miss
            // if cell is empty then it changes to display that it is a miss
            self.guess_board.change_cell(target_x,target_y,Cells::Miss,&mut self.guessgrid);
            opponent.board.change_cell(target_x,target_y,Cells::Miss,&mut opponent.boardgrid);
            println!("Miss!");
        }
    }

    // Twist new firing func - shot up from a x pos until hit occupied or hit cell or leaves the grid
    fn fire_torpedo(&mut self, opponent: &mut Player, target_y: usize) {
        let mut x = GRID_SIZE - 1; // Start at the bottom row
    
        while x < GRID_SIZE {
            match opponent.board.cells[x][target_y] {
                Cells::Occupied => {
                    self.guess_board.change_cell(x, target_y, Cells::Hit, &mut self.guessgrid);
                    opponent.board.change_cell(x, target_y, Cells::Hit, &mut opponent.boardgrid);
                    println!("Torpedo hit!");
                    break;
                }
                Cells::Hit => {
                    println!("Torpedo stopped! Already hit here.");
                    break; // Stop if it reaches a previously hit ship
                }
                _ => {
                    self.guess_board.change_cell(x, target_y, Cells::Miss, &mut self.guessgrid);
                    opponent.board.change_cell(x, target_y, Cells::Miss, &mut opponent.boardgrid);
                    println!("Torpedo missed!");
                }
            }
    
            if x == 0 { break; } // Stop before underflowing
            x -= 1; // Move upwards
        }
    }

    fn get_torpedo_target_column(&self) -> Option<usize> {
        let (mouse_x, mouse_y) = mouse_position();

        let grid_x_offset = 700.0; // Grid offset for the guessboard
        let grid_y_offset = 50.0;
        let cell_size = 40.0; // Cell size, same as before

        let grid_size_px = cell_size * GRID_SIZE as f32;

        // Check if the mouse click is within the bounds of the grid
        if mouse_x >= grid_x_offset && mouse_x < grid_x_offset + grid_size_px &&
           mouse_y >= grid_y_offset && mouse_y < grid_y_offset + grid_size_px {
            let x = ((mouse_y - grid_y_offset) / cell_size) as usize;  // Row (Y)
            let y = ((mouse_x - grid_x_offset) / cell_size) as usize;  // Column (X)
            return Some(y); // Return the column where the torpedo will fire
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
    
                // Update the guess board to reflect the revealed state
                self.guess_board.change_cell(ux, uy, cell, &mut self.guessgrid);
            }
        }
        println!("Radar scan complete!");
    }

    fn check_hit(&self, target_x: usize, target_y: usize) -> bool {
        self.guess_board.cells[target_x][target_y] == Cells::Occupied
    }

    fn get_clicked_cell(&self) -> Option<(usize, usize)> {
        let (mouse_x, mouse_y) = mouse_position();
        
        let grid_x_offset = 700.0;
        let grid_y_offset = 50.0;
        let cell_size = 40.0;  // This should match your grid cell size
        let grid_size_px = cell_size * GRID_SIZE as f32;
    
        // Check if the mouse is within the bounds of the grid
        if mouse_x >= grid_x_offset && mouse_x < grid_x_offset + grid_size_px &&
           mouse_y >= grid_y_offset && mouse_y < grid_y_offset + grid_size_px {
            // Swap x and y calculation to fix the issue
            let x = ((mouse_y - grid_y_offset) / cell_size) as usize;  // Use mouse_y for x
            let y = ((mouse_x - grid_x_offset) / cell_size) as usize;  // Use mouse_x for y
            return Some((x, y));
        }
    
        None
    }

    fn place_ship(&mut self, ship_type: ShipType, orientation: Orientation) -> Option<Ship> {
        let mut rng = ::rand::rng(); // Corrected RNG call
        
        let ship_length = match ship_type {
            ShipType::Battleship => 4,
            ShipType::Cruiser => 3,
            ShipType::Submarine => 3,
            ShipType::Destroyer => 2,
            ShipType::Dreadnaught => 5,
        };
        
        let possible_pos: Vec<usize> = (0..GRID_SIZE).collect();
        
        for _ in 0..100 { // Try up to 100 times to find a valid placement
            let tempx = possible_pos.choose(&mut rng);
            let tempy = possible_pos.choose(&mut rng);
            let mut x: usize = *tempx.unwrap();
            let mut y: usize = *tempy.unwrap();
    
            // Determine possible movement directions
            let mut directions = match orientation {
                Orientation::Horizontal => vec![(0, 1), (0, -1)], // Right, Left (y changes)
                Orientation::Verticle => vec![(1, 0), (-1, 0)],   // Down, Up (x changes)
            };
            
            directions.shuffle(&mut rng); // Randomize direction order
            let (dx, dy) = directions[0]; // Pick a random direction
            
            let mut positions = vec![];
    
            // Check if ship fits within bounds for the chosen direction
            let mut fits = true;
            let mut temp_x = x;
            let mut temp_y = y;
    
            for _ in 0..ship_length {
                // Check if out of bounds at any step
                if temp_x >= GRID_SIZE || temp_y >= GRID_SIZE {
                    fits = false;
                    break;
                }
                positions.push((temp_x, temp_y));
                temp_x = (temp_x as isize + dx) as usize;
                temp_y = (temp_y as isize + dy) as usize;
            }
    
            if !fits {
                continue; // Try again with a new position
            }
    
            // If space is occupied, restart the process
            if positions.iter().any(|&(px, py)| self.board.cells[px][py] != Cells::Empty) {
                continue;
            }
    
            // Place the ship using change_cell
            for &(sx, sy) in &positions {
                self.board.change_cell(sx, sy, Cells::Occupied, &mut self.boardgrid);
            }
    
            let ship = Ship {
                ship_type,
                positions,
                orientation,
            };
            
            self.ships.push(ship.clone()); // Track ship in player's list
            return Some(ship); // Successfully placed the ship
        }
    
        None // If placement failed after 100 retries
    }
}


/*-------- Main -------- */
#[macroquad::main("Battleships")]
async fn main() {
    request_new_screen_size(1280., 720.); // change screen size

    //let bgm = audio::load_sound("src/Bismarck.wav").await.unwrap();
    //let bgm_params = audio::PlaySoundParams{looped:true,volume:1.};
    //audio::play_sound(&bgm, bgm_params);

    // make player
    let mut player1 = Player::new();
    player1.deck.build(); // build the deck
    player1.deck.shuffle(); // shuffle the deck

    // make opponent
    let mut opponent = Player::new();
    opponent.deck.build(); // build the deck
    opponent.deck.shuffle(); // shuffle the deck

    // Define Placeholder ships player 1
    player1.place_ship(ShipType::Battleship,Orientation::Verticle);
    player1.place_ship(ShipType::Submarine,Orientation::Verticle);
    player1.place_ship(ShipType::Cruiser,Orientation::Horizontal);
    player1.place_ship(ShipType::Dreadnaught,Orientation::Verticle);
    player1.place_ship(ShipType::Destroyer,Orientation::Horizontal);

    // Change the positon of the boards
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


    // opponent enemy ships
    opponent.place_ship(ShipType::Battleship,Orientation::Verticle);
    opponent.place_ship(ShipType::Submarine,Orientation::Verticle);
    opponent.place_ship(ShipType::Cruiser,Orientation::Horizontal);
    opponent.place_ship(ShipType::Dreadnaught,Orientation::Verticle);
    opponent.place_ship(ShipType::Destroyer,Orientation::Horizontal);

    let mut player1_turn = true;
    let mut player_acted:bool = false;
    loop {
        clear_background(BLACK);

        if player1_turn == true {
            player1.boardgrid.draw();
            player1.guessgrid.draw();
        }
        else {
            opponent.boardgrid.draw();
            opponent.guessgrid.draw();
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            if player_acted == false {
                if player1_turn {
                    if let Some((x, y)) = player1.get_clicked_cell() {
                        player1.fire_missile(&mut opponent, x, y);
                        player_acted = true;
                    }
                } else {
                    if let Some((x, y)) = opponent.get_clicked_cell() {
                        opponent.fire_missile(&mut player1, x, y);
                        player_acted = true;
                    }
                }
            } else {
                println!("Already used your action this turn!!");
            }
        }

        if is_key_pressed(KeyCode::T) {  // Press "T" to fire a torpedo
            if player_acted == false {
                if player1_turn {
                    if let Some(target_x) = player1.get_torpedo_target_column() {
                        player1.fire_torpedo(&mut opponent, target_x);
                        player_acted = true;
                    }
                } else {
                    if let Some(target_x) = opponent.get_torpedo_target_column() {
                        opponent.fire_torpedo(&mut player1, target_x);
                        player_acted = true;
                    }
                }
            }else {
                println!("Already used your action this turn!!");
            }
        }

        if is_key_pressed(KeyCode::R) {
            if player_acted == false {
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
            }else {
                println!("Already used your action this turn!!");
            }
        }

        if is_key_pressed(KeyCode::Space) {
            if player_acted == true {
                println!("Player changed");
                player1_turn = !player1_turn;
                player_acted = false;
            }
        }

        next_frame().await
    }
}


// Allow user to turn on and off twist