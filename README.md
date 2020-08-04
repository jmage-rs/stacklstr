```
use stacklstr::L;

fn main() {
  let x: [u16; 5] = L!("asdf");
  println!("{:?}", x); // [97, 115, 100, 102, 0]
}
```