use enum_dispatch::enum_dispatch;
use enum_downcast::EnumDowncast;

#[enum_dispatch]
trait GetName {
    fn get_name(&self) -> &str;
}

struct Player {
    name: String,
}

struct Enemy {
    name: String,
}

impl GetName for Player {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl GetName for Enemy {
    fn get_name(&self) -> &str {
        &self.name
    }
}

#[enum_dispatch(GetName)]
#[derive(EnumDowncast)]
enum Enum {
    Player,
    Enemy,
}

fn main() {
    let container = Enum::Player(Player {
        name: "Player".to_string(),
    });

    assert_eq!(container.get_name(), "Player");
    assert_eq!(
        container.enum_downcast_ref::<Player>().unwrap().name,
        "Player"
    );
    assert!(container.enum_downcast_ref::<Enemy>().is_none());

    let container = Enum::Enemy(Enemy {
        name: "Enemy".to_string(),
    });

    assert_eq!(container.get_name(), "Enemy");
    assert_eq!(
        container.enum_downcast_ref::<Enemy>().unwrap().name,
        "Enemy"
    );
    assert!(container.enum_downcast_ref::<Player>().is_none());
}

#[test]
fn test() {
    main();
}
