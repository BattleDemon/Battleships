//! AI implementations for both classic and twist mdoes

use crate::base::{BasePlayer, GameState, GRID_SIZE, Cells};
use crate::twist::{TwistPlayer, ActionType};
use rand::prelude::*;

pub enum AIDifficulty {
    Easy,
    Medium,
    Hard,
}

pub struct BaseBattleshipAI {
    pub difficulty: AIDifficulty,
    pub memory: Vec<(usize, usize)>,
    pub player: BasePlayer,
}

pub struct TwistBattleshipAI {
    pub base_ai: BaseBattleshipAI,
    pub player: TwistPlayer,
}

impl TwistBattleshipAI {

    pub fn make_move(&mut self, opponent:&mut TwistPlayer) {
        match self.difficulty {
            AIDifficulty::Easy => self.random_attack(opponent),
            AIDifficulty::Medium => self.strategic_attack(opponent),
            AIDifficulty::Hard => self.card_based_attack(opponent),
        }
    }

    pub fn random_attack(&mut self, opponent: &mut TwistPlayer) {
        let mut rng = ::rand::rng();
        let x = rng.random_range(0..GRID_SIZE);
        let y = rng.random_range(0..GRID_SIZE);
        
        if !self.memory.contains(&(x, y)) {
            self.memory.push((x, y));
            self.player.base.fire_missile(&mut opponent.base, x, y);
        }
    }

    pub fn strategic_attack(&mut self, opponent: &mut TwistPlayer) {
        let mut rng = ::rand::rng();
        let last_hit = self.memory.last()
        .and_then(|&(x,y)| Some((x,y))).unwrap_or_else(|| (
            rng.random_range(0..GRID_SIZE),
            rng.random_range(0..GRID_SIZE)
        ));

        self.attack_near(last_hit.0,last_hit.1,opponent);
    }

    pub fn card_based_attack(&mut self, opponent: &mut TwistPlayer) {
        if self.player.use_card(ActionType::Torpedo) {
            self.use_torpedo(opponent)
        } else if self.player.use_card(ActionType::RadarScan) {
            self.use_radar(opponent);
        } else {
            self.strategic_attack(opponent);
        }
    }

    pub fn use_torpedo(&mut self, opponent: &mut TwistPlayer) {
        let mut rng = ::rand::rng();
        let col = rng.random_range(0..GRID_SIZE);
        
        self.player.use_card(ActionType::Torpedo);
        self.player.fire_torpedo(opponent,col);
    }

    pub fn use_radar(&mut self, opponent: &mut TwistPlayer) {
        let mut rng = ::rand::rng();
        let x = rng.random_range(0..GRID_SIZE);
        let y = rng.random_range(0..GRID_SIZE);
        
        self.player.use_card(ActionType::RadarScan);
        self.player.radar_scan(opponent,x,y);
    }

    pub fn attack_near(&mut self, x: usize, y: usize, opponent: &mut TwistPlayer) {
        // Check adjacent cells
        let targets = [
            (x.saturating_sub(1), y),  // Safer than wrapping_sub
            (x + 1, y).min((GRID_SIZE-1,GRID_SIZE-1)),
            (x, y.saturating_sub(1)),
            (x, y + 1).min((GRID_SIZE-1, GRID_SIZE-1))
        ];

        for &(tx, ty) in &targets {
            if !self.memory.contains(&(tx, ty)) {
                self.memory.push((tx, ty));
                self.player.base.fire_missile(
                    &mut opponent.base,
                    tx,
                    ty
                );
                return;
            }
        }
        self.random_attack(opponent);
    }
}