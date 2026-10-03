// Source code for the Substrate Telemetry Server.
// Copyright (C) 2021 Parity Technologies (UK) Ltd.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use anyhow::{anyhow, Error};

#[derive(Copy, Clone, Debug)]
pub struct ByteSize(usize);

impl ByteSize {
    pub fn new(bytes: usize) -> ByteSize {
        ByteSize(bytes)
    }
    /// Return the number of bytes stored within.
    pub fn num_bytes(self) -> usize {
        self.0
    }
}

impl From<ByteSize> for usize {
    fn from(b: ByteSize) -> Self {
        b.0
    }
}

impl std::str::FromStr for ByteSize {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let idx = s.find(|c| !char::is_ascii_digit(&c)).unwrap_or(s.len());
        // Empty (no digits at all) or too large for usize: a usage error, not a panic.
        let n: usize = s[..idx]
            .parse()
            .map_err(|e| anyhow!("Cannot parse '{}' into bytes: {}", s, e))?;
        let suffix = s[idx..].trim();
        let multiplier: usize = match suffix {
            "" | "B" | "b" => 1,
            "kB" | "K" | "k" => 1000,
            "MB" | "M" | "m" => 1000 * 1000,
            "GB" | "G" | "g" => 1000 * 1000 * 1000,
            "KiB" | "Ki" => 1024,
            "MiB" | "Mi" => 1024 * 1024,
            "GiB" | "Gi" => 1024 * 1024 * 1024,
            _ => {
                return Err(anyhow!(
                    "\
                Cannot parse into bytes; suffix is '{}', but expecting one of \
                B,b, kB,K,k, MB,M,m, GB,G,g, KiB,Ki, MiB,Mi, GiB,Gi",
                    suffix
                ))
            }
        };
        n.checked_mul(multiplier)
            .map(ByteSize)
            .ok_or_else(|| anyhow!("Cannot parse '{}' into bytes: too large", s))
    }
}

#[cfg(test)]
mod test {
    use crate::byte_size::ByteSize;

    #[test]
    fn can_parse_valid_strings() {
        let cases = vec![
            ("100", 100),
            ("100B", 100),
            ("100b", 100),
            ("20kB", 20 * 1000),
            ("20 kB", 20 * 1000),
            ("20K", 20 * 1000),
            (" 20k", 20 * 1000),
            ("1MB", 1 * 1000 * 1000),
            ("1M", 1 * 1000 * 1000),
            ("1m", 1 * 1000 * 1000),
            ("1 m", 1 * 1000 * 1000),
            ("1GB", 1 * 1000 * 1000 * 1000),
            ("1G", 1 * 1000 * 1000 * 1000),
            ("1g", 1 * 1000 * 1000 * 1000),
            ("1KiB", 1 * 1024),
            ("1Ki", 1 * 1024),
            ("1MiB", 1 * 1024 * 1024),
            ("1Mi", 1 * 1024 * 1024),
            ("1GiB", 1 * 1024 * 1024 * 1024),
            ("1Gi", 1 * 1024 * 1024 * 1024),
            (" 1 Gi ", 1 * 1024 * 1024 * 1024),
        ];

        for (s, expected) in cases {
            let b: ByteSize = s.parse().unwrap();
            assert_eq!(b.num_bytes(), expected);
        }
    }

    #[test]
    fn rejects_invalid_strings_instead_of_panicking() {
        // A typo in --max-node-data-per-second used to abort the shard with a panic
        // ("all ascii digits") instead of a usage error.
        let cases = vec![
            "",
            "abc",
            "k",
            " MB",
            "1.5MB",
            "99999999999999999999",
            "18446744073709551615GiB",
        ];

        for s in cases {
            assert!(s.parse::<ByteSize>().is_err(), "{s:?} should not parse");
        }
    }
}
