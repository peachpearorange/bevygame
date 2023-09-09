// use crate::foxtrot::components;

use {crate::components::{AttackPlayer, Char, *},
     bevy::{ecs::{bundle::*, component},
            prelude::Name},
     std::borrow::Cow};

// const PLAYER: (Char, Name, Container, Combat, Player) =
//   (Char('😔'), Name::from("player"), CONTAINEREMPTY, Combat { hp: 300, damage: 2 }, Player);
pub fn player() -> (Char, Name, Container, Combat, Player) {
  (Char('😔'), name("player"), Container::empty(), Combat { hp: 300, damage: 2 }, Player)
}
// fn wall() -> (Tile, Name, Char) {
//   (Tile { is_wall: true,
//           bg_color: "666666" },
//    name("wall"),
//    Char('#'))
// }
// fn tree() -> (Tile, Name, Char) {
//   (Tile { is_wall: true,
//           bg_color: "27AD00" },
//    name("tree"),
//    Char('🌲'))
// }
// fn rock() -> (Tile, Name, Char) {
//   (Tile { is_wall: true,
//           bg_color: "71A269" },
//    name("rock"),
//    Char('🪨'))
// }
// fn water() -> (Tile, Name) {
//   (Tile { is_wall: false,
//           bg_color: "5961FF" },
//    name("water"))
// }
// fn sand() -> (Tile, Name) {
//   (Tile { is_wall: false,
//           bg_color: "D9DC60" },
//    name("sand"))
// }
// fn grass() -> (Tile, Name) {
//   (Tile { is_wall: false,
//           bg_color: "22B800" },
//    name("grass"))
// }
// fn ladder() -> (Tile, Name, Char) {
//   (Tile { is_wall: false,
//           bg_color: "4A4A4A" },
//    name("ladder down"),
//    Char('🪜'))
// }
// pub fn enemy() -> (Char, Name, EnemyMovement, AttackPlayer, Combat) {
//   (Char('👿'), name("enemy"), EnemyMovement, AttackPlayer, Combat { hp: 30, damage: 1 })
// }
// pub fn dragon() -> (Char, Name, EnemyMovement, DragonAttack, AttackPlayer, Combat) {
//   (Char('🐉'), name("dragon"), EnemyMovement, DragonAttack, AttackPlayer, Combat { hp: 60, damage: 1 })
// }
// pub fn fire() -> (Char, Name, Fire) { (Char('🔥'), name("fire"), Fire { dir: (1, 0) }) }
// pub fn snowman() -> (Char, Name, RandomMovement) { (Char('⛄'), name("snowman"), RandomMovement) }
// pub fn spider() -> (Char, Name, AttackPlayer) { (Char('🕷'), name("spider"), AttackPlayer) }
// pub fn sheep() -> (Char, Name, RandomMovement) { (Char('🐑'), name("sheep"), RandomMovement) }
// pub fn duck() -> (Char, Name, RandomMovement) { (Char('🦆'), name("duck"), RandomMovement) }
// pub fn rabbit() -> (Char, Name, RandomMovement) { (Char('🐇'), name("rabbit"), RandomMovement) }

// const WALL: (Tile, Name, Char) = (Tile { is_wall: true,
//                                          bg_color: "666666" },
//                                   name("wall"),
//                                   Char('#'));
// const TREE: (Tile, Name, Char) = (Tile { is_wall: true,
//                                          bg_color: "27AD00" },
//                                   name("tree"),
//                                   Char('🌲'));
// const ROCK: (Tile, Name, Char) = (Tile { is_wall: true,
//                                          bg_color: "71A269" },
//                                   name("rock"),
//                                   Char('🪨'));
// const WATER: (Tile, Name) = (Tile { is_wall: false,
//                                     bg_color: "5961FF" },
//                              name("water"));
// const SAND: (Tile, Name) = (Tile { is_wall: false,
//                                    bg_color: "D9DC60" },
//                             name("sand"));
// const GRASS: (Tile, Name) = (Tile { is_wall: false,
//                                     bg_color: "22B800" },
//                              name("grass"));
// const LADDER: (Tile, Name, Char) = (Tile { is_wall: false,
//                                            bg_color: "4A4A4A" },
//                                     name("ladder down"),
//                                     Char('🪜'));
// const ENEMY: (Char, Name, EnemyMovement, AttackPlayer, Combat) =
//   (Char('👿'), name("enemy"), EnemyMovement, AttackPlayer, Combat { hp: 30, damage: 1 });
// const DRAGON: (Char, Name, EnemyMovement, DragonAttack, AttackPlayer, Combat) =
//   (Char('🐉'), name("dragon"), EnemyMovement, DragonAttack, AttackPlayer, Combat { hp: 60, damage: 1 });
// const FIRE: (Char, Name, Fire) = (Char('🔥'), name("fire"), Fire { dir: (1, 0) });
// const SNOWMAN: (Char, Name, RandomMovement) = (Char('⛄'), name("snowman"), RandomMovement);
// const SPIDER: (Char, Name, AttackPlayer) = (Char('🕷'), name("spider"), AttackPlayer);
// const SHEEP: (Char, Name, RandomMovement) = (Char('🐑'), name("sheep"), RandomMovement);
// const DUCK: (Char, Name, RandomMovement) = (Char('🦆'), name("duck"), RandomMovement);
// const RABBIT: (Char, Name, RandomMovement) = (Char('🐇'), name("rabbit"), RandomMovement);
// const CHEST: _ = Container {};
// const ITEMS:_ = (Loot{Char(b"💰"),
//                       Name("loot"),
//                       Takeable(true)},
//                 Wood{Char(b"🪵"),
//                       Name("wood"),
//                       Takeable(true)},
//                 Fish{Char(b"🐟"),
//                       Name("fish"),
//                       Takeable(true)});
