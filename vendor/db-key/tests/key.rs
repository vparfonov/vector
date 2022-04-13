extern crate "db-key" as key;

use key::Key;
use key::from_u8;

enum MyValues {
  One
}

struct MyKey {
  #[allow(dead_code)]
  val: MyValues
}

impl Key for MyKey {
  fn from_u8(_key: &[u8]) -> MyKey {
    MyKey { val: MyValues::One }
  }

  fn as_slice<T, F:Fn(&[u8]) -> T>(&self, f: F) -> T {
    f("test".as_bytes())
  }
}

#[test]
fn test() {
  from_u8::<MyKey>("test".as_bytes());
}

#[test]
fn test2() {
  let key = MyKey { val: MyValues::One };
  key.as_slice(|k| {
    assert_eq!(k, "test".as_bytes())
  })
}
