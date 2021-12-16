use std::{convert::TryFrom, num::ParseIntError, str::FromStr};

use anyhow::{anyhow, bail, Result};
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::{all_consuming, map_res},
    multi::{fold_many0, fold_many1, many1, many_m_n},
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum OpCode {
    Sum,
    Product,
    Minimum,
    Maximum,
    Literal,
    Greater,
    Less,
    Equal,
}

impl TryFrom<usize> for OpCode {
    type Error = anyhow::Error;

    fn try_from(value: usize) -> Result<Self> {
        Ok(match value {
            0 => Self::Sum,
            1 => Self::Product,
            2 => Self::Minimum,
            3 => Self::Maximum,
            4 => Self::Literal,
            5 => Self::Greater,
            6 => Self::Less,
            7 => Self::Equal,
            _ => bail!("Invalid opcode: {}", value),
        })
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Length {
    Bits(usize),
    Packets(usize),
}

impl Length {
    pub fn sub_packets<'a>(&self, input: &'a str) -> IResult<&'a str, Vec<Packet>> {
        match self {
            Length::Bits(bits) => {
                let (input, sub_bits) = take(*bits)(input)?;
                let (_, packets) =
                    all_consuming(fold_many1(packet, Vec::new, |mut acc: Vec<_>, item| {
                        acc.push(item);
                        acc
                    }))(sub_bits)?;
                Ok((input, packets))
            }
            Length::Packets(num) => many_m_n(*num, *num, packet)(input),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PacketType {
    Literal(usize),
    Operator {
        code: OpCode,
        len: Length,
        packets: Vec<Packet>,
    },
}

impl PacketType {
    pub fn value(&self) -> usize {
        match self {
            PacketType::Literal(v) => *v,
            PacketType::Operator { code, packets, .. } => match code {
                OpCode::Sum => packets.iter().fold(0, |acc, p| acc + p.value()),
                OpCode::Product => packets.iter().fold(1, |acc, p| acc * p.value()),
                OpCode::Minimum => packets.iter().map(|p| p.value()).min().unwrap_or(0),
                OpCode::Maximum => packets.iter().map(|p| p.value()).max().unwrap_or(0),
                OpCode::Greater => {
                    if packets[0].value() > packets[1].value() {
                        1
                    } else {
                        0
                    }
                }
                OpCode::Less => {
                    if packets[0].value() < packets[1].value() {
                        1
                    } else {
                        0
                    }
                }
                OpCode::Equal => {
                    if packets[0].value() == packets[1].value() {
                        1
                    } else {
                        0
                    }
                }
                _ => {
                    unreachable!("this should not be possible unless this is manually constructed")
                }
            },
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Packet {
    version: usize,
    type_id: PacketType,
}

impl Packet {
    pub fn new(version: usize, type_id: PacketType) -> Self {
        Self { version, type_id }
    }

    pub fn value(&self) -> usize {
        self.type_id.value()
    }

    pub fn version_sum(&self) -> usize {
        let mut sum = self.version;
        if let PacketType::Operator { ref packets, .. } = self.type_id {
            sum += packets.iter().fold(0, |acc, p| acc + p.version_sum());
        }
        sum
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Transmission {
    packets: Vec<Packet>,
}

impl Transmission {
    pub fn version_sum(&self) -> usize {
        self.packets.iter().fold(0, |acc, p| acc + p.version_sum())
    }

    pub fn value(&self) -> usize {
        self.packets.iter().fold(0, |acc, p| acc + p.value())
    }
}

impl FromStr for Transmission {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self> {
        // convert all the hex digits to a string of bits.
        // so, yeah. I realize that I should just operate on a byte array, but
        // this just seemed easier given the time contstraint
        let s = input
            .chars()
            .map(|ch| {
                ch.to_digit(16)
                    .map(|d| format!("{:04b}", d))
                    .ok_or_else(|| anyhow!("Invalid characters in input"))
            })
            .collect::<Result<Vec<String>>>()?
            .join("");

        // we have to do this because of the lifetime on the value from the parser
        let (_, packets) = (many1(packet)(&s)).map_err(|_| anyhow!("Failed to parse input"))?;
        Ok(Self { packets })
    }
}

// So let's take this opportunity to play around with nom a bit
// Parsers below

// Used for converting string of binary characters to usize
fn from_bin(input: &str) -> Result<usize, ParseIntError> {
    usize::from_str_radix(input, 2)
}

// extract a version u8 from the input
fn version(input: &str) -> IResult<&str, usize> {
    map_res(take(3_usize), from_bin)(input)
}

// Length type 0 has 15 bits specifying a number
fn length_bits(input: &str) -> IResult<&str, Length> {
    let (input, v) = map_res(preceded(tag("0"), take(15_usize)), from_bin)(input)?;

    Ok((input, Length::Bits(v)))
}

// Length type 1 has 11 bits specifying a number
fn length_packets(input: &str) -> IResult<&str, Length> {
    let (input, v) = map_res(preceded(tag("1"), take(11_usize)), from_bin)(input)?;

    Ok((input, Length::Packets(v)))
}

// extract the Length value for an operator
fn operator_length(input: &str) -> IResult<&str, Length> {
    alt((length_bits, length_packets))(input)
}

// extract a PacketType from the input
fn packet_type(input: &str) -> IResult<&str, PacketType> {
    let (input, code) = map_res(map_res(take(3_usize), from_bin), OpCode::try_from)(input)?;

    match code {
        OpCode::Literal => {
            let (input, val) = literal_value(input)?;
            Ok((input, PacketType::Literal(val)))
        }
        x => {
            // if we're not 4, we need to parse out the proper operator
            let (input, len) = operator_length(input)?;
            let (input, packets) = len.sub_packets(input)?;
            Ok((
                input,
                PacketType::Operator {
                    code: x,
                    len,
                    packets,
                },
            ))
        }
    }
}

// extract a Packet the input
fn packet(input: &str) -> IResult<&str, Packet> {
    let (input, (version, packet_type)) = tuple((version, packet_type))(input)?;
    Ok((input, Packet::new(version, packet_type)))
}

fn literal_group(input: &str) -> IResult<&str, usize> {
    map_res(preceded(tag("1"), take(4_usize)), from_bin)(input)
}

fn literal_end_group(input: &str) -> IResult<&str, usize> {
    map_res(preceded(tag("0"), take(4_usize)), from_bin)(input)
}

fn literal_value(input: &str) -> IResult<&str, usize> {
    let (input, (groups, end)) = tuple((
        fold_many0(literal_group, || 0_usize, |acc, item| (acc << 4) + item),
        literal_end_group,
    ))(input)?;

    Ok((input, (groups << 4) + end))
}

#[cfg(test)]
mod tests {
    mod transmission {
        use super::super::*;

        #[test]
        fn sum_version_numbers() {
            let input = "8A004A801A8002F478";
            let t = Transmission::from_str(input).expect("Could not make transmission");
            assert_eq!(t.version_sum(), 16);

            let input = "620080001611562C8802118E34";
            let t = Transmission::from_str(input).expect("Could not make transmission");
            assert_eq!(t.version_sum(), 12);

            let input = "C0015000016115A2E0802F182340";
            let t = Transmission::from_str(input).expect("Could not make transmission");
            assert_eq!(t.version_sum(), 23);

            let input = "A0016C880162017C3686B18A3D4780";
            let t = Transmission::from_str(input).expect("Could not make transmission");
            assert_eq!(t.version_sum(), 31);
        }

        #[test]
        fn value() {
            let input = "C200B40A82";
            let t = Transmission::from_str(input).expect("Could not make transmission");
            assert_eq!(t.value(), 3);

            let input = "04005AC33890";
            let t = Transmission::from_str(input).expect("Could not make transmission");
            assert_eq!(t.value(), 54);

            let input = "880086C3E88112";
            let t = Transmission::from_str(input).expect("Could not make transmission");
            assert_eq!(t.value(), 7);

            let input = "CE00C43D881120";
            let t = Transmission::from_str(input).expect("Could not make transmission");
            assert_eq!(t.value(), 9);

            let input = "D8005AC2A8F0";
            let t = Transmission::from_str(input).expect("Could not make transmission");
            assert_eq!(t.value(), 1);

            let input = "F600BC2D8F";
            let t = Transmission::from_str(input).expect("Could not make transmission");
            assert_eq!(t.value(), 0);

            let input = "9C005AC2F8F0";
            let t = Transmission::from_str(input).expect("Could not make transmission");
            assert_eq!(t.value(), 0);

            let input = "9C0141080250320F1802104A08";
            let t = Transmission::from_str(input).expect("Could not make transmission");
            assert_eq!(t.value(), 1);
        }
    }

    mod parsers {
        use super::super::*;

        #[test]
        fn parse_version() {
            let input = "100";
            assert_eq!(version(input).unwrap(), ("", 4));

            let input = "1011";
            assert_eq!(version(input).unwrap(), ("1", 5));

            let input = "10";
            assert!(version(input).is_err());

            let input = "1A0";
            assert!(version(input).is_err());
        }

        #[test]
        fn parse_packet_type() {
            let input = "110000000000001101111010001010010100100010010010011";
            let expected = PacketType::Operator {
                code: OpCode::Less,
                len: Length::Bits(27),
                packets: vec![
                    Packet::new(6, PacketType::Literal(10)),
                    Packet::new(2, PacketType::Literal(20)),
                ],
            };
            assert_eq!(packet_type(input).unwrap(), ("10011", expected));

            let input = "01110000000001101010000001100100000100011000001110011";
            let expected = PacketType::Operator {
                code: OpCode::Maximum,
                len: Length::Packets(3),
                packets: vec![
                    Packet::new(2, PacketType::Literal(1)),
                    Packet::new(4, PacketType::Literal(2)),
                    Packet::new(1, PacketType::Literal(3)),
                ],
            };
            assert_eq!(packet_type(input).unwrap(), ("10011", expected));

            let input = "100101111111000101000";
            assert_eq!(
                packet_type(input).unwrap(),
                ("000", PacketType::Literal(2021))
            );

            let input = "10";
            assert!(packet_type(input).is_err());

            let input = "1A0";
            assert!(packet_type(input).is_err());
        }

        #[test]
        fn parse_packet() {
            let input = "11101110000000001101010000001100100000100011000001110011";
            let expected = Packet::new(
                7,
                PacketType::Operator {
                    code: OpCode::Maximum,
                    len: Length::Packets(3),
                    packets: vec![
                        Packet::new(2, PacketType::Literal(1)),
                        Packet::new(4, PacketType::Literal(2)),
                        Packet::new(1, PacketType::Literal(3)),
                    ],
                },
            );
            assert_eq!(packet(input).unwrap(), ("10011", expected));

            let input = "110100101111111000101000";
            let expected = Packet::new(6, PacketType::Literal(2021));
            assert_eq!(packet(input).unwrap(), ("000", expected));

            let input = "11111";
            assert!(packet(input).is_err());
        }

        #[test]
        fn parse_literal_value() {
            let input = "10111111100010111000";
            assert_eq!(literal_value(input).unwrap(), ("11000", 2021));

            let input = "0011111000";
            assert_eq!(literal_value(input).unwrap(), ("11000", 7));

            // missing end group
            let input = "1011111000";
            assert!(literal_value(input).is_err());
        }
    }
}
