//combat.rs
use rand::Rng;
use crate::player::Player;
use crate::monster::Monster;

pub enum CombatOutcome {
    Ongoing,
    PlayerWon,
    PlayerFled,
}


pub struct Combat {
    pub player: Player,
    pub monster: Monster,
}

impl Combat {
    pub fn dodge_succeeds(speed: u32) -> bool {
        let roll = rand::thread_rng().gen_range(1..=6);
        roll <= speed
    }

    pub fn resolve_round(player: &mut Player, monster: &mut Monster, monster_attacks_first: bool) -> (CombatOutcome, Vec<String>) {
        let mut log = Vec::new();
    
        if monster_attacks_first {
            if Self::dodge_succeeds(player.speed) {
                log.push("You dodge the attack!".to_string());
            }
        } else {
            player.health -= monster.strength;
            log.push(format!("The {} hits you for {} damage.", monster.name, monster.strength));
        }

        monster.health -= player.strength;
        log.push(format!("You hit the {} for {} damage.", monster.name, player.strength));
        if monster.health <= 0 {
            log.push(format!("The {} falls.", monster.name));
            return (CombatOutcome::PlayerWon, log);
        }

        if !monster_attacks_first {
            if Self::dodge_succeeds(player.speed) {
                log.push("You dodged the attack!".to_string());
            } else {
                player.health -= monster.strength;
                log.push(format!("The {} hits you for {} damage.", monster.name, monster.strength));
            }
        }
        (CombatOutcome::Ongoing, log)
    }
}
