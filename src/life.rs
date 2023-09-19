pub fn test() {

}

pub fn life1<'c>(a: &'c str, b: &'c str) -> &'c str {
  if a.len() == b.len() {
    return a;
  }
  b

}

pub fn life2(a: usize, b: usize) -> usize {
  let mut c = a;
  if a > b {
    c = b;
  }
  c
}

pub fn life3(a: String, b: String) -> String {
  if a.len() > b.len() {
    return a;
  }
  b
}