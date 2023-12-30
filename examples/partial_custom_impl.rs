use enum_downcast::{AsVariant, EnumDowncast};

struct Player {
    name: String,
}

struct Enemy {
    power: u32,
}

struct Coffee;
struct Donut;

#[derive(EnumDowncast)]
enum Enum {
    Player(Player),
    Enemy(Enemy),
    #[enum_downcast(skip)]
    CoffeeAndDonut {
        coffee: Coffee,
        donut: Donut,
    },
}

impl AsVariant<Coffee> for Enum {
    fn as_variant(&self) -> Option<&Coffee> {
        match self {
            Enum::CoffeeAndDonut { coffee, .. } => Some(coffee),
            _ => None,
        }
    }
}

impl AsVariant<Donut> for Enum {
    fn as_variant(&self) -> Option<&Donut> {
        match self {
            Enum::CoffeeAndDonut { donut, .. } => Some(donut),
            _ => None,
        }
    }
}

fn main() {
    // player in container
    let container = Enum::Player(Player {
        name: "Player".to_string(),
    });

    assert_eq!(
        container.enum_downcast_ref::<Player>().unwrap().name,
        "Player"
    );
    assert!(container.enum_downcast_ref::<Enemy>().is_none());
    assert!(container.enum_downcast_ref::<Coffee>().is_none());
    assert!(container.enum_downcast_ref::<Donut>().is_none());

    // enemy in container
    let container = Enum::Enemy(Enemy { power: 100 });

    assert!(container.enum_downcast_ref::<Player>().is_none());
    assert_eq!(container.enum_downcast_ref::<Enemy>().unwrap().power, 100);
    assert!(container.enum_downcast_ref::<Coffee>().is_none());
    assert!(container.enum_downcast_ref::<Donut>().is_none());

    let container = Enum::CoffeeAndDonut {
        coffee: Coffee,
        donut: Donut,
    };

    assert!(container.enum_downcast_ref::<Player>().is_none());
    assert!(container.enum_downcast_ref::<Enemy>().is_none());
    assert!(container.enum_downcast_ref::<Coffee>().is_some());
    assert!(container.enum_downcast_ref::<Donut>().is_some());
}
