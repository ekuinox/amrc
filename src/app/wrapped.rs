macro_rules! wrap_oauth2_url {
    ($wrapped:ident) => {
        #[derive(PartialEq, Eq, Clone, Debug)]
        pub struct $wrapped(oauth2::$wrapped);

        impl std::str::FromStr for $wrapped {
            type Err = oauth2::url::ParseError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let url = oauth2::$wrapped::new(s.to_string())?;
                Ok($wrapped(url))
            }
        }
        impl From<$wrapped> for oauth2::$wrapped {
            fn from($wrapped(url): $wrapped) -> Self {
                url
            }
        }

        impl From<oauth2::$wrapped> for $wrapped {
            fn from(url: oauth2::$wrapped) -> Self {
                $wrapped(url)
            }
        }
    };
}

wrap_oauth2_url!(AuthUrl);
wrap_oauth2_url!(TokenUrl);
wrap_oauth2_url!(RedirectUrl);
