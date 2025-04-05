# Battleship 
## Table of Contents
- [How to Play](#how-to-play)
    - [Controls](#controls)
    - [How to Run](#how-to-run)
- [Preplanning](#preplanning)
    - [What is Battleships](#what-is-battleships)
    - [The History of Battleships](#the-history-of-battleships)
    - [My twist](#my-twist)
    - [Flowchart](#flowchart)
    - [Timeline](#timeline)
    - [If i get more time](#if-i-get-more-time)
- [Prototyping](#prototyping)
    - [Prototype 1: Basic Game Loop](#prototype-1-basic-game-loop)
        - [Code at March 24th](#code-at-march-24th)
        - [Video Of Functionality (link to youtube)](#video-of-functionality-link-to-youtube)
        - [Issues and Solutions](#issues-and-solutions)
    - [Prototype 2: Added Torpedo and Radar Scan Actions and random ship placement](#prototype-2-added-torpedo-and-radar-scan-actions-and-random-ship-placement)
        - [Code at March 27th](#code-at-march-27th)
        - [Video Of Functionality](#video-of-functionality)
        - [Issues and Solutions](#issues-and-solutions-1)
    - [Prototype 3: Reinforce and Patrol](#prototype-3-reinforce-and-patrol)
        - [Code at March 30th](#code-at-march-30th)
        - [Video of Functionality](#video-of-functionality-1)
        - [Issues and Solutions](#issues-and-solutions-2)
    - [Prototype 4: Hand display and fully developed action systems](#prototype-4-hand-display-and-fully-developed-action-systems)
        - [Code at April 2nd](#code-at-april-2nd)
        - [Video of Functionality](#video-of-functionality-2)
        - [Issue and Solutions](#issues-and-solution)
    - [Final version: Seperate files and toggleable twist system](#final-version-seperate-files-and-toggleable-twist-system)
        - [Code at Submission](#code-at-submission)
        - [Video of Functionality](#video-of-functionality-3)
        - [Issue and Solutions](#issue-and-solutions)
- [Reflection](#reflection)
    - [How is the overall desing](#how-is-the-overall-desing)
    - [What Changes could i make](#what-changes-could-i-make)
    - [What issues did I encounter](#what-issues-did-i-encounter)
    - [Player Feedback](#player-feedback)
    
## How to Play 
### Controls
- **Classic Mode**: Click to fire missiles.  
- **Twisted Mode** (`cargo run --features twist`):  
  - `T`: Fire Torpedo (vertical strike)  
  - `R`: Reinforce a ship cell  
  - `S`: Radar Scan (reveal 5 cells)  
  - `P`: Start Patrol (move ships with arrow keys)  
- `Space`: End turn  
- `H`: Toggle help screen  

### How to Run
1. Open the terminal in this folder.
2. To run the base game use 'cargo run' 
3. To run the twisted version of the game use 'cargo run --features'
4. Alternatively you could navigate to the Battleships/Release folder 
5. Launch the .exe for the version you want to play
    - battleshipsV1.0.exe for the base game 
    - battleshipsTwistedV1.0.exe for the twisted game

## Preplanning 
### What is Battleships

 Battleships is a two-player game played on four 10x10 grids—two for each player. One grid is used to place their own ships, while the other is for tracking their opponent's guesses. The game begins with each player placing their battleships, followed by the placement of additional ships until both players have five in total. Players take turns guessing coordinates on their opponent's grid and announcing their guess. The opponent responds with either "hit" or "miss," which the current player records on their vertical grid. The turn then passes to the other player. This continues until all ships are sunk, which happens when every coordinate a ship occupies has been hit.

### The History of Battleships

  Originating in the early 1900s, Battleships began as a game played with pen and paper. The first published version of the game appeared in 1931 under the name "Salvo" by Starex Novelty Co. It was still played with paper and pen, but now with specifically printed pads for the game. Unlike today, each player had as many shots as they had ships, with some variations allowing certain ships two shots. Once a player had announced all their shots, the opponent would respond by saying how many shots hit and what was hit, but without revealing which specific shot resulted in which outcome. In the late 1930s and 1940s, the game was re-released under different names, such as Combat: The Battleship Game and Broadside: A Game of Naval Strategy. In 1967, Milton Bradley released Battleship, which looked much like the version we know today. It was made of plastic and included small pieces to represent ships, hits, and misses, along with two boards—one horizontal for housing ships and another for tracking guesses. In the 1980s and 1990s, new versions and spin-offs of the game were released, and it was adapted into video games. In 2012, Battleship was even made into a feature film.
  
### My Twist

 So, what's the twist? Instead of simply firing at the enemy, you'll now draw from a deck and choose an action from your hand. These new actions include: shooting one missile in a straight line from the X-axis you choose until it either leaves the board or hits something (Torpedo), moving a ship one space in any direction (Patrol), adding an extra hit point to a ship (Reinforce), and revealing part of the enemy's board (Radar Scan).

### Flowchart

![screenshot](images/Screenshot_flowchart.png)

### Timeline
Please see [timeline.md](timeline.md)

### If I get more time

  If I finish my game before it's due, I'll try to add a simple AI that goes beyond just a random number generator. Another improvement I could make is enhancing the graphics or making the game more user-friendly. 

## Prototyping 
### Prototype 1: Basic game loop
#### Code at March 24th 
To view all code at this point please see [Prototypes/Prototype1.rs](Prototypes/Prototype1.rs)

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

You can play this prototype by going to battleships/prototypes_exes then run `battleshipsV0.1.exe`

#### Issues and Solutions 
During this time, I ran into an issue with the library I was using `Macroquad Grid`, which was meant to handle rendering and managing my on-screen game board. The problem was that it had been compiled with an older version of Rust, as well as an outdated version of its base library `Macroquad`. Fortunately, I found the library's repository and was able to fork it. After some tweaking, all it needed was a recompile with the updated Rust version and a few minor syntax fixes thankfully, they were straightforward to resolve.

Another challenge I faced was with mouse input on the guessing board, the clicks weren’t aligning properly with the visual grid. If you clicked on the right side of a cell, it would register a hit on the cell next to it instead. The fix itself was simple, but since I wasn’t sure exactly how many pixels it was off by, it took a bit of trial and error to get the alignment as close to perfect as possible.

### Prototype 2: Added Torpedo and Radar scan actions and random ship placement
#### Code at March 27th 
To view all code at this point please see [Prototypes/Prototype2.rs](Prototypes/Prototype2.rs)

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

You can play this prototype by going to battleships/prototypes_exes then run `battleshipsV0.2.exe`

#### Issues and Solutions 
The first issue I encountered was that using the radar scan on cells at the edges of the grid caused the game to crash. This happened because my implementation led to an integer underflow when converting back to usize. On the opposite edge, the game crashed due to attempting to modify a grid cell that was out of bounds, something the `Macroquad grid` documentation warned could be an issue.

Another problem was with the torpedo system, which continued moving upwards after hitting an already hit cell instead of stopping as intended. This was an easy fix, I simply added an if statement to check if the torpedo was passing through a previously hit cell rather than just stopping at the first occupied one.

I also encountered an issue with random ship placement, where ships could be placed on top of each other. This was because the random placement algorithm selected the first random position rather than iterating through multiple attempts to find one that met the placement requirements. Expanding the conditions for valid placement resolved this issue.

### Prototype 3: Reinforce and Patrol
#### Code at March 30th 
To view all code at this point please see [Prototypes/Prototype3.rs](Prototypes/Prototype3.rs)

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

You can play this prototype by going to battleships/prototypes_exes then run `battleshipsV0.3.exe`
#### Issues and Solutions 

When implementing the Reinforce cell state, it would sometimes incorrectly downgrade a reinforced cell to hit instead of to occupied while the visual feedback didn't aling with this. This was because the change_cell function didn't properly handle the Reinforced case and the color would map incorrectly. The fix was to just add a explicit check for `Cells::Reinforced` and updating the `change_cell` to properly map `DARKGREEN`.

During patrol mode, ships would sometimes move to invalid positions and overlap with ships. This was because the `Try_patrol_move` function didn't properly validate the new positions before updating the ships location. Adding strict bounds checking and collision detection before moving fixed this issue.

### Prototype 4: Hand display and fully developed action systems
#### Code at April 2nd 
To view all code at this point please see [Prototypes/Prototype4.rs](Prototypes/Prototype4.rs)

```rs
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
```

```rs
// In Player impl
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

// Outside Player
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
```

#### Video of Functionality  

[![IT Prototype 2 April](https://img.youtube.com/vi/t9gRkWBTLv4/0.jpg)](https://www.youtube.com/watch?v=t9gRkWBTLv4)

You can play this prototype by going to battleships/prototypes_exes then run `battleshipsV0.4.exe`

#### Issues and Solution 

The game would crash when an attemping to draw a card from an empty deck. This happens because the `draw_card` method didn't handle the case where `deck_list` was empty causing a panic when calling `pop()` on an emprty vector. The fix was to just modify the `draw_card` to return an `Option<ActionType>` and added checks in `draw_hand` to stop drawing when the deck is empty. 

When using the patrol action but not completing the move before the timer runs out it would return the patrol card to your hand and give you en extra card when the timer finished. This was a simple fix, you just modify the `cancel_patrol` function to return a value instead of before when it didn't then give back the card based of on that.

### Final version: Seperate files and toggleable twist system
#### Code at Submission 

To view final code please see the below files in `battleships/src`
- `base.rs`: Core logic for boards, ships, and classic gameplay.  
- `twist.rs`: Action card system, patrol mechanics, and extended player logic.  
- `main.rs`: Game loop, UI rendering, and feature toggling with `cfg` macros. 

How functionality is Split
```rs
#[cfg(feature = "twist")] // makes the code under it only run when you compile with this feature

#[cfg(not(feature = "twist"))] // Inversly this only runs the code when it is compiled without the twist feature

// Eg
#[cfg(feature = "twist")] 
use twist::*;
// Only imports twist module when the feature twist is enabled

#[cfg_attr(not(feature = "twist"), macroquad::main("Battleship Classic"))]
// Changes the title of the program when the twist feature is not enabled

// You can also do
#[cfg(feature = "twist")] {
    // Every thing in here will only run if the feature is enabled
}
```

Twist Functionality and loop
```rs 
        /* --- Input for the Twisted Version --- */
        #[cfg(feature = "twist")] 
        {
            // Stops if the player has acted
            if !player_acted { 

                /* --- Missile Action --- */
                if is_mouse_button_pressed(MouseButton::Left) {
                    // If the current player has the card 
                    if current_player.use_card(ActionType::Missile) {
                        // Gets the grid pos of where the mouse was when clicked
                        if let Some((x,y)) = current_player.base.get_clicked_cell() {
                            // If you hit or not
                            let hit = current_player.base.fire_missile(&mut current_opponent.base, x, y);

                            player_acted = true;

                            // player feedback
                            println!("Missile {}", if hit { "hit!"} else { "missed."});
                            if hit {audio::play_sound_once(&missile_sound)} else {audio::play_sound_once(&splash_sound)};  
                        } else {
                            // If didn't click on a grid return the missile to hand
                            current_player.hand.push(ActionType::Missile);
                        }
                    } else {
                        // If the player didn't have the action
                        println!("{}",NO_ACTION_ERROR);
                    }
                }

                /* --- Torpedo Action --- */
                if is_key_pressed(KeyCode::T) {
                    if current_player.use_card(ActionType::Torpedo) {
                        // Audio feedback
                        audio::play_sound_once(&torpedo_sound);

                        // Gets the grid pos of where the mouse was when 't' was pressed
                        if let Some(target_x) = current_player.get_torpedo_target_column(){
                            // If you hit or not
                            let hit = current_player.fire_torpedo(&mut current_opponent, target_x);

                            player_acted = true;

                            // Text feedback
                            println!("Torpedo {}", if hit { "hit!" } else { "missed." });
                        } else {
                            // If didn't click on a grid return the torpedo to hand
                            current_player.hand.push(ActionType::Torpedo);
                        }
                    } else {
                        // If the player didn't have the action
                        println!("{}",NO_ACTION_ERROR);
                    }
                }

                /*--- Reinforce Action --- */
                if is_key_pressed(KeyCode::R) {
                    if current_player.use_card(ActionType::Reinforce) {
                        // Gets the grid pos of where the mouse was when 'r' was pressed
                        if let Some((x,y)) = current_player.get_clicked_cell_on_own_board() {
                            // If the cell was reinforced
                            let success = current_player.reinforce(x,y);

                            player_acted = true;

                            // Player feedback
                            println!("Reinforcement {}", if success { "successful!" } else { "failed." });
                            if success { audio::play_sound_once(&reinforce_sound)}                             
                        } else {
                            // If didn't click on an occupied cell return the reinforce card to hand
                            current_player.hand.push(ActionType::Reinforce);
                        }
                    } else {
                        // If the player didn't have the action
                        println!("{}",NO_ACTION_ERROR);
                    }
                }

                /* --- Radar Scan Action ---*/
                if is_key_pressed(KeyCode::S) {
                    if current_player.use_card(ActionType::RadarScan) {
                        // Gets the grid pos of where the mouse was when 's' was pressed
                        if let Some((x,y)) = current_player.base.get_clicked_cell() {
                            current_player.radar_scan(&mut current_opponent,x,y);

                            player_acted = true;

                            // Audio feedback
                            audio::play_sound_once(&sonar_sound);
                        } else {
                            // If didn't click on board return radar scan to hand
                            current_player.hand.push(ActionType::RadarScan);
                        }
                    } else {
                        // If the player didn't have the action
                        println!("{}",NO_ACTION_ERROR);
                    }
                }

                /*--- Patrol Action --- */
                if is_key_pressed(KeyCode::P) {
                    if !current_player.patrol_mode {
                        if current_player.use_card(ActionType::Patrol) {
                            // Gets the grid pos of where the mouse was when 'p' was pressed
                            if let Some((x,y)) = current_player.get_clicked_cell_on_own_board(){
                                let started = current_player.start_patrol(x,y);

                                if !started {println!("Couldn't start patrol")}
                            } else {
                                // If didn't click on board return patrol to hand
                                current_player.hand.push(ActionType::Patrol);
                                println!("No ship selected");
                            }
                        } else {
                            // If the player didn't have the action
                            println!("{}",NO_ACTION_ERROR);
                        }
                    } 
                }
            } 

            /* --- Patrol Moving --- */
            if !player_acted {
                if current_player.patrol_mode {
                    let dir = if is_key_pressed(KeyCode::Up) {
                        Some((-1, 0))
                    } else if is_key_pressed(KeyCode::Down) {
                        Some((1,0))
                    } else if is_key_pressed(KeyCode::Left) {
                        Some((0, -1))
                    } else if is_key_pressed(KeyCode::Right) {
                        Some((0, 1))
                    } else {
                        None
                    };

                    if let Some((dir_x, dir_y)) = dir {
                        let success = current_player.try_patrol_move(dir_x, dir_y);
                        println!("Patrol move {}", if success { "successful!" } else { "failed."});
                        player_acted = success;
                    }
                }
            }
        } 
```

Base Functionality and loop
```rs
        /*--- Classic Mode input --- */
        // Classic base mode input handleing
        #[cfg(not(feature = "twist"))]{
            if !player_acted && game_state != GameState::Else{
                // Gets the grid pos of where the mouse was when it was clicked
                if is_mouse_button_pressed(MouseButton::Left) {
                    if let Some((x,y)) = current_player.get_clicked_cell() {
                        let hit = current_player.fire_missile(&mut current_opponent, x, y);

                        player_acted = true;

                        println!("Missile {}", if hit { "hit!" } else { "missed." });
                        if hit { audio::play_sound_once(&missile_sound)} else { audio::play_sound_once(&splash_sound)};
                    }
                }
            }
        }
```

Turn Change
```rs
        /*--- Change Turn --- */
        if is_key_pressed(KeyCode::Space) {
            // Switch to inbetween screen if the player has acted
            if player_acted {
                println!("Player changed"); 
                println!(" ");
                
                // Draws cards 
                #[cfg(feature = "twist")]{
                    if current_player.deck.deck_list.is_empty() {
                        current_player.draw_hand();
                    }
                    let newcard = current_player.draw_card().unwrap();
                    current_player.hand.push(newcard);
                }
        
                // Reset player_acted here
                player_acted = false;
                game_state = GameState::Else;
                turncounter += 1.0;
            } else {
                // Switch to next turn
                if game_state == GameState::Else {
                    // Switch turns and reset state
                    if player_turn == GameState::Player1 {
                        game_state = GameState::Player2;
                        player_turn = GameState::Player2;
                    } else {
                        game_state = GameState::Player1;
                        player_turn = GameState::Player1;
                    }
                    // Ensure player_acted is reset for the new turn
                    player_acted = false;
                }
            }
        }
```

Win Check
```rs
        /*--- Win Check --- */
        #[cfg(feature = "twist")]{
            // Win check for twist (just added a .base)
            if player1.base.ship_count == 0 {
                player_won = GameState::Player2;
                break;
            }else if player2.base.ship_count == 0 {
                player_won = GameState::Player1;
                break;
            }
        }

        #[cfg(not(feature = "twist"))]{
            // Win check for base
            if player1.ship_count == 0 {
                player_won = GameState::Player2;
                break;
            }else if player2.ship_count == 0 {
                player_won = GameState::Player1;
                break;
            }
        }
```

#### Video of Functionality 

[![IT Prototype final version](https://img.youtube.com/vi/DLUWaWqjja4/0.jpg)](https://www.youtube.com/watch?v=DLUWaWqjja4)

To play follow the instructions in [How to run](#how-to-run)

#### Issue and Solutions 

Between `Prototype4` and now I transfered all the functions enums and structs over to their respective files, this process was involved a lot of trial and error, and took about a day of debugging to make a compileable programm. Some of these issue included making sure everything that needs to be public was public, in the twist section every refrence to base player variabled and methods was `twistplayer.base.var/func` instead of just `twistplayer.var/func`. 

## Reflection
### How is the overall desing
Modular desing


Visual desing


Mechanical desing

### What Changes could I make
Card Balance


Ai Opponent


More Feedback


### What issues did I encounter
Dependency Errors


Grid Coordinates Mismatched


Quarrels with a new language


### How were these issues solved
Debugging 


Testing 


Updating 


### What would I do if I were to do this again

### What have I learnt
Rust

### Player Feedback

