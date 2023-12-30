// This example is only works with nightly Rust

#![feature(min_specialization)]

use enum_downcast::{AsVariant, AsVariantMut, EnumDowncast, IntoVariant};

#[derive(EnumDowncast)]
enum Enum {
    PlayerName(String),
    Number(i32),
}

impl<T> IntoVariant<T> for Enum {
    default fn into_variant(self) -> Result<T, Self>
    where
        Self: Sized,
    {
        // Always fail
        Err(self)
    }
}

impl<T> AsVariant<T> for Enum {
    default fn as_variant(&self) -> Option<&T> {
        // Always fail
        None
    }
}

impl<T> AsVariantMut<T> for Enum {
    default fn as_variant_mut(&mut self) -> Option<&mut T> {
        // Always fail
        None
    }
}

fn main() {
    let container = Enum::PlayerName("Player".to_string());
    assert!(container.enum_downcast_ref::<String>().is_some());
    assert!(container.enum_downcast_ref::<i32>().is_none());

    // Specialization
    assert!(container.enum_downcast_ref::<usize>().is_none());
    assert!(container.enum_downcast_ref::<&str>().is_none());

    let container = Enum::Number(42);
    assert!(container.enum_downcast_ref::<i32>().is_some());
    assert!(container.enum_downcast_ref::<String>().is_none());

    // Specialization
    assert!(container.enum_downcast_ref::<usize>().is_none());
    assert!(container.enum_downcast_ref::<&str>().is_none());
}
