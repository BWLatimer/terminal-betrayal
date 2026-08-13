//combat.rs
use rand::Rng;
use crate::player::Player;
use crate::monster::Monster;

pub enum CombatOutcome {
    Ongoing,
    PlayerWon,
    PlayerFled,
}

    pub fn dodge_succeeds(speed: i32) -> bool {
        let roll = rand::thread_rng().gen_range(1..=12);
        roll <= speed
    }

    pub fn resolve_round(player: &mut Player, monster: &mut Monster, monster_attacks_first: bool) -> (CombatOutcome, Vec<String>) {
        let mut log = Vec::new();
    
        if monster_attacks_first {

            if dodge_succeeds(player.speed) {
                log.push("You dodge the attack!".to_string());
            
            } else {
                player.health -= monster.strength;
                log.push(format!("The {} hits you for {} damage.", monster.name, monster.strength));
            }
        monster.health -= player.strength;
        log.push(format!("You hit the {} for {} damage.", monster.name, player.strength));
        } else if !monster_attacks_first {
            if dodge_succeeds(player.speed) {
                log.push("You dodged the attack!".to_string());
            } else {
                player.health -= monster.strength;
                log.push(format!("The {} hits you for {} damage.", monster.name, monster.strength));
            }
        }
        if monster.health <= 0 {
            log.push(format!("The {} falls.", monster.name));
            return (CombatOutcome::PlayerWon, log);
        }
        (CombatOutcome::Ongoing, log)
    }

