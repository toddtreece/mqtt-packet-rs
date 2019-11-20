#[macro_export]
macro_rules! build_enum {
  (@accum (1, $name:ident { $([$($doc:expr,)*] $key:ident = $value:expr),* }))
    => {
    #[repr(u8)]
    #[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
    pub enum $name {
      $(
        /// # Examples
        /// ```rust
        /// use std::convert::TryFrom;
        $(#[doc=$doc])*
        /// ```
        $key = $value,
      )*
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
  };

  ($name:ident {
      $($key:ident = $value:expr),*
  }) => {
    build_enum!(@accum (1, $name {
        $(
          [
            concat!("use mqtt_packet::", stringify!($name), ";"),
            concat!("let id = ", stringify!($name), "::", stringify!($key), ";"),
            concat!("assert_eq!(u8::from(id), ", stringify!($value), ");"),
            concat!("assert_eq!(", stringify!($name), "::try_from(", stringify!($value), ").unwrap(), id);"),
          ]
          $key = $value),*
      })
    );
  };
}
