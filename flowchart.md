# Flowchart

Define the nessesary Structs and Enums
    Board (struct) - Stores the grid and ships
    Cell (enum) - represents board space (empty, ship, hit, miss)
    Ship (struct) - Position, size, health
    ShipType (enum) - Ship catagories
    Action (enum) - represents possible actions
    Deck (struct) - shuffled list of actions
    Player (struct) - players board, ships, and deck
    GameState (struct) - Tracks turns and handles win conditions

Implement the game board
    Make a 10x10 grid
    Function to place ships while ensuring no overlaps
    function to display board

Defines Ships and their properties
    Diffirent ship sizes and unique IDs
    Track health and determine when a ship sinks
    ensure ships are place in the correct possitions with no overlaps or placed in board boundaries

Implement the action deck
    Define a function for each action card
        Missle
        Torpedo
        Move ship
        reinforce 
        radar scan
        etc
    shuffle deck function
    draw from deck
    
Turn Based logic
    Each player draws an action from their deck
    execute the action
    check for ship destruction and if a player lost
    alternate between players

Game Loop
    run game until win condition is met
    display board states 
    handle input 

Implement very simple AI
    AiPlayer struct
    Ai randomly draws actions 
    Ai makes intelligent targeting choices

Playtest and debugging 

