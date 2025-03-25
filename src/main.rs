// Imports
use ::rand::prelude::SliceRandom;
use macroquad::prelude::*;
extern crate macroquad_grid_dex;
use macroquad_grid_dex::Grid;

// Constants
const GRID_SIZE:usize = 10; 
const HAND_SIZE:usize = 3;
const DECK_SIZE: usize = 48;

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
    boardgrid: Grid,
    guess_board: Board,
    guessgrid: Grid,
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
    
    fn change_cell(&mut self, x:usize,y:usize,ctype:Cells,grid:&mut Grid) {
        // Changes the provided cell to occupied
        self.cells[x][y] = ctype;

        match ctype {
            Cells::Empty => grid.color_cell(x,y ,BLACK),
            Cells::Occupied => grid.color_cell(x,y,GREEN),
            Cells::Hit => grid.color_cell(x, y, RED),
            Cells::Miss => {grid.set_cell_text(x,y, Some("0"));
                            grid.color_cell(x,y,GRAY); },
        }

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

    fn build(&mut self) {
        let mut deck_pos = 17; // first card not a missle

        loop {
            if deck_pos <= 26 {
                self.deck_list[deck_pos] = ActionType::Torpedo;
            }
            else if deck_pos <= 34 {
                self.deck_list[deck_pos] = ActionType::Patrol;
            }
            else if deck_pos <= 41 {
                self.deck_list[deck_pos] = ActionType::Reinforce;
            }
            else if deck_pos <= 47 {
                self.deck_list[deck_pos] = ActionType::RadarScan;
            }
            else if deck_pos == 48 {
                break;
            }
            deck_pos += 1;
        }  
    }

    fn shuffle(&mut self,){
        // randomly select a permutation of the deck 
        let mut rng = ::rand::rng();
        self.deck_list.shuffle(&mut rng);
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
            boardgrid: Grid::new(400.0,400.0,10,10,1.0),
            guess_board: Board::new(),
            guessgrid: Grid::new(400.0,400.0,10,10,1.0),
            hand: Vec::new(),
            deck: Deck::new(),
            ships: Vec::new(),
            ship_count: 5,
        }
    }

    fn fire_missile(&mut self, opponent: &mut Player , target_x: usize, target_y: usize) {
        // create local mutable cell for both self and your opponent
        let cell = &mut self.guess_board.cells[target_x][target_y];
        let ocell = &mut opponent.board.cells[target_x][target_y];

        // Check if your opponents cell is occupied if so then muts it to be a hit
        if *ocell == Cells::Occupied {
            self.guess_board.change_cell(target_x, target_y, Cells::Hit, &mut self.guessgrid);
            opponent.board.change_cell(target_x,target_y,Cells::Hit,&mut opponent.boardgrid);
            println!("Hit!");
        } else { // muts it to be a miss
            self.guess_board.change_cell(target_x,target_y,Cells::Miss,&mut self.guessgrid);
            opponent.board.change_cell(target_x,target_y,Cells::Miss,&mut opponent.boardgrid);
            println!("Miss!");
        }
    }

    fn check_hit(&self, target_x: usize, target_y: usize) -> bool {
        self.guess_board.cells[target_x][target_y] == Cells::Occupied
    }
}


// Main
#[macroquad::main("Battleships")]
async fn main() {
    request_new_screen_size(1280., 720.); // change screen size

    // make player
    let mut player1 = Player::new();
    player1.deck.build(); // build the deck
    player1.deck.shuffle(); // shuffle the deck

    // make opponent
    let mut opponent = Player::new();
    opponent.deck.build(); // build the deck
    opponent.deck.shuffle(); // shuffle the deck

    // Define Placeholder ships player 1
    player1.board.change_cell(0,2,Cells::Occupied,&mut player1.boardgrid);
    player1.board.change_cell(0,3,Cells::Occupied,&mut player1.boardgrid);

    player1.board.change_cell(7,5,Cells::Occupied,&mut player1.boardgrid);
    player1.board.change_cell(8,5,Cells::Occupied,&mut player1.boardgrid);
    player1.board.change_cell(9,5,Cells::Occupied,&mut player1.boardgrid);

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
    opponent.board.change_cell(3,3,Cells::Occupied,&mut opponent.boardgrid);

    player1.fire_missile(&mut opponent, 3, 3);
    player1.fire_missile(&mut opponent, 5,2);

    let mut player1_turn = true;
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

        if is_key_pressed(KeyCode::Q) {
            println!("key down");
            player1_turn = !player1_turn;
        }
        
        if is_key_pressed(KeyCode::A) {
            let nums: Vec<usize> = (0..10).collect();
            let rng = ::rand::rng();
            nums.shuffle(&mut rng);
            let x = nums.choose(&mut rng);
            let y = nums.choose(&mut rng);
            player1.fire_missile(&mut opponent,x,y);
        }

        next_frame().await
    }
}
