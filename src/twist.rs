//! implements the Twisted game mode with action cards and enhanced gameplay mechanics.
//! Extends the base Battleship logic with strategic abilities like ship movement and special attacks.

 /* ------ Import Used Libraries ------ */
 // Import the base game
use super::base::*;
// Random library
use ::rand::prelude::SliceRandom;
// Graphics library
use macroquad:: prelude::*;
// A module I recompiled and made small fixes to, but did not write. Used for grid graphics and logic.
extern crate macroquad_grid_dex;

/*------ Constants ------ */
/// Size of the hand
pub const HAND_SIZE: usize = 3; 
/// Total number of cards in the deck (Missile/Torpedo/Patrol/Reinforce/RadarScan).
pub const DECK_SIZE: usize = 48;

/*------ Enums and Structs ------ */
/// Types of action cards available in the Twisted mode.
#[derive(Clone, Copy,PartialEq)] // Copy - Enables bitwise copying of the type, doesn't move ownership. Clone - Creates a deep copy of the value, can proform complex copying. PartialEq - Allows comparison of this type.
pub enum ActionType {
    Missile,     // Missle is the base battle ships fire ability
    Torpedo,    // Torpedo fires from a point on the x axis then shots upwards along the y axis
    Patrol,     // Allows the player to move a ship
    RadarScan,  // Reveals what is on the selected position and adjacent cells
    Reinforce,  // Gives a cell an extra life
}

/// Manages the deck of action cards.
pub struct Deck { 
    pub deck_list: Vec<ActionType>, // All cards (shuffled during gameplay)
}

/// Extends BasePlayer with Twisted mode features.
pub struct TwistPlayer {
    pub base: BasePlayer, // Inherits core Battleship logic

    pub deck: Deck, // Drawable action cards
    pub hand: Vec<ActionType>, // Current available actions

    // Patrol system state
    pub patrol_mode: bool, // True when moving a ship
    pub patrol_ship: Option<usize>, // Index of moving ship
    pub patrol_frames: usize, // Time limit for patrol action
}

/* ------ Struct Implementations ------ */
// Implementation for Deck struct 
impl Deck {
    /// Deck constructor for an empty deck.
    pub fn new() -> Self {
        Deck {
            deck_list: Vec::with_capacity(DECK_SIZE),
        }
    }

