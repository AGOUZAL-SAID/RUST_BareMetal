use std::ops::Deref;
use std::ops::DerefMut;
fn ret_string() -> String {
    String::from("  A String object  ")
}

fn choose_str<'c, 'a: 'c, 'b: 'c>(s1: &'a str, s2: &'b str, select_s1: bool) -> &'c str {
    if select_s1 { s1 } else { s2 }
}
/// the error befor was because we use the value returned
/// whiche the life time is immidiate so i give it to an vriable
/// that will survive until the end of main
fn main() {
    let s = ret_string();
    let word = s.trim();
    let a: &str = "hello";
    println!("{}", a);
    assert_eq!(word, "A String object");
    choose_str(a, word, false);
    // Check Deref for both variants of Oor
    let s1 = Oor::Owned(String::from("  Hello, world.  "));
    assert_eq!(s1.trim(), "Hello, world.");
    let mut s2 = Oor::Borrowed("  Hello, world!  ");
    assert_eq!(s2.trim(), "Hello, world!");

    // Check choose
    let s = choose_str(&s1, &s2, true);
    assert_eq!(s.trim(), "Hello, world.");
    let s = choose_str(&s1, &s2, false);
    assert_eq!(s.trim(), "Hello, world!");

    // Check DerefMut, a borrowed string should become owned
    assert!(matches!(s1, Oor::Owned(_)));
    assert!(matches!(s2, Oor::Borrowed(_)));
    unsafe {
        for c in s2.as_bytes_mut() {
            if *c == b'!' {
                *c = b'?';
            }
        }
    }
    assert!(matches!(s2, Oor::Owned(_)));
    assert_eq!(s2.trim(), "Hello, world?");
}
// Define an enum that can either own a `String` or borrow a `&str`.
enum Oor<'a> {
    Owned(String),     // Owns the string data
    Borrowed(&'a str), // Borrows the string data
}

// Implement the `Deref` trait so that `Oor` can be treated as a `&str`.
impl Deref for Oor<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            // If owned, return a reference to the owned string
            Oor::Owned(word) => word,
            // If borrowed, return the borrowed string reference
            Oor::Borrowed(word) => word,
        }
    }
}

// Implement the `DerefMut` trait to allow mutable access to the inner string.
// If the variant is `Borrowed`, it will be upgraded to an `Owned` string.
impl DerefMut for Oor<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // If it's borrowed, convert it into an owned `String`
        if let Oor::Borrowed(s) = self {
            *self = Oor::Owned(s.to_string().clone());
        }

        // After conversion (or if already owned), return mutable reference to the string
        match self {
            Oor::Owned(s) => s,
            _ => unreachable!(), // Should never happen since Borrowed was converted
        }
    }
}
