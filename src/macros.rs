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

    impl From<u8> for $name {
        fn from(v: u8) -> Self {
            match v {
              $($value => $name::$key,)*
              _ => panic!("Unable to convert byte to {}", stringify!($name))
            }
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
            concat!("assert_eq!(", stringify!($name), "::from(", stringify!($value), "), id);"),
          ]
          $key = $value),*
      })
    );
  };
}
