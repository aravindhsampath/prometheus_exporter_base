macro_rules! string_enum {
    ($name:ident, $error:ident, $($lit:ident),*) => {

        #[derive(Debug, Clone)]
        pub struct $error {
            passed_name: String
        }

        impl $error {
            pub fn passed_name(&self) -> &str {
                &self.passed_name
            }

            pub(crate) fn new(passed_name: &str) -> $error {
                $error { passed_name: passed_name.to_owned() }
            }
        }

        impl std::fmt::Display for $error {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    f,
                    "enum $name does not have the {} variant", self.passed_name
               )
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum $name {
            $($lit,)*
        }

        impl std::convert::TryFrom<&str> for $name {
            type Error = $error;

            fn try_from(txt: &str) -> Result<Self, Self::Error> {
                match txt {
                    $(stringify!($lit) => Ok($name::$lit),)*
                    _ => Err($error::new(txt))
                }
            }
        }

        impl std::convert::AsRef<str> for $name {
            fn as_ref(&self) -> &'static str {
                 match self {
                     $($name::$lit => stringify!($lit),)*
                 }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    f,
                    "{}", self.as_ref()
                )
            }
        }
    };
}

string_enum!(
    MetricType,
    UnknownMetricType,
    Counter,
    Gauge,
    Histogram,
    Summary
);

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_as_ref() {
        assert_eq!("Gauge", MetricType::Gauge.as_ref());
        assert_eq!("Counter", MetricType::Counter.as_ref());
        assert_eq!("Gauge", MetricType::Gauge.as_ref());
        assert_eq!("Histogram", MetricType::Histogram.as_ref());
    }

    #[test]
    fn test_display() {
        assert_eq!("Gauge", format!("{}", MetricType::Gauge));
    }

    #[test]
    fn test_try_from_ok() {
        assert_eq!(
            MetricType::Histogram,
            MetricType::try_from("Histogram").unwrap()
        );
    }

}
