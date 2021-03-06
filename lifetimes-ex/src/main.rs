use std::vec;

// TODO: returns a reference to the value inserted in the vector.
fn copy_and_return<'a>(vector: &'a mut Vec<String>, value: &str) -> &'a String {
    vector.push(String::from(value));

    // get the value insertd to the vector
    // get() -> returns reference of an element or None
    // unwrap -> get the value from Option::Some(String)
    vector.get(vector.len() - 1).unwrap()
}

fn main() {
    let name1 = "Joe";
    let name2 = "Chris";
    let name3 = "Anne";

    let mut names = Vec::new();

    assert_eq!("Joe", copy_and_return(&mut names, &name1));
    assert_eq!("Chris", copy_and_return(&mut names, &name2));
    assert_eq!("Anne", copy_and_return(&mut names, &name3));

    assert_eq!(
        names,
        vec!["Joe".to_string(), "Chris".to_string(), "Anne".to_string()]
    )
}
