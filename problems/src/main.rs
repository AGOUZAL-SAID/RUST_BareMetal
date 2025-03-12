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
    // Check Deref for both variants of OOR
    let s1 = OOR::Owned(String::from("  Hello, world.  "));
    assert_eq!(s1.trim(), "Hello, world.");
    let mut s2 = OOR::Borrowed("  Hello, world!  ");
    assert_eq!(s2.trim(), "Hello, world!");

    // Check choose
    let s = choose_str(&s1, &s2, true);
    assert_eq!(s.trim(), "Hello, world.");
    let s = choose_str(&s1, &s2, false);
    assert_eq!(s.trim(), "Hello, world!");

    // Check DerefMut, a borrowed string should become owned
    assert!(matches!(s1, OOR::Owned(_)));
    assert!(matches!(s2, OOR::Borrowed(_)));
    unsafe {
        for c in s2.as_bytes_mut() {
            if *c == b'!' {
                *c = b'?';
            }
        }
    }
    assert!(matches!(s2, OOR::Owned(_)));
    assert_eq!(s2.trim(), "Hello, world?");
}
enum OOR<'a> {
    Owned(String),
    Borrowed(&'a str),
}

impl Deref for OOR<'_> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        match self {
            OOR::Owned(word) => word,
            OOR::Borrowed(word) => word,
        }
    }
}

impl DerefMut for OOR<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if let OOR::Borrowed(s) = self {
            *self = OOR::Owned(s.to_string().clone());
        }
        match self {
            OOR::Owned(s) => s,
            _ => unreachable!(),
        }
    }
}
//`word`
