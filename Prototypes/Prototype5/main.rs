//! Handles game loop logic, ui drawing and input
//! Calls into `base.rs` for classic mode logic and `twist.rs` for twist mode logic

 /* ------ Import Used Libraries ------ */
 mod base; // Imports the base game module
 mod twist; // Imports the twist game module
 
 // Uses base game module
 use base::*; 
 // Graphics library
 use macroquad::{audio, prelude::*}; 
 // A module I recompiled and made small fixes to, but did not write. Used for grid graphics and logic.
 // Origional code avaliable here: https://github.com/TheDinner22/macroquad_grid
 extern crate macroquad_grid_dex;
 
 // Conditionaly uses the twist module
 #[cfg(feature = "twist")] 
 use twist::*;
 
 /*------ Constants ------ */
 // Sound Effects Constants (Bytes needed for succesful compile)
 #[cfg(feature = "twist")] 
 const REINFORCE_SOUND: &[u8] = include_bytes!("Sound/Reinforce(new version).wav");
 
 #[cfg(feature = "twist")] 
 const SONAR_SOUND: &[u8] = include_bytes!("Sound/Sonar(new version).wav");
 
 const MISSLE_SOUND: &[u8] = include_bytes!("Sound/Sound Effect - Missile Launch.wav");
 
 const SPLASH_SOUND: &[u8] = include_bytes!("Sound/Splash(new version).wav");
 
 #[cfg(feature = "twist")] 
 const TORPEDO_SOUND: &[u8] = include_bytes!("Sound/Torpedo(new version).wav");
 
 /*------ Main Loop ------ */
 // Change the title of the game window based of of the compile specifications
 #[cfg_attr(feature = "twist", macroquad::main("Battleship Twisted"))]
 #[cfg_attr(not(feature = "twist"), macroquad::main("Battleship Classic"))]
 async fn main() {
     request_new_screen_size(1280.,720.); // Change screen resolution
 
     /* --- Loads Sound Assets --- */
     // Load sound effects from data
     #[cfg(feature = "twist")] 
     let reinforce_sound: audio::Sound = audio::load_sound_from_bytes(REINFORCE_SOUND).await.unwrap();
     #[cfg(feature = "twist")] 
     let torpedo_sound: audio::Sound = audio::load_sound_from_bytes(TORPEDO_SOUND).await.unwrap();
     #[cfg(feature = "twist")] 
     let sonar_sound: audio::Sound = audio::load_sound_from_bytes(SONAR_SOUND).await.unwrap();
     let splash_sound: audio::Sound = audio::load_sound_from_bytes(SPLASH_SOUND).await.unwrap();
     let missile_sound: audio::Sound = audio::load_sound_from_bytes(MISSLE_SOUND).await.unwrap();
 
     /* --- Initialise Players --- */
     #[cfg(feature = "twist")] 
     let mut player1: TwistPlayer = {
         let base = BasePlayer::new();
         TwistPlayer::new(base)
     };
 
     #[cfg(feature = "twist")]
     let mut player2: TwistPlayer = {
         let base = BasePlayer::new();
         TwistPlayer::new(base)
     };
 
     #[cfg(not(feature = "twist"))] 
     let mut player1: BasePlayer = BasePlayer::new();
     
     #[cfg(not(feature = "twist"))]
     let mut player2: BasePlayer = BasePlayer::new();
 
      /*--- Initialise Variables --- */
      #[cfg(feature = "twist")] // Twist specific variable
      const NO_ACTION_ERROR: &str = "You can't use that action, it isn't in your hand.";
     
     let mut player_turn: GameState = GameState::Player1; // Handles whose turn it is and whose it was last
     let mut game_state: GameState = GameState::Player1; // Handles turns but also handles inbetween turns.
 
     let mut show_help = false;
 
     let mut player_acted: bool = false; // If the player acted that turn
 
     let mut turncounter: f64 = 1.0; // Number of turns elapsed 
 
     let mut player_won: GameState = GameState::Else; // Which player has won
 
     /* ------ Game Loop ------ */
     loop {
         clear_background(BLACK); // Clears screen to black
 
         // Set current player and opponent
         let (mut current_player, mut current_opponent) = if game_state == GameState::Player1 {
             (&mut player1, &mut player2)
         } else {
             (&mut player2, &mut player1)
         };
 
         /* --- UI Drawing --- */
         if game_state == GameState::Else {
             // Inbetween UI
             draw_text("Press Space to change player",(screen_width()/2.0)-350.0,(screen_height()/2.0)-30.0,60.0,WHITE);
         }
         else {
             // Player UI
             #[cfg(not(feature = "twist"))] 
             {
                 current_player.boardgrid.draw();
                 current_player.guessgrid.draw();
             }
 
             draw_text("Player 1's turn", (screen_width()/2.0)-100.0, 45.0, 30.0, WHITE);
         
             #[cfg(feature = "twist")] 
             {
                 current_player.base.boardgrid.draw();
                 current_player.base.guessgrid.draw();
 
                 current_player.update_patrol();
                 draw_hand_to_screen(&current_player.hand, (screen_width()/2.0)-120.0, 500.0);
             }
         }
 
         if show_help {
             draw_help_menu();
         }
 
         /* --- Input for the Twisted Version --- */
         #[cfg(feature = "twist")] 
         {
             // Stops if the player has acted
             if !player_acted && game_state != GameState::Else{ 
 
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
 
         if is_key_pressed(KeyCode::H) {
             show_help = !show_help;
         }
 
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
 
         // Allows early exit of game
         if is_key_pressed(KeyCode::Escape) {
             break;
         }
 
         // Displays turn UI
         let temp_turncounter = (turncounter/2.0).floor();
         draw_text(format!("Turn: {}", temp_turncounter).as_str(),75.0,45.0,30.0,WHITE);
 
         // Updates view
         next_frame().await;
     }
 
     // Halve turns to get the true number
     turncounter = turncounter/2.;
     turncounter = turncounter.floor(); // Round down
 
     // loop to display winner message and wait for exit
     loop{
 
         clear_background(BLACK);
 
         // Checks if Player 1 has won and displays win message
         if player_won == GameState::Player1 {
             draw_text("Player 1 Won!!", (screen_width()/2.0)-200.0, screen_height()/2.0, 60.0, WHITE);
         }
 
         // Checks if Player 2 has won and display win message
         else if player_won == GameState::Player2 {
             draw_text("Player 2 Won!!", (screen_width()/2.0)-200.0, screen_height()/2.0, 60.0, WHITE);
         }
 
         // Wait for space key or escape to be pressed the exit the game
         if is_key_pressed(KeyCode::Space) {
             break;
         }
 
         if is_key_pressed(KeyCode::Escape) {
             break;
         }
 
         // Displays how many turns it took to win
         draw_text(format!("After {} turns",turncounter).as_str(),(screen_width()/2.0)-180.0,(screen_height()/2.0)+50.0,30.0,WHITE);
         
         next_frame().await;
     }
 }