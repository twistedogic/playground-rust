#![allow(dead_code)]

#[derive(Default, Debug, PartialEq)]
enum Skill {
    #[default]
    Stomp,
    Fire,
    Fly,
}

#[derive(Default, Debug, PartialEq)]
enum State {
    #[default]
    Mario,
    SuperMario,
    FireMario,
    CapeMario,
}

impl State {
    fn use_skill(&self) -> Skill {
        match self {
            FireMario => Fire,
            CapeMario => Fly,
            _ => Stomp,
        }
    }
}

enum PowerUp {
    Mushroom,
    Flower,
    Feather,
}

use PowerUp::*;
use Skill::*;
use State::*;

type Life = u8;

#[derive(Debug, PartialEq)]
struct Player(State, Life);

impl Player {
    fn new() -> Self {
        Player(Default::default(), 1)
    }
    fn collect(&mut self, item: PowerUp) {
        match (&self.0, item) {
            (Mario, Mushroom) => self.0 = SuperMario,
            (_, Flower) => self.0 = FireMario,
            (_, Feather) => self.0 = CapeMario,
            (_, Mushroom) => self.1 += 1,
        }
    }
    fn skill(&self) -> Skill {
        self.0.use_skill()
    }
}

#[test]
fn test_player() {
    let mut player = Player::new();
    assert_eq!(player.0, Mario);
    let seq: Vec<(PowerUp, State, Life, Skill)> = vec![
        (Mushroom, SuperMario, 1, Stomp),
        (Mushroom, SuperMario, 2, Stomp),
        (Flower, FireMario, 2, Fire),
        (Feather, CapeMario, 2, Fly),
        (Mushroom, CapeMario, 3, Fly),
        (Flower, FireMario, 3, Fire),
    ];
    for (powerup, state, life, skill) in seq {
        player.collect(powerup);
        assert_eq!(player, Player(state, life));
        assert_eq!(player.skill(), skill);
    }
}
