use enum_downcast::{AsVariant, AsVariantMut, EnumDowncast, IntoVariant};

enum Enum {
    PlayerName(String),
    EnemyName(String),
    Other,
}

impl IntoVariant<String> for Enum {
    fn into_variant(self) -> Result<String, Self>
    where
        Self: Sized,
    {
        match self {
            Enum::PlayerName(name) => Ok(name),
            Enum::EnemyName(name) => Ok(name),
            Enum::Other => Err(self),
        }
    }
}

impl AsVariant<String> for Enum {
    fn as_variant(&self) -> Option<&String> {
        match self {
            Enum::PlayerName(name) => Some(name),
            Enum::EnemyName(name) => Some(name),
            Enum::Other => None,
        }
    }
}

impl AsVariantMut<String> for Enum {
    fn as_variant_mut(&mut self) -> Option<&mut String> {
        match self {
            Enum::PlayerName(name) => Some(name),
            Enum::EnemyName(name) => Some(name),
            Enum::Other => None,
        }
    }
}

impl IntoVariant<i32> for Enum {
    fn into_variant(self) -> Result<i32, Self>
    where
        Self: Sized,
    {
        Err(self)
    }
}

impl AsVariant<i32> for Enum {
    fn as_variant(&self) -> Option<&i32> {
        None
    }
}

impl AsVariantMut<i32> for Enum {
    fn as_variant_mut(&mut self) -> Option<&mut i32> {
        None
    }
}

fn main() {
    let container = Enum::PlayerName("Player".to_string());
    assert!(container.enum_downcast_ref::<String>().is_some());
    assert!(container.enum_downcast_ref::<i32>().is_none());

    let container = Enum::EnemyName("Enemy".to_string());
    assert!(container.enum_downcast_ref::<String>().is_some());
    assert!(container.enum_downcast_ref::<i32>().is_none());

    let container = Enum::Other;
    assert!(container.enum_downcast_ref::<String>().is_none());
    assert!(container.enum_downcast_ref::<i32>().is_none());
}
