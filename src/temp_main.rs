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

#[macroquad::main("Battleship")]
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

                }

                if is_key_pressed(KeyCode::T) {

                }

                if is_key_pressed(KeyCode::R) {

                }

                if is_key_pressed(KeyCode::S) {

                }

                if is_key_pressed(KeyCode::P) {

                }

            }
        } 

        #[cfg(not(feature = "twist"))]{
            if !player_acted {
                if is_mouse_button_pressed(MouseButton::Left) {

                }
            }
        }



        let temp_turncounter = (turncounter/2.0).floor();
        draw_text(format!("Turn: {}", temp_turncounter).as_str(),75.0,45.0,30.0,WHITE);

        next_frame().await;
    }

    turncounter = turncounter/2.;
    turncounter = turncounter.floor();

    loop{
        if player_won == Player1 {

        }

        else if player_won == Player2 {
            
        }
    }
}