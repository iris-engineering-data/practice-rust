fn main() {
  let s = String::from("hello");

  let len = s.len();

  println!("slice is {}", &s[..]);
}