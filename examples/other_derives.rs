use enum_downcast::EnumDowncast;
use serde::{Deserialize, Serialize};
use strum::{EnumVariantNames, VariantNames};

#[derive(Serialize, Deserialize)]
struct Player {
    name: String,
}
#[derive(Serialize, Deserialize)]
struct Enemy {
    power: u32,
}
#[derive(Serialize, Deserialize)]
struct Item(String);

#[derive(EnumDowncast, Serialize, Deserialize, EnumVariantNames)]
enum Enum {
    Player(Player),
    Enemy(Enemy),
    Items { vec: Vec<Item> },
}

fn main() {
    // player in container
    let container = Enum::Player(Player {
        name: "Player".to_string(),
    });

    assert!(container.enum_downcast_ref::<Enemy>().is_none());
    assert!(container.enum_downcast_ref::<Vec<Item>>().is_none());

    let player_ref = container.enum_downcast_ref::<Player>().unwrap();
    assert_eq!(player_ref.name, "Player");

    let serialized = serde_json::to_string_pretty(&container).unwrap();
    println!("{}", serialized);

    // enemy in container
    let container = Enum::Enemy(Enemy { power: 100 });

    assert!(container.enum_downcast_ref::<Player>().is_none());
    assert!(container.enum_downcast_ref::<Vec<Item>>().is_none());

    let enemy_ref = container.enum_downcast_ref::<Enemy>().unwrap();
    assert_eq!(enemy_ref.power, 100);

    let serialized = serde_json::to_string_pretty(&container).unwrap();
    println!("{}", serialized);

    // items in container
    let container = Enum::Items {
        vec: vec![Item("Item".to_string())],
    };

    assert!(container.enum_downcast_ref::<Player>().is_none());
    assert!(container.enum_downcast_ref::<Enemy>().is_none());

    let serialized = serde_json::to_string_pretty(&container).unwrap();
    println!("{}", serialized);

    // discrimninants
    assert_eq!(Enum::VARIANTS, &["Player", "Enemy", "Items"]);
}
