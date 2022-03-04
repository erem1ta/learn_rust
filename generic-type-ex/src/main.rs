// transform a Container struct that only accepts positive integers of the u32 type into a generic container that can hold values of any given type
struct Container<T> {
    value: T,
}

// need to define the generic type for the implement and the struct
impl<T> Container<T> {
    // no need to define the generic for each function
    pub fn new(value: T) -> Self {
        Container { value }
    }
}

fn main() {
    assert_eq!(Container::new(42).value, 42);
    assert_eq!(Container::new(3.14).value, 3.14);
    assert_eq!(Container::new("Foo").value, "Foo");
    assert_eq!(
        Container::new(String::from("Bar")).value,
        String::from("Bar")
    );
    assert_eq!(Container::new(true).value, true);
    assert_eq!(Container::new(-12).value, -12);
    assert_eq!(Container::new(Some("text")).value, Some("text"));
}
