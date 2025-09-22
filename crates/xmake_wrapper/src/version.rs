// xmake v3.0.0+HEAD.0db4fe610, A cross-platform build utility based on Lua
// Copyright (C) 2015-present Ruki Wang, tboox.org, xmake.io
//                          _
//     __  ___ __  __  __ _| | ______
//     \ \/ / |  \/  |/ _  | |/ / __ \
//      >  <  | \__/ | /_| |   <  ___/
//     /_/\_\_|_|  |_|\__ \|_|\_\____|
//                          by ruki, xmake.io
//     👉  Manual: https://xmake.io/#/getting_started
//     🙏  Donate: https://xmake.io/#/sponsor
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmakeVersion {
    /// Major version number
    pub major: u32,
    /// Minor version number
    pub minor: u32,
    /// Patch version number
    pub patch: u32,
    /// Additional version info (e.g., "HEAD.0db4fe610")
    pub extra: Option<String>,
}

impl std::fmt::Display for XmakeVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(extra) = &self.extra {
            write!(f, "{}.{}.{}+{}", self.major, self.minor, self.patch, extra)
        } else {
            write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
        }
    }
}

impl XmakeVersion {
    /// Parses a version string into an XmakeVersion struct
    /// Handles formats like:
    /// - "xmake v3.0.0+HEAD.0db4fe610, A cross-platform build utility based on Lua"
    /// - "3.0.0+HEAD.0db4fe610"
    /// - "3.0.0"
    pub fn parse(version_str: &str) -> Option<Self> {
        // First, extract the version part from the full output
        // Handle "xmake v3.0.0+HEAD.0db4fe610, ..." format
        let version_part = if version_str.starts_with("xmake v") {
            // Extract everything after "xmake v" and before the first comma or end
            let after_prefix = &version_str[7..]; // Skip "xmake v"
            if let Some(comma_pos) = after_prefix.find(',') {
                &after_prefix[..comma_pos]
            } else {
                after_prefix
            }
        } else {
            // Assume the entire string is the version
            version_str
        }
        .trim();

        // Split by '+' to separate main version from extra info
        let parts: Vec<&str> = version_part.split('+').collect();
        let version_parts: Vec<&str> = parts[0].split('.').collect();

        if version_parts.len() < 3 {
            return None;
        }

        let major = version_parts[0].parse().ok()?;
        let minor = version_parts[1].parse().ok()?;
        let patch = version_parts[2].parse().ok()?;

        // Handle extra version info (everything after '+')
        let extra = if parts.len() > 1 {
            // Join all parts after the first '+' in case there are multiple '+'
            Some(parts[1..].join("+"))
        } else {
            None
        };

        Some(Self {
            major,
            minor,
            patch,
            extra,
        })
    }
}

impl Default for XmakeVersion {
    fn default() -> Self {
        Self {
            major: 0,
            minor: 0,
            patch: 0,
            extra: None,
        }
    }
}

// Remove ANSI escape sequences (colors/control) from a string
pub fn strip_ansi_codes(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out = String::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == 0x1B {
            i += 1;
            if i >= bytes.len() {
                break;
            }
            let b = bytes[i];
            if b == b'[' {
                // CSI sequence: ESC [ ... final byte in @..~
                i += 1;
                while i < bytes.len() {
                    let ch = bytes[i];
                    if ch >= 0x40 && ch <= 0x7E {
                        i += 1;
                        break;
                    }
                    i += 1;
                }
            } else if b == b']' {
                // OSC sequence: ESC ] ... terminated by BEL or ST (ESC \)
                i += 1;
                while i < bytes.len() {
                    if bytes[i] == 0x07 {
                        i += 1;
                        break;
                    }
                    if bytes[i] == 0x1B && i + 1 < bytes.len() && bytes[i + 1] == b'\\' {
                        i += 2;
                        break;
                    }
                    i += 1;
                }
            } else {
                // Other escape: skip this one extra byte (e.g., ESC ( B)
                i += 1;
            }
        } else {
            out.push(bytes[i] as char);
            i += 1;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_full_xmake_output() {
        let input = "xmake v3.0.0+HEAD.0db4fe610, A cross-platform build utility based on Lua";
        let version = XmakeVersion::parse(input).unwrap();

        assert_eq!(version.major, 3);
        assert_eq!(version.minor, 0);
        assert_eq!(version.patch, 0);
        assert_eq!(version.extra, Some("HEAD.0db4fe610".to_string()));
        assert_eq!(version.to_string(), "3.0.0+HEAD.0db4fe610");
    }

    #[test]
    fn test_parse_simple_version() {
        let input = "3.0.0";
        let version = XmakeVersion::parse(input).unwrap();

        assert_eq!(version.major, 3);
        assert_eq!(version.minor, 0);
        assert_eq!(version.patch, 0);
        assert_eq!(version.extra, None);
        assert_eq!(version.to_string(), "3.0.0");
    }

    #[test]
    fn test_parse_version_with_extra() {
        let input = "3.0.0+HEAD.0db4fe610";
        let version = XmakeVersion::parse(input).unwrap();

        assert_eq!(version.major, 3);
        assert_eq!(version.minor, 0);
        assert_eq!(version.patch, 0);
        assert_eq!(version.extra, Some("HEAD.0db4fe610".to_string()));
    }

    #[test]
    fn test_parse_invalid_version() {
        assert!(XmakeVersion::parse("invalid").is_none());
        assert!(XmakeVersion::parse("1.2").is_none());
        assert!(XmakeVersion::parse("").is_none());
    }

    #[test]
    fn test_parse_xmake_prefix_without_comma() {
        let input = "xmake v2.5.1+dev.abc123";
        let version = XmakeVersion::parse(input).unwrap();

        assert_eq!(version.major, 2);
        assert_eq!(version.minor, 5);
        assert_eq!(version.patch, 1);
        assert_eq!(version.extra, Some("dev.abc123".to_string()));
    }
}
