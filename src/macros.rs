/// Generates a public enum with the following traits implemented:
/// * `From<T> for u8`
/// * `TryFrom<u8> for T`
#[macro_export]
macro_rules! build_enum {
  ($name:ident {
      $($key:ident = $value:expr),*
  }) => {
    #[repr(u8)]
    #[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
    pub enum $name {
      $($key = $value,)*
    }

    impl From<$name> for u8 {
        fn from(t: $name) -> Self {
            match t {
                $($name::$key => $value),*
            }
        }
    }

    impl TryFrom<u8> for $name {
        type Error = crate::Error;
        fn try_from(v: u8) -> Result<Self, crate::Error> {
            return match v {
              $($value => Ok($name::$key),)*
              _ => Err(crate::Error::GenerateError)
            };
        }
    }

    #[cfg(test)]
    mod enum_tests {
      use super::$name;
      use std::convert::TryFrom;
      $(
        #[test]
        #[allow(non_snake_case)]
        fn $key() {
          let id = $name::$key;
          assert_eq!(u8::from(id), $value);
          assert_eq!($name::try_from($value).unwrap(), id);
        }
      )*
    }
  };
}
