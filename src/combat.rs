//combat.rs
use rand::Rng;
use crate::player::Player;
use crate::monster::Monster;

pub enum CombatOutcome {
    Ongoing,
    PlayerWon,
    PlayerFled,
    PlayerDefeated,
}

    pub fn dodge_succeeds(speed: i32) -> bool {
        let roll = rand::thread_rng().gen_range(1..=12);
        roll <= speed
    }
        
    pub fn resolve_round(player: &mut Player, monster: &mut Monster, monster_attacks_first: bool) -> (CombatOutcome, Vec<String>) {
        let mut log = Vec::new();
    
        match monster_attacks_first {
            true => {
                if dodge_succeeds(player.speed) {
                    log.push("You dodge the attack!".to_string());
                } else {
                    player.health -= monster.strength;
                    log.push(format!("The {} hits you for {} damage.", monster.name, monster.strength));
                }//monster ambushes and you have a chance to dodge
                if player.health <= 0 {
                    log.push("Everything goes dark...".to_string());
                    return (CombatOutcome::PlayerDefeated, log);
                } //if you die, you respawn with depleated health
                if dodge_succeeds(monster.speed) {
                    log.push("You missed!".to_string());
                } else {
                    monster.health -= player.strength;
                    log.push(format!("You hit the {} for {} damage.", monster.name, player.strength));
                }//player counter attacks and monster has opportunity to dodge
                if monster.health <= 0 {
                    log.push(format!("The {} falls.", monster.name));
                    return (CombatOutcome::PlayerWon, log);
                }
            }
            false => {
                if dodge_succeeds(monster.speed) {
                    log.push("You missed!".to_string());
                } else {
                    monster.health -= player.strength;
                    log.push(format!("You hit the {} for {} damage.", monster.name, player.strength));
                }//player counter attacks and monster has opportunity to dodge
                if monster.health <= 0 {
                    log.push(format!("The {} falls.", monster.name));
                    return (CombatOutcome::PlayerWon, log);
                }
                if dodge_succeeds(player.speed) {
                    log.push("You dodge the attack!".to_string());
                } else {
                    player.health -= monster.strength;
                    log.push(format!("The {} hits you for {} damage.", monster.name, monster.strength));
                }//monster ambushes and you have a chance to dodge
                if player.health <= 0 {
                    log.push("Everything goes dark...".to_string());
                    return (CombatOutcome::PlayerDefeated, log);
                } //if you die, you respawn with depleated health
            }
        }
        (CombatOutcome::Ongoing, log)
    }

