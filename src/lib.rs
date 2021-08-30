

macro_rules! guid_lookup {
    (
        $([ $os:literal, $name:literal, $guid:literal ])*
    ) => {
        impl PartitionDescription {
            pub const fn from_guid(guid: PartitionTypeGuid) -> Option<Self> {
                match match guid {
                    $(
                        guid if eq(guid, parse_guid($guid)) => Some(Self {
                            os: $os,
                            type_description: $name,
                        }),
                    )*
                    _ => None,
                } {
                    None => match guid.flip_endian() {
                        $(
                            guid if eq(guid, parse_guid($guid)) => Some(Self {
                                os: $os,
                                type_description: $name,
                            }),
                        )*
                        _ => None,
                    },
                    Some(x) => Some(x),
                }
            }
        }
    };
}

const fn eq(x: PartitionTypeGuid, y: PartitionTypeGuid) -> bool {
    x.time_low == y.time_low &&
        x.time_mid == y.time_mid &&
        x.time_hi_and_version == y.time_hi_and_version &&
        x.clock_seq == y.clock_seq &&
        x.node == y.node
}

type U48 = u64;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct PartitionTypeGuid {
    pub time_low: u32,
    pub time_mid: u16,
    pub time_hi_and_version: u16,
    pub clock_seq: u16,
    pub node: U48,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PartitionDescription {
    pub os: &'static str,
    pub type_description: &'static str,
}

impl PartitionTypeGuid {
    pub const fn flip_endian(&self) -> Self {
        Self {
            // flipped
            time_low: u32::from_le(self.time_low.to_be()),
            time_mid: u16::from_le(self.time_mid.to_be()),
            time_hi_and_version: u16::from_le(self.time_hi_and_version.to_be()),

            // not flipped
            clock_seq: self.clock_seq,
            node: self.node,
        }
    }

    pub const fn from_bytes(bytes: [u8; 0x10]) -> Self {
        let [a, b, c, d,   e, f,   g, h,   i, j,   k, l, m, n, o, p] = bytes;

        Self {
            time_low: u32::from_le_bytes([a, b, c, d]),
            time_mid: u16::from_le_bytes([e, f]),
            time_hi_and_version: u16::from_le_bytes([g, h]),
            clock_seq: u16::from_be_bytes([i, j]),
            node: u64::from_be_bytes([0, 0, k, l, m, n, o, p]),
        }
    }

    pub const fn into_bytes(self) -> [u8; 0x10] {
        let [a, b, c, d] = self.time_low.to_le_bytes();
        let [e, f] = self.time_mid.to_le_bytes();
        let [g, h] = self.time_hi_and_version.to_le_bytes();
        let [i, j] = self.clock_seq.to_be_bytes();
        let [_, _, k, l, m, n, o, p] = self.node.to_be_bytes();

        [a, b, c, d,   e, f,   g, h,   i, j,   k, l, m, n, o, p]
    }

    pub const fn description(self) -> Option<PartitionDescription> {
        PartitionDescription::from_guid(self)
    }
}

include!(concat!(env!("OUT_DIR"), "/guid_lookup.rs"));

const UH_OH: [&str; 1] = ["Invalid hex digit"];

const fn from_hex(byte: u8) -> u64 {
    (match byte {
        b'A'..=b'F' => (byte - b'A') + 0xA,
        b'a'..=b'f' => (byte - b'a') + 0xa,
        b'0'..=b'9' => (byte - b'0'),
        byte => UH_OH[byte as usize + 1].as_bytes()[0],
    }) as u64
}

const fn parse_hex_bytes<const BITS: usize>(bytes: &[u8], at: usize) -> u64 {
    let nibble_count = BITS / 4;

    let mut num = 0;
    let mut pos = at;
    let end_pos = at + nibble_count;

    while pos < end_pos {
        num = (num << 4) + from_hex(bytes[pos]);
        pos += 1;
    }

    num
}

pub const fn parse_guid(guid: &str) -> PartitionTypeGuid {
    let bytes = guid.as_bytes();
    PartitionTypeGuid {
        time_low: parse_hex_bytes::<32>(bytes, 0) as _,
        time_mid: parse_hex_bytes::<16>(bytes, 9) as _,
        time_hi_and_version: parse_hex_bytes::<16>(bytes, 14) as _,
        clock_seq: parse_hex_bytes::<16>(bytes, 19) as _,
        node: parse_hex_bytes::<48>(bytes, 24),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_hex() {
        assert_eq!(0x123, parse_hex_bytes::<16>(b"0123", 0));
        assert_eq!(0x23, parse_hex_bytes::<8>(b"0123", 2));
        assert_eq!(
            PartitionTypeGuid {
                time_low: 264650159,
                time_mid: 33923,
                time_hi_and_version: 18290,
                clock_seq: 36473,
                node: 67524809424356
            },
            parse_guid("0FC63DAF-8483-4772-8E79-3D69D8477DE4")
        );
        assert_eq!(
            parse_guid("0FC63DAF-8483-4772-8E79-3D69D8477DE4"),
            PartitionTypeGuid::from_bytes(
                parse_guid("0FC63DAF-8483-4772-8E79-3D69D8477DE4").into_bytes()
            )
        );
        assert_eq!(
            parse_guid("0FC63DAF-8483-4772-8E79-3D69D8477DE4").description().unwrap(),
            PartitionDescription {
                os: "Linux",
                type_description: "Linux filesystem data"
            }
        );
    }
}
