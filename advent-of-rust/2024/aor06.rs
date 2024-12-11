// Write a function that returns the reference to the longer string
// without any new allocations
pub fn longer_wish<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str> {
    let s1 = s1.trim();
    // Apparently, AoR wants Unicode codepoints and not bytes.
    let s1_len = s1.chars().count();
    let s2 = s2.trim();
    let s2_len = s2.chars().count();
    use std::cmp::Ordering;
    match s1_len.cmp(&s2_len) {
        Ordering::Greater => Some(s1),
        Ordering::Less => Some(s2),
        Ordering::Equal => None,
    }
}
