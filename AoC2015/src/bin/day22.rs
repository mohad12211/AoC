#![allow(non_upper_case_globals)]

#[derive(Clone)]
struct Player {
    hp: i32,
    armor: i32,
    mana: i32,
}

#[derive(Clone)]
struct Boss {
    hp: i32,
    dmg: i32,
}

#[derive(Clone)]
struct Game {
    mana_spent: i32,
    spells: Vec<Spell>,
    player: Player,
    boss: Boss,
    hard_mode: bool,
}

#[derive(Clone, Copy)]
struct Spell {
    timer: Option<i32>,
    cost: i32,
    action: fn(&mut Player, &mut Boss),
}

const MagicMissile: Spell = Spell {
    timer: None,
    cost: 53,
    action: |_, boss| {
        boss.hp -= 4;
    },
};

const Drain: Spell = Spell {
    timer: None,
    cost: 73,
    action: |player, boss| {
        boss.hp -= 2;
        player.hp += 2;
    },
};

const Shield: Spell = Spell {
    timer: Some(6),
    cost: 113,
    action: |player, _| {
        player.armor = 7;
    },
};

const Poison: Spell = Spell {
    timer: Some(6),
    cost: 173,
    action: |_, boss| {
        boss.hp -= 3;
    },
};

const Recharge: Spell = Spell {
    timer: Some(5),
    cost: 229,
    action: |player, _| {
        player.mana += 101;
    },
};

const spells: [Spell; 5] = [MagicMissile, Drain, Shield, Poison, Recharge];

enum GameResult {
    Win,
    Loss,
}

impl Game {
    fn new(hard_mode: bool) -> Self {
        let player = Player {
            hp: 50,
            armor: 0,
            mana: 500,
        };
        // NOTE: Input
        let boss = Boss { hp: 71, dmg: 10 };
        Game {
            mana_spent: 0,
            spells: Vec::new(),
            player,
            boss,
            hard_mode,
        }
    }

    fn play(&mut self, spell: Spell) -> Option<GameResult> {
        let result = self.player_turn(spell);
        if result.is_some() {
            return result;
        }
        let result = self.boss_turn();
        if result.is_some() {
            return result;
        }
        None
    }

    fn apply_effects(&mut self) {
        // only apply armor if armor effect is there
        self.player.armor = 0;
        for spell in self.spells.iter_mut() {
            (spell.action)(&mut self.player, &mut self.boss);
            *spell
                .timer
                .as_mut()
                .expect("only spells with timers are added") -= 1;
        }
        self.spells
            .retain(|spell| spell.timer.expect("only spells with timers are added") > 0);
    }

    fn check_dead(&mut self) -> Option<GameResult> {
        if self.player.hp <= 0 {
            Some(GameResult::Loss)
        } else if self.boss.hp <= 0 {
            Some(GameResult::Win)
        } else {
            None
        }
    }

    fn player_turn(&mut self, spell: Spell) -> Option<GameResult> {
        if self.hard_mode {
            self.player.hp -= 1;
            let result = self.check_dead();
            if result.is_some() {
                return result;
            }
        }
        self.apply_effects();
        if let Some(r) = self.check_dead() {
            return Some(r);
        }
        // NOTE: assuming that spells are ordered by cost
        if self.player.mana < spells[0].cost {
            return Some(GameResult::Loss);
        }

        self.player.mana -= spell.cost;
        self.mana_spent += spell.cost;
        if spell.timer.is_some() {
            self.spells.push(spell);
        } else {
            (spell.action)(&mut self.player, &mut self.boss);
        }

        self.check_dead()
    }

    fn boss_turn(&mut self) -> Option<GameResult> {
        self.apply_effects();
        if let Some(r) = self.check_dead() {
            return Some(r);
        }
        self.player.hp = (self.player.hp) - (self.boss.dmg - self.player.armor);
        self.check_dead()
    }

    fn is_unplayable_spell(&self, spell: &Spell) -> bool {
        self.spells
            .iter()
            // NOTE: spells with timer `1` can be played since we can play the spell _after_ it applies
            // is effect in the next turn
            .any(|x| x.cost == spell.cost && x.timer >= Some(2))
    }
}

fn main() {
    part1();
    part2();
}

fn part2() {
    let mut min_mana = i32::max_value();

    for spell in spells {
        let mut game = Game::new(true);
        match game.play(spell) {
            Some(_) => unreachable!(),
            None => search(game, &mut min_mana),
        };
    }
    println!("{min_mana}");
}

fn part1() {
    let mut min_mana = i32::max_value();

    for spell in spells {
        let mut game = Game::new(false);
        match game.play(spell) {
            Some(_) => {}
            None => search(game, &mut min_mana),
        };
    }
    println!("{min_mana}");
}

fn search(game: Game, min_mana: &mut i32) {
    for spell in spells {
        if game.is_unplayable_spell(&spell) {
            continue;
        }
        let mut new_game = game.clone();
        match new_game.play(spell) {
            Some(GameResult::Loss) => {}
            Some(GameResult::Win) => {
                if new_game.mana_spent < *min_mana {
                    *min_mana = new_game.mana_spent
                }
            }
            None => search(new_game, min_mana),
        };
    }
}
