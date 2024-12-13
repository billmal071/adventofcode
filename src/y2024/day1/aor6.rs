// Write a function that returns the reference to the longer string
// without any new allocations
pub fn longer_wish<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str> {
  // Your code here
  if s1.trim().chars().count() > s2.trim().chars().count() {
    return Some(&s1);
  } else if s1.trim().chars().count() < s2.trim().chars().count() {
    return Some(&s2);
  } else {
    None
  }
}
