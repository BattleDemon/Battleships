# Battleship 
## Preplanning
### What is Battleships

 Battleships is a two-player game played on four 10x10 grids—two for each player. One grid is used to place their own ships, while the other is for tracking their opponent's guesses. The game begins with each player placing their battleships, followed by the placement of additional ships until both players have five in total. Players take turns guessing coordinates on their opponent's grid and announcing their guess. The opponent responds with either "hit" or "miss," which the current player records on their vertical grid. The turn then passes to the other player. This continues until all ships are sunk, which happens when every coordinate a ship occupies has been hit.

### The History of Battleships

  Originating in the early 1900s, Battleships began as a game played with pen and paper. The first published version of the game appeared in 1931 under the name "Salvo" by Starex Novelty Co. It was still played with paper and pen, but now with specifically printed pads for the game. Unlike today, each player had as many shots as they had ships, with some variations allowing certain ships two shots. Once a player had announced all their shots, the opponent would respond by saying how many shots hit and what was hit, but without revealing which specific shot resulted in which outcome. In the late 1930s and 1940s, the game was re-released under different names, such as Combat: The Battleship Game and Broadside: A Game of Naval Strategy. In 1967, Milton Bradley released Battleship, which looked much like the version we know today. It was made of plastic and included small pieces to represent ships, hits, and misses, along with two boards—one horizontal for housing ships and another for tracking guesses. In the 1980s and 1990s, new versions and spin-offs of the game were released, and it was adapted into video games. In 2012, Battleship was even made into a feature film.
  
### My Twist

 So, what's the twist? Instead of simply firing at the enemy, you'll now draw from a deck and choose an action from your hand. These new actions include: shooting one missile in a straight line from the X-axis you choose until it either leaves the board or hits something (Torpedo), moving a ship one space in any direction (Patrol), adding an extra hit point to a ship (Reinforce), and revealing part of the enemy's board (Radar Scan).

### Flowchart

![screenshot](images/Screenshot_flowchart.png)

### Pseudo Code
#### Core Game Loop Systems
Game State Manegment
```
```

Turn Switching 
```
```

Win Checking
```
```

Main Loop
```
```

#### Player Action Systems
Missle System
```
```

Torpedo System
```
```

Radar System
```
```

Reinforce System
```
```

Patrol System
```
Function start_patrol(x, y):
    Find ship at position (x, y)
    If ship found:
        Check if ship is hit
        If hit, return False
        Set patrol mode and ship
        Highlight ship's positions
        Return True
    Else:
        Return False

Function try_patrol_move(dir_x, dir_y):
    If patrol is active:
        Check if new positions are valid
        If valid, update ship's positions
        Clear old positions and mark new ones
        Return True
    Else:
        Return False

Function cancel_patrol():
    If patrol is active:
        Remove highlights and reset cell colors
        Reset patrol state

Function update_patrol():
    If patrol is active and timer > 0:
        Decrease timer
        If timer reaches 0, cancel patrol

```

#### Struct Systems
Deck Shuffle
```
```

Hand System
```
```

Ship Placement
```
```

Ship Destruction
```
```

Cell Manegement
```
```

Board Rendering
```
```

Mouse Input
```
```

### Timeline 
Please see https://github.com/BattleDemon/Battleships/blob/main/timeline.md

### If I get more time

  If I finish my game before it's due, I'll try to add a simple AI that goes beyond just a random number generator. Another improvement I could make is enhancing the graphics or making the game more user-friendly. 

## Prototyping 
### Prototype 1: Basic game loop 
#### Code at March 24th 
To view all code at this point please see https://github.com/BattleDemon/Battleships/blob/main/Prototypes/Prototype1.rs

Main Loop
```rs
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
        
        if is_key_pressed(KeyCode::A) {

            let nums: Vec<usize> = (0..10).collect();
                let mut rng = ::rand::rng();
                let tempx = nums.choose(&mut rng);
                let tempy = nums.choose(&mut rng);
                let x: usize = *tempx.unwrap();
                let y: usize = *tempy.unwrap();

            if player1_turn == true {
                player1.fire_missile(&mut opponent,x,y);
                
            }
            else {
                opponent.fire_missile(&mut player1,x,y)
            }
            player1_turn = !player1_turn;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            if player1_turn {
                if let Some((x, y)) = player1.get_clicked_cell() {
                    player1.fire_missile(&mut opponent, x, y);
                    player1_turn = false;
                }
            } else {
                if let Some((x, y)) = opponent.get_clicked_cell() {
                    opponent.fire_missile(&mut player1, x, y);
                    player1_turn = true;
                }
            }
        }

        next_frame().await
    }
}
```

