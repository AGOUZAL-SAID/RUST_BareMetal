use std::ops::Deref;
use std::ops::DerefMut;
fn ret_string() -> String {
    String::from("  A String object  ")
}

fn choose_str(s1: & str, s2: & str, select_s1: bool) -> &str {
    let a = s1 ;
    let b = s2 ;
    if select_s1 { a } else { b }
}
/// the error befor was because we use the value returned
/// whiche the life time is immidiate so i give it to an vriable
/// that will survive until the end of main
fn main() {
    let s = ret_string(); 
    let word = s.trim(); 
    assert_eq!(word, "A String object");
}
enum OOR <'a> {
    Owned(String),
    Borrowed(&'a str), // `'static` is used for this example; you can customize lifetimes
}

impl<'a> Deref for OOR<'a>  {
    type Target = str;
    fn deref(&self) -> &Self::Target {

        match self {
            OOR::Owned(word) => &word,               // Deref to the `String` stored in the `Owned` variant
            OOR::Borrowed(word) => *word,            // Deref to the `&str` stored in the `Borrowed` variant
        }
    }


}

impl<'a> DerefMut OOR<'a>{
    fn deref_mut(&mut self) -> &mut Self::Target{
        let mut a;
        let mut b;
        match self {
            OOR:: Owned(word)=> {
                a = word;
                return & mut a},
            OOR::Borrowed(word)=> {
                b = *word;
                return & mut b;
            }
        }}}