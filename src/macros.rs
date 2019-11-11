#[macro_export]
macro_rules! build_enum {
  ($name:ident {
      $($key:ident = $value:expr), *
  }) => {
    #[repr(u8)]
    #[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
    pub enum $name { $($key = $value,)* }

    impl From<$name> for u8 {
        fn from(t: $name) -> Self {
            match t {
              $($name::$key => $value,)*
            }
        }
    }

    impl From<u8> for $name {
        fn from(v: u8) -> Self {
            match v {
              $($value => $name::$key,)*
              _ => panic!("Unable to convert byte to {}", stringify!($name))
            }
        }
    }
  };
}

mod test {

  build_enum!(Foo { A = 1, B = 2 });

  #[test]
  fn from_u8() {
    let one: Foo = Foo::from(1);
    assert_eq!(one, Foo::A);
    let two: Foo = Foo::from(2);
    assert_eq!(two, Foo::B);
  }

  #[test]
  fn to_u8() {
    let one: u8 = u8::from(Foo::A);
    assert_eq!(1, one);
    let two: u8 = u8::from(Foo::B);
    assert_eq!(2, two);
  }
}
