#![no_std]

#[cfg(feature = "derive")]
pub use enum_downcast_derive::EnumDowncast;

pub trait IntoVariant<T> {
    fn into_variant(self) -> Result<T, Self>
    where
        Self: Sized;
}

pub trait AsVariant<T> {
    fn as_variant(&self) -> Option<&T>;
}

pub trait AsVariantMut<T> {
    fn as_variant_mut(&mut self) -> Option<&mut T>;
}

pub trait EnumDowncast {
    fn enum_downcast<T>(self) -> Result<T, Self>
    where
        Self: IntoVariant<T> + Sized;

    fn enum_downcast_ref<T>(&self) -> Option<&T>
    where
        Self: AsVariant<T>;

    fn enum_downcast_mut<T>(&mut self) -> Option<&mut T>
    where
        Self: AsVariantMut<T>;
}

impl<Enum> EnumDowncast for Enum {
    fn enum_downcast<T>(self) -> Result<T, Self>
    where
        Self: IntoVariant<T> + Sized,
    {
        self.into_variant()
    }

    fn enum_downcast_ref<T>(&self) -> Option<&T>
    where
        Self: AsVariant<T>,
    {
        self.as_variant()
    }

    fn enum_downcast_mut<T>(&mut self) -> Option<&mut T>
    where
        Self: AsVariantMut<T>,
    {
        self.as_variant_mut()
    }
}