    /// Populates the deck with cards according to Twisted mode rules:
    /// - 16 Missiles (most frequent basic action)
    /// - 9 Torpedoes (powerful vertical strikes)
    /// - 8 Patrols (ship movement)
    /// - 7 Reinforcements (defensive buffs)
    /// - 8 RadarScans (information gathering)
    pub fn build(&mut self) {
        // Clear any existing cards
        self.deck_list.clear();
        
        // Add Missile cards (first 16 cards)
        for _ in 0..16 {
            self.deck_list.push(ActionType::Missile);
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

    /// Randomises card order using rusts shuffle func
    pub fn shuffle(&mut self) {
        let mut rng = ::rand::rng();
        self.deck_list.shuffle(&mut rng);
    }
}

// Implementation for the Twisted version of the player
impl TwistPlayer {
    /// TwistPlayer constructor
    /// Initializes a Twisted mode player with:
    /// - Randomized ship placement (from BasePlayer)
    /// - Shuffled deck of action cards
    /// - Empty starting hand (filled via draw_hand())
    pub fn new(base_player: BasePlayer) -> Self {
        let mut p = TwistPlayer {
            base: base_player,

            deck: Deck::new(),
            hand: Vec::new(),

            patrol_mode: false,
            patrol_ship: None,
            patrol_frames: 0,
        };

        p.deck.build();
        p.deck.shuffle();
        p.draw_hand();

        return p;
    }

    /// Fires a torpedo attack along a vertical column:
    /// 1. Starts at bottom of grid (x = GRID_SIZE - 1)
    /// 2. Moves upward until hitting a ship or leaving grid
    /// 3. Updates both players' boards visually
    /// Returns true if any target was hit.
    pub fn fire_torpedo(&mut self, opponent: &mut TwistPlayer, target_y: usize) -> bool {
        let mut x = crate::base::GRID_SIZE - 1;
        let mut hit_something = false;
    
        while x < GRID_SIZE {
            match opponent.base.board.cells[x][target_y] {
                Cells::Reinforced => {
                    self.base.guess_board.change_cell(x, target_y, Cells::Occupied, &mut self.base.guessgrid);
                    opponent.base.board.change_cell(x, target_y, Cells::Occupied, &mut opponent.base.boardgrid);
                    println!("Torpedo hit a reinforced cell! Protection removed.");
                    hit_something = true;
                    break;
                }
                Cells::Occupied => {
                    self.base.guess_board.change_cell(x, target_y, Cells::Hit, &mut self.base.guessgrid);
                    opponent.base.board.change_cell(x, target_y, Cells::Hit, &mut opponent.base.boardgrid);
                    println!("Torpedo hit!");
                    hit_something = true;
                    
                    // Check if this hit destroyed a ship
                    if let Some(ship_idx) = opponent.base.find_ship_at(x, target_y) {
                        if opponent.base.is_ship_destroyed(ship_idx) {
                            println!("Ship completely destroyed by torpedo!");
                            opponent.base.update_ship_count();
                        }
                    }
                    break;
                }
                Cells::Hit => {
                    println!("Torpedo stopped! Already hit here.");
                    break;
                }
                _ => {
                    self.base.guess_board.change_cell(x, target_y, Cells::Miss, &mut self.base.guessgrid);
                    opponent.base.board.change_cell(x, target_y, Cells::Miss, &mut opponent.base.boardgrid);
                    println!("Torpedo missed!");
                }
            }
    
            if x == 0 { break; }
            x -= 1;
        }
    
        hit_something
    }

    /// Gets vertical column for torpedo attacks:
    /// - Maps mouse X-position to guess grid columns
    /// - Returns `Some(usize)` (0-9) if click within right-side grid
    /// - Used exclusively for torpedo targeting
    pub fn get_torpedo_target_column(&self) -> Option<usize> {
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

    /// Scans a 3x3 area centered on (target_x, target_y):
    /// - Reveals cell states on guess board
    /// - Works even at grid edges (ignores out-of-bounds cells)
    pub fn radar_scan(&mut self, opponent: &mut TwistPlayer, target_x: usize, target_y: usize) {
        let offsets = [(0, 0), (0, 1), (0, -1), (1, 0), (-1, 0)];
    
        for &(dx, dy) in &offsets {
            let nx = target_x as isize + dx;
            let ny = target_y as isize + dy;
    
            if nx >= 0 && nx < GRID_SIZE as isize && ny >= 0 && ny < GRID_SIZE as isize {
                let ux = nx as usize;
                let uy = ny as usize;
                let cell = opponent.base.board.cells[ux][uy];
    
                self.base.guess_board.change_cell(ux, uy, cell, &mut self.base.guessgrid);
            }
        }
        println!("Radar scan complete!");
    }

    /// Attempts to reinforce a ship cell:
    /// - Only works on Occupied cells
    /// - Fails if cell is already Reinforced
    /// Returns success status.
    pub fn reinforce(&mut self, target_x: usize, target_y: usize) -> bool {
        let current_state = self.base.board.cells[target_x][target_y];
        
        match current_state {
            Cells::Occupied => {
                self.base.board.change_cell(target_x, target_y, Cells::Reinforced, &mut self.base.boardgrid);
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

    /// Converts mouse position to grid coordinates on the player's OWN board:
    /// - Uses different grid offset than guess board
    /// - Returns `Some((x, y))` if within placement grid bounds
    /// - Used for ship reinforcement and patrol selection
    pub fn get_clicked_cell_on_own_board(&self) -> Option<(usize, usize)> {
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

    /// Enters patrol mode for ship at (x,y):
    /// 1. Validates ship exists and isn't damaged
    /// 2. Highlights ship with yellow cells
    /// 3. Starts 0.5-second timer (30 frames)
    /// Returns false if invalid selection.
    pub fn start_patrol(&mut self, x: usize, y: usize) -> bool {
        // Find the ship at this position
        if let Some(ship_idx) = self.base.ships.iter().position(|ship| ship.positions.contains(&(x, y))) {
            let ship = &self.base.ships[ship_idx];
            
            // Check if any part of the ship is hit
            let has_hit = ship.positions.iter().any(|&(px, py)| {
                self.base.board.cells[px][py] == Cells::Hit
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
            for &(px, py) in &self.base.ships[ship_idx].positions {
                self.base.boardgrid.color_cell(px, py, YELLOW);
            }
            true
        } else {
            self.hand.push(ActionType::Patrol); // Return card to hand
            false
        }
    }

    /// Attempts to move ship in patrol mode:
    /// - Checks new positions are within bounds
    /// - Prevents overlapping with other ships
    /// - Preserves Reinforced status during movement
    /// Returns success status.
    pub fn try_patrol_move(&mut self, dir_x: isize, dir_y: isize) -> bool {
        if let Some(ship_idx) = self.patrol_ship {
            let ship = &mut self.base.ships[ship_idx];

            // Track which positions were reinforced
            let mut reinforced_positions = Vec::new();
            for &(x, y) in &ship.positions {
                if self.base.board.cells[x][y] == Cells::Reinforced {
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
                if self.base.board.cells[new_x][new_y] == Cells::Occupied || 
                   self.base.board.cells[new_x][new_y] == Cells::Reinforced {
                    // Check if this is part of our own ship
                    if !ship.positions.contains(&(new_x, new_y)) {
                        return false;
                    }
                }

                new_positions.push((new_x, new_y));
            }

            // Clear old positions - change back to BLACK (default)
            for &(x, y) in &ship.positions {
                self.base.boardgrid.color_cell(x, y, BLACK);  // Set to black instead of changing cell type
                self.base.board.cells[x][y] = Cells::Empty;   // Still mark as empty in the backend
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
                    self.base.board.change_cell(x, y, Cells::Reinforced, &mut self.base.boardgrid);
                } else {
                    self.base.board.change_cell(x, y, Cells::Occupied, &mut self.base.boardgrid);
                }
            }

            // Clean up patrol state
            self.cancel_patrol(false);
            true
        } else {
            false
        }
    }

    /// Clears patrol mode state:
    /// - Removes ship highlights
    /// - Resets tracking variables
    /// - `return_to_hand`: If true, returns Patrol card to hand
    pub fn cancel_patrol(&mut self, return_to_hand: bool) {
        if let Some(ship_idx) = self.patrol_ship {
            // Remove highlight and reset to proper colors
            for &(x, y) in &self.base.ships[ship_idx].positions {
                match self.base.board.cells[x][y] {
                    Cells::Occupied => self.base.boardgrid.color_cell(x, y, GREEN),
                    Cells::Reinforced => self.base.boardgrid.color_cell(x, y, DARKGREEN),
                    Cells::Empty => self.base.boardgrid.color_cell(x, y, BLACK),
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

    /// Updates patrol mode countdown timer:
    /// - Decrements `patrol_frames` each frame (at ~60 FPS)
    /// - Automatically cancels patrol if timer expires
    /// - Returns patrol card to hand on timeout
    pub fn update_patrol(&mut self) {
        if self.patrol_mode && self.patrol_frames > 0 {
            self.patrol_frames -= 1;
            
            if self.patrol_frames == 0 {
                self.cancel_patrol(true); // Return to hand
                println!("Patrol move timed out - card returned to hand");
            }
        }
    }

    /// Draws one card from the top of the deck.
    /// - Returns `Some(ActionType)` if cards remain
    /// - Returns `None` if deck is empty (game should handle reshuffling)
    pub fn draw_card(&mut self) -> Option<ActionType> {
        self.deck.deck_list.pop()
    }

    /// Draws cards until hand contains HAND_SIZE cards.
    /// Handles empty deck gracefully (no infinite loops).
    pub fn draw_hand(&mut self) {
        while self.hand.len() < HAND_SIZE {
            if let Some(card) = self.draw_card() {
                self.hand.push(card);
            } else {
                break; // No more cards in deck
            }
        }
    }

    /// Attempts to use a specified action card from the player's hand.
    /// - `action_type`: The card to play (e.g., ActionType::Torpedo)
    /// - Returns `true` if card was found and removed from hand
    /// - Returns `false` if card isn't available (prevents invalid actions)
    pub fn use_card(&mut self, action_type: ActionType) -> bool {
        if let Some(pos) = self.hand.iter().position(|&x| x == action_type) {
            self.hand.remove(pos);
            true
        } else {
            false
        }
    }
}

/// Renders action cards at bottom of screen:
/// - Missile: Red
/// - Torpedo: Blue
/// - Patrol: Yellow
/// - RadarScan: Purple
/// - Reinforce: Green
pub fn draw_hand_to_screen(hand: &[ActionType], x: f32, y: f32) {
    for (i, card) in hand.iter().enumerate() {
        let card_x = x + (i as f32 * 70.0);
        let color = match card {
            ActionType::Missile => RED,
            ActionType::Torpedo => BLUE,
            ActionType::Patrol => YELLOW,
            ActionType::RadarScan => PURPLE,
            ActionType::Reinforce => GREEN,
        };
        
        draw_rectangle(card_x, y, 70.0, 100.0, color);
        draw_text(
            match card {
                ActionType::Missile => "Missile",
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

