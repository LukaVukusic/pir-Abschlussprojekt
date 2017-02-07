extern crate rand;

use super::moves::Technique;
use super::pokemon_token::PokemonToken;
use super::enums;
use self::rand::{Rng, thread_rng};
use player::Player;

///Resolves moves that simply deals damage to the opponent.
pub fn deal_damage(attack: Technique, user: PokemonToken, target: PokemonToken) -> u16 {
    unimplemented!();
    //TODO: Methode die matcht zwischen Attacken die direkt verrechnet werden können und denen,
    //die variable Power haben. Hier muss eine Möglichkeit gefunden werden die Power möglichst
    //effizient für alle Attacken zu berechnen.
    let mut stab = 0;
    let mut rng = thread_rng();
    let random = rng.gen_range(0.85, 1);
    if attack.attack_type == user.get_types() {
        stab = 1.5;
    } else {
        stab = 1;

    }
    let mut modifier = stab*model.get_types()*random;
    (((2*user.get_level+10)/250)*user.dv.attack/user.dv.defense*attack.power+2)*modifier as u16
}

pub fn ailment(ailment: enums::Ailment, effect_chance: u8, target: PokemonToken) {
    let mut rng = thread_rng();
    let random = rng.gen_range(1, 101);
    let probability = effect_chance;
    if random <= probability {
        match ailment {
            enums::Ailment::Confusion => {},
            _ => {},
        }
    }
}

//TODO: Methode implementieren, die errechnet wie viel ein Stage für das entsprechende Pokemon ist
//und den Stat entsprechend verringert/erhöht, wenn Stage 6/-6 noch nicht erreicht ist. Gibt einen
//bool zurück der anzeigt, ob der Stat verändert wurde oder nicht.
pub fn change_stats(stages: i8, stat: enums::Stats, target: PokemonToken) -> bool {
    true
}

pub fn heal(target: PokemonToken, value: u16) {
    if value + target.get_current().get_stat(enums::Stats::Hp) >= target.get_base().
    get_stat(enums::Stats::Hp) {
        target.get_current().set_stats(enums::Stats::Hp, target.get_base().
            get_stat(enums::Stats::Hp));
    } else {
        target.get_current().set_stats(enums::Stats::Hp, (target.get_current().
            get_stat(enums::Stats::Hp) + value));
    }
}

pub fn switch_pokemon<T> (target: T)
    where T: Player {
        unimplemented!();
}

pub fn ko_attack (target: PokemonToken) {
    target.get_current().set_stats(enums::Stats::Hp, 0);
}
