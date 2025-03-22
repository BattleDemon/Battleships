// Imports

extern crate macroquad_grid_dex;
use macroquad_grid_dex::Grid;
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
    Battleship, // ship size 4
    Cruiser, // ship size 3
    Submarine, // ship size 3
    Destroyer, // ship size 2
    Dreadnaught, // ship size 5
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
    RadarScan,
    Reinforce,
}

// Keeps a vector of actions and used to randomly select them
struct Deck {
    deck_list: [ActionType; DECK_SIZE]
    // deck contents
    // missle the most common ( 16/48 )
    // torpedo ( 10/48 )
    // patrol ( 8/48 )
    // Radarscan ( 7/48 )
    // Reinforce ( 7/48 )
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
    
    fn change_cell(&mut self, x:usize,y:usize,ctype:Cells,mut grid: Grid)-> Grid {
        // Changes the provided cell to occupied
        self.cells[x][y] = ctype;
        grid.color_cell(x, y, GRAY);

        grid
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
            deck_list: [ActionType::Missle; DECK_SIZE],
        }
    }

    fn shuffle() {
        // randomly select a permutation of the deck 
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
    request_new_screen_size(1280., 720.);

    let mut test_player_board = Board::new();
    let mut test_grid = Grid::new(400.0,400.0,10,10,1.0); 
    let mut test_quess_board = Board::new();
    let mut test_grid2 = Grid::new(400.0,400.0,10,10,1.0);

    // Define Placeholder ships
    test_grid = test_player_board.change_cell(0,2,Cells::Occupied,test_grid);
    test_grid = test_player_board.change_cell(0,3,Cells::Occupied,test_grid);

    test_grid = test_player_board.change_cell(7,5,Cells::Occupied,test_grid);
    test_grid = test_player_board.change_cell(8,5,Cells::Occupied,test_grid);
    test_grid = test_player_board.change_cell(9,5,Cells::Occupied,test_grid);

    // Place holder grids 
    test_grid.set_x_offset(macroquad_grid_dex::Position::Pixels((150.)));
    test_grid.set_y_offset(macroquad_grid_dex::Position::Pixels((50.)));
    test_grid.set_cell_bg_color(WHITE);

    
    test_grid2.set_x_offset(macroquad_grid_dex::Position::Pixels((screen_width()-100.)));
    test_grid2.set_y_offset(macroquad_grid_dex::Position::Pixels((50.)));
    test_grid2.set_cell_bg_color(WHITE);

    test_grid2.set_cell_text(2,2,Some("X"));

    loop {
        clear_background(WHITE);

        
        test_grid.draw();
        test_grid2.draw();

        next_frame().await
    }
}
