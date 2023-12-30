use enum_downcast::EnumDowncast;

#[derive(EnumDowncast)]
enum Enum {
    String(String),
    Int(i32),
    #[enum_downcast(skip)]
    Other,
}

fn main() {
    let container = Enum::String("Hello".to_string());
    assert!(container.enum_downcast_ref::<String>().is_some());
    assert!(container.enum_downcast_ref::<i32>().is_none());

    let container = Enum::Int(42);
    assert!(container.enum_downcast_ref::<i32>().is_some());
    assert!(container.enum_downcast_ref::<String>().is_none());

    let container = Enum::Other;
    assert!(container.enum_downcast_ref::<String>().is_none());
    assert!(container.enum_downcast_ref::<i32>().is_none());
}
