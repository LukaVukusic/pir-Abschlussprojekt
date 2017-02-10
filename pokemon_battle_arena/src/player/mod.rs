pub mod human;
pub mod ki;


use arena;
use db::pokemon_token::PokemonToken;
use db::pokedex::*;
use db::{enums, moves};


/// The Player Trait must be implemented by every sort of human players or ai´s

#[derive(Clone)]
pub enum PlayerType {
    Human,
    SimpleAi,
}

#[derive(Clone)]
pub struct Player {
    player: PlayerType,
    pokemon_list: Vec<PokemonToken>,
    pokemon_count: usize,
    current: usize,
    next_move: Option<moves::Technique>, 
}

impl Player {
    //
    // Constuctor
    //
    /// Takes an array with pokemon id´s and the PlayerType and returns a player
    pub fn new_by_id(input: &[usize], player_type: PlayerType) -> Self {
        let mut pokemon = Vec::new();
        let len = input.len();
        let dex = Pokedex::new();
        for i in 0..input.len() {
            pokemon.push(PokemonToken::from_model(dex.pokemon_by_id(input[i]).unwrap()));
        }
        Player {
            player: player_type,
            pokemon_list: pokemon,
            pokemon_count: len,
            current: 0,
            next_move: None,
        }
    }

    //
    // Getter Methods
    //
    /// Returns the list of pokemon choosen by the player
    pub fn get_pokemon_list(&self) -> &Vec<PokemonToken> {
        &self.pokemon_list
    }
    /// Returns the currently fighting pokemon
    pub fn get_current(&self) -> usize {
        self.current
    }
    /// Gets the amount of pokemon choosen by the player
    pub fn get_pokemon_count(&self) -> usize {
        self.pokemon_count
    }
    /// Returns the amount of pokemon with atleast one hp
    pub fn get_alive_count(&self) -> usize {
        self.pokemon_list.iter().filter(|x| x.get_current().get_stat(enums::Stats::Hp) != 0)
            .count()
    }
    /// Returns a Vec with the id´s from the pokemon which are alive
    pub fn get_alive_list(&self) -> Vec<usize> {
        let mut vec = Vec::new();
        for i in 0..self.pokemon_list.len() {
            if self.pokemon_list[i].get_current().get_stat(enums::Stats::Hp) != 0 {
                vec.push(i);
            }
        }
        vec
    }
    /// Gets a attack saved in an attackslot
    pub fn get_attack(&mut self, slot: &AttackSlot) -> moves::Technique {
        match slot {
            &AttackSlot::One => self.pokemon_list[self.current].clone().get_move_one().unwrap(),
            &AttackSlot::Two => self.pokemon_list[self.current].clone().get_move_two().unwrap(),
            &AttackSlot::Three => self.pokemon_list[self.current].clone().get_move_three().unwrap(),
            &AttackSlot::Four =>  self.pokemon_list[self.current].clone().get_move_four().unwrap(),
        }
    }
    /// Gets the next attack from the Player. Returns none if no Technique is selected 
    pub fn get_next_move(&self) -> Option<moves::Technique> {
        self.next_move.clone()
    }

    //
    // Setter Methods
    //
    /// Sets the current value (e.g. after a pokemon swap)
    /// Given values should be between 1 and the maximum amount of pokemon you have
    pub fn set_current(&mut self, new: usize) {
        self.current = new;
    }
    pub fn set_next_move(&mut self, next: Option<moves::Technique>) {
        self.next_move = next;
    }
    //
    // Other
    //
    pub fn attack_or_swap(&self) -> arena::to_ui::Move {
        match self.player {
            PlayerType::Human => arena::to_ui::ui_move(),
            PlayerType::SimpleAi => arena::to_ui::ui_move(),
        }
    }
}

/// An enum which represents the AttackSlot to match with it
pub enum AttackSlot {
    One,
    Two,
    Three,
    Four,
}


//Ignore this section, it's only a note for me which work needs to be done:

//moves: TODOs in resolve_effect method (2x Heal), is_asleep methode für PokemonToken

//TODO Artur: hits in moves.rs, change stats + deal damage in resolve.rs, Methode zum errechnen
//der stats in stats.rs
