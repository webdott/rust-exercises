//! Run this file with `cargo test --test 05_srl_validator`.

// TODO: Implement a SRL (Simple Resource Locator) validator.
// A SRL consists of two parts, an optional protocol (string) and an address (string).
// The format of the SRL looks like this: `[<protocol>://]<address>`
// The protocol and the address have to contain only lowercase English characters.
// Protocol must not be empty if :// is present in the SRL.
// Address must not be empty.
//
// As an example, these are valid SRLs:
// - `http://foo`
// - `bar://baz`
// - `foobar`
//
// And these are invalid SRLs:
// - `http://foo1` (invalid character in address)
// - `asd://bar://` (invalid character in address)
// - `://baz` (empty protocol)
// - `01://baz` (invalid character in protocol)
//
// Create a struct `SRL` in a module named `srl`. Expose functions for parsing a SRL and getting
// its individual parts, but do not allow modifying the fields of `SRL` outside its module.
// Do not use regular expressions, SRLs can be easily parsed with a big of parsing logic.
//
// Hint: Put `#[derive(Debug, Eq, PartialEq)]` on top of `SRL` and `SRLValidationError`,
// so that asserts in tests work.

mod srl {
    use std::error::Error;
    use std::fmt::Display;
    use regex::Regex;

    #[derive(Debug, Eq, PartialEq)]
    pub enum SRLValidationError {
        EmptyProtocol,
        EmptyAddress,
        InvalidCharacterInAddress(char),
        InvalidCharacterInProtocol(char)
    }

    impl Display for SRLValidationError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    impl Error for SRLValidationError {}

    #[derive(Debug, Eq, PartialEq)]
    pub struct SRL {
        address: String,
        protocol: Option<String>,
    }

    impl SRL {
        pub fn new(full_address: &str) -> Result<Self, SRLValidationError> {
            if full_address.is_empty() {
                return Err(SRLValidationError::EmptyAddress);
            }

            let regex = Regex::new(r"(?<protocol>[a-z]*)(?<invalidp>\w*)(?<delimeter>://)*(?<address>[a-z]*)(?<invalida>.*)").unwrap();
            let captures = regex.captures(full_address).unwrap();

            let delimeter = match captures.name("delimeter") {
                Some(delimeter) => delimeter.as_str(),
                None => "",
            };

            let invalid_p_char = match captures.name("invalidp") {
                Some(invalidp) => invalidp.as_str(),
                None => "",
            };

            if !invalid_p_char.is_empty() {
                return if delimeter.is_empty() {
                    Err(SRLValidationError::InvalidCharacterInAddress(invalid_p_char.chars().nth(0).unwrap()))
                } else {
                    Err(SRLValidationError::InvalidCharacterInProtocol(invalid_p_char.chars().nth(0).unwrap()))
                }
            }

            let invalid_a_char = match captures.name("invalida") {
                Some(invalida) => invalida.as_str(),
                None => "",
            };

            if !invalid_a_char.is_empty() {
                return Err(SRLValidationError::InvalidCharacterInAddress(invalid_a_char.chars().nth(0).unwrap()));
            }

            let address = match captures.name("address") {
                Some(addr) => addr.as_str(),
                None => {
                    return Err(SRLValidationError::EmptyAddress);
                }
            };
            let protocol = match captures.name("protocol") {
                Some(protocol) => protocol.as_str(),
                None => {
                    return Err(SRLValidationError::EmptyProtocol)
                }
            };

            match true {
                _v if protocol.is_empty() && address.is_empty() => Err(SRLValidationError::EmptyProtocol),
                _v if !protocol.is_empty() && address.is_empty() && !delimeter.is_empty() => Err(SRLValidationError::EmptyAddress),
                _v if !protocol.is_empty() && address.is_empty() && delimeter.is_empty() => {
                    Ok(Self {
                        address: protocol.to_string(),
                        protocol: None,
                    })
                },
                _v if protocol.is_empty() => Err(SRLValidationError::EmptyProtocol),
                _ => {
                    Ok(Self {
                        address: address.to_string(),
                        protocol: Some(protocol.to_string()),
                    })
                },
            }
        }

        pub fn get_protocol(&self) -> Option<&str> {
            match &self.protocol {
                Some(protocol) => Some(&protocol),
                None => None
            }
        }

        pub fn get_address(&self) -> &str {
            &self.address
        }
    }
}

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use super::srl::{SRLValidationError, SRL};

    #[test]
    fn empty_address() {
        assert_eq!(SRL::new(""), Err(SRLValidationError::EmptyAddress));
    }

    #[test]
    fn only_separator() {
        assert_eq!(SRL::new("://"), Err(SRLValidationError::EmptyProtocol));
    }

    #[test]
    fn empty_protocol() {
        assert_eq!(SRL::new("://foo"), Err(SRLValidationError::EmptyProtocol));
    }

    #[test]
    fn multiple_protocols() {
        assert_eq!(
            SRL::new("ab://bc://foo"),
            Err(SRLValidationError::InvalidCharacterInAddress(':'))
        );
    }

    #[test]
    fn invalid_protocol() {
        assert_eq!(
            SRL::new("bAc://foo"),
            Err(SRLValidationError::InvalidCharacterInProtocol('A'))
        );
        assert_eq!(
            SRL::new("a02://foo"),
            Err(SRLValidationError::InvalidCharacterInProtocol('0'))
        );
    }

    #[test]
    fn invalid_address_with_protocol() {
        assert_eq!(
            SRL::new("abc://fo1o"),
            Err(SRLValidationError::InvalidCharacterInAddress('1'))
        );
        assert_eq!(
            SRL::new("bar://fooBZcX"),
            Err(SRLValidationError::InvalidCharacterInAddress('B'))
        );
    }

    #[test]
    fn invalid_address_without_protocol() {
        assert_eq!(
            SRL::new("fo1o"),
            Err(SRLValidationError::InvalidCharacterInAddress('1'))
        );
        assert_eq!(
            SRL::new("fooBAc"),
            Err(SRLValidationError::InvalidCharacterInAddress('B'))
        );
    }

    #[test]
    fn invalid_protocol_and_address() {
        assert_eq!(
            SRL::new("bZcA://fo2o"),
            Err(SRLValidationError::InvalidCharacterInProtocol('Z'))
        );
        assert_eq!(
            SRL::new("a20://barBAZ"),
            Err(SRLValidationError::InvalidCharacterInProtocol('2'))
        );
    }

    #[test]
    fn invalid_char_emoji() {
        assert_eq!(
            SRL::new("asd://fo🙃o"),
            Err(SRLValidationError::InvalidCharacterInAddress('🙃'))
        );
    }

    #[test]
    fn no_protocol() {
        let srl = SRL::new("foobar").unwrap();
        assert_eq!(srl.get_protocol(), None);
        assert_eq!(srl.get_address(), "foobar");
    }

    #[test]
    fn protocol_and_scheme() {
        let srl = SRL::new("bar://foobar").unwrap();
        assert_eq!(srl.get_protocol(), Some("bar"));
        assert_eq!(srl.get_address(), "foobar");
    }
}