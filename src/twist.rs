use super::base::*;
use macroquad::{audio, prelude::*};
extern crate macroquad_grid_dex;
use macroquad_grid_dex::Grid;

const HAND_SIZE: usize = 3;
const DECK_SIZE: usize = 48;

pub enum ActionType {
    Missle,     // Missle is the base battle ships fire ability
    Torpedo,    // Torpedo fires from a point on the x axis then shots upwards along the y axis
    Patrol,     // Allows the player to move a ship
    RadarScan,  // Reveals what is on the selected position and adjacent cells
    Reinforce,  // Gives a cell an extra life
}

pub struct Deck {
    pub deck_list: Vec<ActionType>,
}

pub struct TwistPlayer {
    pub base: BasePlayer,

    pub deck: Deck,
    pub hand: Vec<ActionType>,

    pub patrol_mode: bool,
    pub patrol_ship: Option<usize>,
    pub patrol_frames: usize,
}

impl Deck {
    pub fn new() -> Self {
        Deck {
            deck_list: Vec::with_capacity(DECK_SIZE),
        }
    }

    pub fn build(&mut self) {
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

    pub fn shuffle(&mut self) {
        let mut rng = ::rand::rng();
        self.deck_list.shuffle(&mut rng);
    }
}

impl TwistPlayer {
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

    pub fn fire_torpedo(&mut self, opponent: &mut Player, target_y: usize) -> bool {
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

    pub fn radar_scan(&mut self, opponent: &mut TwistPlayer, target_x: usize, target_y: usize) {
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

    pub fn reinforce(&mut self, target_x: usize, target_y: usize) -> bool {
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

    pub fn start_patrol(&mut self, x: usize, y: usize) -> bool {
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

    pub fn try_patrol_move(&mut self, dir_x: isize, dir_y: isize) -> bool {
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

    pub fn cancel_patrol(&mut self, return_to_hand: bool) {
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


    pub fn update_patrol(&mut self) {
        if self.patrol_mode && self.patrol_frames > 0 {
            self.patrol_frames -= 1;
            
            if self.patrol_frames == 0 {
                self.cancel_patrol(true); // Return to hand
                println!("Patrol move timed out - card returned to hand");
            }
        }
    }

    pub fn draw_card(&mut self) -> Option<ActionType> {
        self.deck.deck_list.pop()
    }

    pub fn draw_hand(&mut self) {
        while self.hand.len() < HAND_SIZE {
            if let Some(card) = self.draw_card() {
                self.hand.push(card);
            } else {
                break; // No more cards in deck
            }
        }
    }

    pub fn use_card(&mut self, action_type: ActionType) -> bool {
        if let Some(pos) = self.hand.iter().position(|&x| x == action_type) {
            self.hand.remove(pos);
            true
        } else {
            false
        }
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

