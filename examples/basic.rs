use enum_downcast::EnumDowncast;

struct Player {
    name: String,
}
struct Enemy {
    power: u32,
}

#[allow(dead_code)]
struct Item(String);

#[derive(EnumDowncast)]
enum Enum {
    Player(Player),
    Enemy(Enemy),
    Items { vec: Vec<Item> },
}

fn main() {
    // player in container
    let mut container = Enum::Player(Player {
        name: "Player".to_string(),
    });

    assert!(container.enum_downcast_ref::<Enemy>().is_none());
    assert!(container.enum_downcast_ref::<Vec<Item>>().is_none());

    let player_ref = container.enum_downcast_ref::<Player>().unwrap();
    assert_eq!(player_ref.name, "Player");

    let player_mut = container.enum_downcast_mut::<Player>().unwrap();
    player_mut.name = "Player2".to_string();
    assert_eq!(player_mut.name, "Player2");

    let player = container.enum_downcast::<Player>().ok().unwrap();
    assert_eq!(player.name, "Player2");

    // enemy in container
    let container = Enum::Enemy(Enemy { power: 100 });

    assert!(container.enum_downcast_ref::<Player>().is_none());
    assert!(container.enum_downcast_ref::<Vec<Item>>().is_none());

    let enemy_ref = container.enum_downcast_ref::<Enemy>().unwrap();
    assert_eq!(enemy_ref.power, 100);

    // items in container
    let container = Enum::Items {
        vec: vec![Item("Item".to_string())],
    };

    assert!(container.enum_downcast_ref::<Player>().is_none());
    assert!(container.enum_downcast_ref::<Enemy>().is_none());

    for item in container
        .enum_downcast_ref::<Vec<Item>>()
        .into_iter()
        .flatten()
    {
        println!("{}", item.0);
    }
}

#[test]
fn test() {
    main();
}
