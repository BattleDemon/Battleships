mod base;
mod twist;

use base::*;
use ::rand::prelude::{SliceRandom, IndexedRandom};
use macroquad::{audio, prelude::*};
extern crate macroquad_grid_dex;
use macroquad_grid_dex::Grid;

#[cfg(feature = "twist")]
use twist::*;

// Sound Effects Constants (Bytes)
const REINFORCE_SOUND: &[u8] = include_bytes!("Sound/Reinforce(new version).wav");
const SONAR_SOUND: &[u8] = include_bytes!("Sound/Sonar(new version).wav");
const MISSLE_SOUND: &[u8] = include_bytes!("Sound/Sound Effect - Missile Launch.wav");
const SPLASH_SOUND: &[u8] = include_bytes!("Sound/Splash(new version).wav");
const TORPEDO_SOUND: &[u8] = include_bytes!("Sound/Torpedo(new version).wav");


#[cfg_attr(feature = "twist", macroquad::main("Battleship Twisted"))]
#[cfg_attr(not(feature = "twist"), macroquad::main("Battleship Classic"))]
async fn main() {
    request_new_screen_size(1280.,720.);

    // Load sound effects from data
    let reinforce_sound: audio::Sound = audio::load_sound_from_bytes(REINFORCE_SOUND).await.unwrap();
    let torpedo_sound: audio::Sound = audio::load_sound_from_bytes(TORPEDO_SOUND).await.unwrap();
    let sonar_sound: audio::Sound = audio::load_sound_from_bytes(SONAR_SOUND).await.unwrap();
    let splash_sound: audio::Sound = audio::load_sound_from_bytes(SPLASH_SOUND).await.unwrap();
    let missle_sound: audio::Sound = audio::load_sound_from_bytes(MISSLE_SOUND).await.unwrap();

    let mut player1: BasePlayer = BasePlayer::new();
    let mut player2: BasePlayer = BasePlayer::new();

    #[cfg(feature = "twist")]
    {
        let mut player1: TwistPlayer = TwistPlayer::new(player1);
        let mut player2: TwistPlayer = TwistPlayer::new(player2);
    
        let no_action_error = "You can't use that action, it isn't in your hand.";
    }

    let mut player_turn: GameState = GameState::Player1;
    let mut game_state: GameState = GameState::Player1;

    let mut player_acted: bool = false;

    let mut turncounter: f64 = 1.0;

    let mut player_won: GameState = GameState::Else;

    loop {
        clear_background(BLACK);

        if game_state == GameState::Player1 {
            player1.boardgrid.draw();
            player1.guessgrid.draw();

            draw_text("Player 1's turn", (screen_width()/2.0)-100.0, 45.0, 30.0, WHITE);
        
            #[cfg(feature = "twist")] 
            {
                player1.update_patrol();
                draw_hand_to_screen(&player1.hand, (screen_width()/2.0)-120.0, 500.0);
            }
        }
        else if game_state == GameState::Player2 {
            player2.boardgrid.draw();
            player2.guessgrid.draw();

            draw_text("Player 2's turn", (screen_width()/2.0)-100.0, 45.0, 30.0, WHITE);

            #[cfg(feature = "twist")] 
            {
                player2.update_patrol();
                draw_hand_to_screen(&player2.hand, (screen_width()/2.0)-120.0, 500.0);
            }
        }
        else {
            draw_text("Press Space to change player",(screen_width()/2.0)-350.0,(screen_height()/2.0)-30.0,60.0,WHITE);
        }

        #[cfg(feature = "twist")] 
        {
            if !player_acted {

                if is_mouse_button_pressed(MouseButton::Left) {
                    if game_state == GameState::Player1 {
                        if player1.use_card(ActionType::Missle) {
                            if let Some((x,y)) = player1.get_clicked_cell() {
                                let hit = player1.fire_missile(&mut player2, x, y);

                                player_acted = true;

                                println!("Missle {}", if hit { "hit!"} else { "missed."});
                                if hit {audio::play_sound_once(&missle_sound)} else {audio::play_sound_once(&splash_sound)};  

                            } else {
                                player1.hand.push(ActionType::Missile);
                            }
                        } else {
                        println!(no_action_error);
                        }
                    } else if game_state == GameState::Player2 {
                        if player2.use_card(ActionType::Missile) {
                            if let Some((x,y)) = player2.get_clicked_cell() {
                                let hit = player2.fire_missile(&mut player2, x, y);

                                player_acted = true;

                                println!("Missle {}", if hit { "hit!"} else { "missed."});
                                if hit {audio::play_sound_once(&missle_sound)} else {audio::play_sound_once(&splash_sound)};  

                            } else {
                                player2.hand.push(ActionType::Missile);
                            }
                        } else {
                            println!(no_action_error);
                        }
                    }
                }

                if is_key_pressed(KeyCode::T) {
                    if game_state == GameState::Player1 {
                        if player1.use_card(ActionType::Torpedo) {
                            audio::play_sound_once(&torpedo_sound);

                            if let Some(target_x) = player1.get_torpedo_target_column(){
                                let hit = player1.fire_torpedo(&mut player2, target_x);

                                player_acted = true;

                                println!("Torpedo {}", if hit { "hit!" } else { "missed." });
                            } else {
                                player1.hand.push(ActionType::Torpedo);
                            }
                        } else {
                            println!(no_action_error);
                        }
                    } else if game_state == GameState::Player2 {
                        if player2.use_card(ActionType::Torpedo) {
                            audio::play_sound_once(&torpedo_sound);

                            if let Some(target_x) = player2.get_torpedo_target_column() {
                                let hit = player2.fire_torpedo(target_x);

                                player_acted = true;

                                println!("Torpedo {}", if hit { "hit!" } else { "missed." });
                            } else {
                                player2.hand.push(Actiong::Torpedo);
                            }
                        } else {
                            println!(no_action_error);
                        }
                    }
                }

                if is_key_pressed(KeyCode::R) {
                    if game_state == GameState::Player1 {
                        if player1.use_card(ActionType::Reinforce) {
                            if let Some((x,y)) = player1.get_clicked_cell_on_own_board {
                                let success = player1.reinforce(x,y);

                                player_acted = true;

                                println!("Reinforcement {}", if success { "successful!" } else { "failed." });
                                if success { audio::play_sound_once(&reinforce_sound)}                             
                            } else {
                                player1.hand.push(ActionType::Reinforce);
                            }
                        } else {
                            println!(no_action_error);
                        }
                    } else if game_state = GameState::Player2 {
                        if let Some((x,y)) = player2.get_clicked_cell_on_own_board {
                            let success = player2.reinforce(x,y);

                            player_acted = true;

                            println!("Reinforcement {}", if success { "successful!" } else { "failed." });
                            if success { audio::play_sound_once(&reinforce_sound)}                             
                        } else {
                            player2.hand.push(ActionType::Reinforce);
                        }
                    } else {
                        println!(no_action_error);
                    }
                }

                if is_key_pressed(KeyCode::S) {
                    if game_state == GameState::Player1 {
                        if player1.usecard(ActionType::RadarScan) {
                            if let Some((x,y)) = player1.get_clicked_cell() {
                                player1.radar_scan(&mut player2,x,y);

                                player_acted = true;

                                audio::play_sound_once(&sonar_sound);
                            } else {
                                player1.hand.push(ActionType::RadarScan);
                            }
                        } else {
                            println!(no_action_error);
                        }
                    } else if game_state == GameState::Player2 {
                        if player2.usecard(ActionType::RadarScan) {
                            if let Some((x,y)) = player2.get_clicked_cell() {
                                player2.radar_scan(&mut player2,x,y);

                                player_acted = true;

                                audio::play_sound_once(&sonar_sound);
                            } else {
                                player2.hand.push(ActionType::RadarScan);
                            }
                        } else {
                            println!(no_action_error);
                        }
                    }
                }

                if is_key_pressed(KeyCode::P) {

                }

            } else {
                println!();
            }
        } 

        #[cfg(not(feature = "twist"))]{
            if !player_acted {
                if is_mouse_button_pressed(MouseButton::Left) {

                }
            }
        }

        if is_key_pressed(KeyCode::Space) {
            if player_acted {
                println!("Player changed");
                println!(" ");

                #[cfg(feature = "twist")]{
                    if game_state == GameState::Player1 {
                        let newcard = player1.draw_card().unwrap();
                        player1.hand.push(newcard);
                    } else if game_state == GameState::Player2{
                        let newcard = player2.draw_card().unwrap();
                        player2.hand.push(newcard);
                    }
                }

                game_state = GameState::Else;
                player_acted = false;
                turncounter += 1.0;
            } else {
                if game_state == GameState::Else {
                    if player_turn = GameState::Player1  {
                        game_state = GameState::Player2;
                        player1_turn = GameState::Player2;
                    } else if player_turn = GameState::Player2  {
                        game_state = GameState::Player1;
                        player1_turn = GameState::Player1;
                    }
                }
            }
        }

        if player1.ship_count == 0 {
            player_won = GameState::Player2;
            break;
        }else if player2.ship_count == 0 {
            player_won = GameState::Player1;
            break;
        }

        let temp_turncounter = (turncounter/2.0).floor();
        draw_text(format!("Turn: {}", temp_turncounter).as_str(),75.0,45.0,30.0,WHITE);

        next_frame().await;
    }

    turncounter = turncounter/2.;
    turncounter = turncounter.floor();

    loop{
        if player_won == Player1 {
            clear_background(BLACK);
            draw_text("Player 1 Won!!", (screen_width()/2.0)-200.0, screen_height()/2.0, 60.0, WHITE);
        }

        else if player_won == Player2 {
            clear_background(BLACK);
            draw_text("Player 2 Won!!", (screen_width()/2.0)-200.0, screen_height()/2.0, 60.0, WHITE);
        }

        if is_key_pressed(KeyCode::Space) {
            break;
        }

        draw_text(format!("After {} turns",turncounter).as_str(),(screen_width()/2.0)-180.0,(screen_height()/2.0)+50.0,30.0,WHITE);
        
        next_frame().await;
    }
}