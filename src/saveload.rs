
#[derive(Serialize, Deserialize)]
pub struct SaveState {
    pub player1: PlayerData,
    pub player2: PlayerData,
    pub turn: GameState,
    pub twist_mode: bool,
    pub turn_counter: f64,
}

#[derive(Serialize, Deserialize)]
pub enum PlayerData {
    Classic(BasePlayer),
    Twist(TwistPlayer)
}

pub fn save_game(state: &SaveState) -> Result<(), std::io::Error> {
    let data = bincode::serialize(state).unwrap();

    std::fs::write("savegame.bin",data)
}

pub fn load_game() -> Result<SaveState, Box<dyn std::error::Error>> {
    let save_data = std::fs::read("savegame.bin")?;

    let save_state: SaveState = bincode::deserialize(&save_data)?;
    Ok(save_state)
}
