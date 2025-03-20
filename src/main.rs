// Imports

use macroquad_grid::Grid;
use macroquad::prelude::*;


// Constants
const GRID_SIZE:usize = 10;
const HAND_SIZE:usize = 3;
const DECK_SIZE: usize = 48;

// Structs and Enums
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
    
    fn change_cell(&mut self, x:usize,y:usize,ctype:Cells) {
        // Changes the provided cell to occupied
        self.cells[x][y] = ctype;
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
/* 
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
*/

// Main
#[macroquad::main("Battleships")]
async fn main() {
    let mut test_player_board = Board::new(); 
    let mut test_quess_board = Board::new();

    // Define Placeholder ships
    test_player_board.change_cell(1,2,Cells::Occupied);
    test_player_board.change_cell(2,2,Cells::Occupied);

    test_player_board.change_cell(7,5,Cells::Occupied);
    test_player_board.change_cell(8,5,Cells::Occupied);
    test_player_board.change_cell(9,5,Cells::Occupied);

    let test_grid = Grid::new(100.0,100.0,10,10,10.0);

    loop {
        clear_background(PURPLE);

        
        //test_grid.draw();

        next_frame().await
    }
}
