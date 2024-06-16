use enum_downcast::EnumDowncast;

struct Player<T: Clone>(T);
struct Enemy<T: Clone>(T);

#[derive(EnumDowncast)]
enum Enum<T: Clone> {
    Player(Player<T>),
    Enemy(Enemy<T>),
}

fn main() {
    let mut container = Enum::Player(Player("Player".to_string()));

    assert!(container.enum_downcast_ref::<Enemy<String>>().is_none());

    let player_ref = container.enum_downcast_ref::<Player<String>>().unwrap();
    assert_eq!(player_ref.0, "Player");

    let player_mut = container.enum_downcast_mut::<Player<String>>().unwrap();
    player_mut.0 = "Player2".to_string();
    assert_eq!(player_mut.0, "Player2");

    let player = container.enum_downcast::<Player<String>>().ok().unwrap();
    assert_eq!(player.0, "Player2");

    let container = Enum::Enemy(Enemy(100));

    assert!(container.enum_downcast_ref::<Player<u32>>().is_none());

    let enemy_ref = container.enum_downcast_ref::<Enemy<u32>>().unwrap();
    assert_eq!(enemy_ref.0, 100);
}

#[test]
fn test() {
    main();
}