Change Cell function
```rs
fn change_cell(&mut self, x:usize,y:usize,ctype:Cells,grid:&mut Grid) {

        if self.cells[x][y] != Cells::Hit {
            match ctype {
                Cells::Empty => grid.color_cell(x,y ,BLACK),
                Cells::Occupied => grid.color_cell(x,y,GREEN),
                Cells::Hit => grid.color_cell(x, y, RED),
                Cells::Miss => {grid.set_cell_text(x,y, Some("0"));
                                grid.color_cell(x,y,GRAY); },
            }
            self.cells[x][y] = ctype;
        }
    }
```

Fire Missle and get clicked cell Functions
```rs
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

fn get_clicked_cell(&self) -> Option<(usize, usize)> {
        let (mouse_x, mouse_y) = mouse_position();
        
        let grid_x_offset = 710.0;
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
```
#### Video of Functionality (link to youtube)
[![IT Prototype 25 March](https://img.youtube.com/vi/NM8lwhZ-a-o/0.jpg)](https://www.youtube.com/watch?v=NM8lwhZ-a-o)
#### Issues and Solutions 

### Prototype 2: Added Torpedo and Radar scan actions and random ship placement
#### Code at March 27th 
To view all code at this point please see [https://github.com/BattleDemon/Battleships/blob/main/Prototypes/Prototype2.rs](https://github.com/BattleDemon/Battleships/blob/main/Prototypes/Prototype2.rs)

Main Loop
```rs
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
```

Torpedo Function
```rs
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
```

Radarscan Function
```rs
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
```

Random Ship Placement
```rs
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
```

#### Video of Functionality 
[![IT Prototype 27 March](https://img.youtube.com/vi/BdwVXEb1Fnw/0.jpg)](https://www.youtube.com/watch?v=BdwVXEb1Fnw)
#### Issues and Solutions 

### Prototype 3: Reinforce and Patrol
#### Code at March 30th 
To view all code at this point please see https://github.com/BattleDemon/Battleships/blob/main/Prototypes/Prototype3.rs

Getting Click input on your own board
```rs
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
```
Reinforce Function
```rs
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
```
Patrol Functions
```rs 
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
            self.cancel_patrol();
            true
        } else {
            false
        }
    }

    fn cancel_patrol(&mut self) {
        if let Some(ship_idx) = self.patrol_ship {
            // Remove highlight and reset to proper colors
            for &(x, y) in &self.ships[ship_idx].positions {
                match self.board.cells[x][y] {
                    Cells::Occupied => self.boardgrid.color_cell(x, y, GREEN),
                    Cells::Reinforced => self.boardgrid.color_cell(x, y, DARKGREEN),
                    Cells::Empty => self.boardgrid.color_cell(x, y, BLACK),  // Reset empty cells to black
                    _ => {}
                }
            }
        }
        self.patrol_mode = false;
        self.patrol_ship = None;
        self.patrol_frames = 0;
    }


    fn update_patrol(&mut self) {
        if self.patrol_mode && self.patrol_frames > 0 {
            self.patrol_frames -= 1;
            
            if self.patrol_frames == 0 {
                self.cancel_patrol();
            }
        }
    }
```
How does this work in the main loop
```rs 
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

        if is_key_pressed(KeyCode::P) && !player_acted {
            if player1_turn && !player1.patrol_mode {
                if let Some((x, y)) = player1.get_clicked_cell_on_own_board() {
                    let started = player1.start_patrol(x, y);
                    println!("{}", if started { "Select direction with arrow keys" } else { "No ship at that position" });
                }
            } else if !player1_turn && !opponent.patrol_mode {
                if let Some((x, y)) = opponent.get_clicked_cell_on_own_board() {
                    let started = opponent.start_patrol(x, y);
                    println!("{}", if started { "Select direction with arrow keys" } else { "No ship at that position" });
                }
            }
        }

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
```

#### Video of Functionality 
[![IT Prototype 30 March](https://img.youtube.com/vi/NQqllY27vTk/0.jpg)](https://www.youtube.com/watch?v=NQqllY27vTk)
#### Issues and Solutions 

### Prototype 4: Hand display and fully developed action systems
#### Code at April 2nd
To view all code at this point please see https://github.com/BattleDemon/Battleships/blob/main/Prototypes/Prototype4.rs

```rs
// Hand related Functions
```

```rs
// How do actions work with the hand system
```

#### Video of Functionality 

#### Issues and Solution

## Reflecting 
### How is the overall desing 

### What Changes could I make

### What issues did I encounter

### How were these issues solved

### What would I do if I were to do this again

### What have I learnt 

