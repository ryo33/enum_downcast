use enum_downcast::EnumDowncast;

#[derive(EnumDowncast)]
enum Enum {
    String(String),
    Number(u32),
}

fn main() {
    let mut container = Enum::String("Hello".to_string());

    assert!(container.enum_downcast_ref::<u32>().is_none());

    let string_ref = container.enum_downcast_ref::<String>().unwrap();
    assert_eq!(string_ref, "Hello");

    let string_mut = container.enum_downcast_mut::<String>().unwrap();
    string_mut.push_str(" World");

    let string: String = container.enum_downcast::<String>().ok().unwrap();
    assert_eq!(string, "Hello World");

    let container = Enum::Number(100);

    assert!(container.enum_downcast_ref::<String>().is_none());

    let number_ref = container.enum_downcast_ref::<u32>().unwrap();
    assert_eq!(*number_ref, 100);
}

#[test]
fn test() {
    main();
}
