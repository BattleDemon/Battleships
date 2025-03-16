// Imports
use macroquad::prelude::*;

// Constants
const GRID_SIZE:usize = 10;
const HAND_SIZE:usize = 3;
const DECK_SIZE: usize = 48;

// Structs and Enums
// Cells used to keep track of the state of a cell/coordinate on the board
#[derive(Copy, Clone)]
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
    Battleship,
    Cruiser,
    Submarine,
    Destroyer,
    Dreadnaught,
}


// Track ships, its positions and orientation
#[derive(Clone)]
struct Ship {
    ship_type: ShipType,
    positions: Vec<(usize,usize)>,// Cords ship occupies
    orientation: Orientation,
}

// Tracks all types of actions 
#[derive(Copy, Clone)]
enum ActionType {
    Missle,
    Torpedo,
    Patrol,
    Reinforce,
    RadarScan,
    AirDefence,
}

// Keeps a vector of actions and used to randomly select them
struct Deck {
    deck_list: [ActionType; DECK_SIZE]
}

// Track all player related variables 
struct Player {
    board: Board,
    guess_board: Board,
    hand: Vec<ActionType>,
    deck: Deck,
    ships: Vec<Ship>,
    ship_count: usize,
}

// implements
// Board Functions
impl Board {
    // Board Constructor
    fn new() -> Self {
        Board {  
            cells: [[Cells::Empty; GRID_SIZE]; GRID_SIZE], // Initialise all cells as Empty
        }
    }
    
    fn place_ship() {

    }
}

// Ship functions
impl Ship {
    // ship constructor
    fn new(){
    }
}

impl Deck {
    // Deck Constructor
    fn new() -> Self {
        Deck {
            deck_list: [ActionType::Missle; DECK_SIZE]
        }
    }
    fn randomise_deck() {
        
    }

    fn shuffle() {

    }

    fn draw_card() {

    }
}

// Player functions
impl Player {
    // Player Constructor
    fn new() -> Self {
        Player{
            board: Board::new(),
            guess_board: Board::new(),
            hand: Vec::new(),
            deck: Deck::new(),
            ships: Vec::new(),
            ship_count: 5,
        }
    }
}

// Main
#[macroquad::main("Battleships")]
async fn main() {
    loop {

        next_frame().await
    }
}
