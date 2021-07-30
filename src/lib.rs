use std::fmt::{Display, Formatter};

use bitvec::order::Msb0;
use bitvec::vec::BitVec;
use nom::{Finish, IResult};
use nom::bits::complete::tag as bit_tag;
use nom::branch::alt;
use nom::bytes::complete::tag as byte_tag;
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::preceded;

use crate::error::{ByteStyleError, ByteStyleResult};

pub mod error;

const BYTE_STYLE_A: &str = "追求极致";
const BYTE_STYLE_B: &str = "务实敢为";
const BYTE_STYLE_C: &str = "开放谦逊";
const BYTE_STYLE_D: &str = "始终创业";
const BYTE_STYLE_E: &str = "多元兼容";

pub fn encode_to_byte_style(input: &str) -> ByteStyleResult<String> {
    Ok(Words::parse_from_bits((input.as_bytes(), 0)).finish().map_err(
        |e| ByteStyleError::Parse(format!("unable to parse from bit: {:?}", e))
    )?.1.to_string())
}

pub fn decode_from_byte_style(input: &str) -> ByteStyleResult<String> {
    let decoded_bytes = Words::parse_from_byte_style(input).finish().map_err(
        |e| ByteStyleError::Parse(format!("unable to parse from byte style: {:?}", e))
    )?.1.to_bytes();
    let decoded = String::from_utf8(decoded_bytes)?;
    Ok(decoded)
}

#[derive(Debug)]
struct Word(&'static str);

impl Word {
    fn parse_from_bits(input: (&[u8], usize)) -> IResult<(&[u8], usize), Self> {
        alt((
            map(bit_tag(0b000_u8, 3_u8), |_| Self(BYTE_STYLE_A)),
            map(bit_tag(0b001_u8, 3_u8), |_| Self(BYTE_STYLE_B)),
            map(bit_tag(0b00_u8, 2_u8), |_| Self(BYTE_STYLE_A)),
            map(bit_tag(0b01_u8, 2_u8), |_| Self(BYTE_STYLE_C)),
            map(bit_tag(0b10_u8, 2_u8), |_| Self(BYTE_STYLE_D)),
            map(bit_tag(0b11_u8, 2_u8), |_| Self(BYTE_STYLE_E)),
            map(bit_tag(0b0_u8, 1_u8), |_| Self(BYTE_STYLE_C)),
            map(bit_tag(0b1_u8, 1_u8), |_| Self(BYTE_STYLE_D)),
        ))(input)
    }

    fn parse_from_byte_style(input: &str) -> IResult<&str, Self> {
        alt((
            map(byte_tag(BYTE_STYLE_A), |_| Self(BYTE_STYLE_A)),
            map(byte_tag(BYTE_STYLE_B), |_| Self(BYTE_STYLE_B)),
            map(byte_tag(BYTE_STYLE_C), |_| Self(BYTE_STYLE_C)),
            map(byte_tag(BYTE_STYLE_D), |_| Self(BYTE_STYLE_D)),
            map(byte_tag(BYTE_STYLE_E), |_| Self(BYTE_STYLE_E)),
        ))(input)
    }
}

impl ToString for Word {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Debug)]
struct Words(pub Vec<Word>);

impl Words {
    fn parse_from_bits(input: (&[u8], usize)) -> IResult<(&[u8], usize), Self> {
        map(many0(Word::parse_from_bits), Self)(input)
    }

    fn parse_from_byte_style(input: &str) -> IResult<&str, Self> {
        map(many0(preceded(multispace0, Word::parse_from_byte_style)), Self)(input)
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bit_vec: BitVec<Msb0, u8> = BitVec::with_capacity(self.0.len() * 3);
        self.0.iter().for_each(|w| match w.0 {
            BYTE_STYLE_A => bit_vec.extend([false, false, false]),
            BYTE_STYLE_B => bit_vec.extend([false, false, true]),
            BYTE_STYLE_C => bit_vec.extend([false, true]),
            BYTE_STYLE_D => bit_vec.extend([true, false]),
            BYTE_STYLE_E => bit_vec.extend([true, true]),
            _ => { unreachable!() }
        });
        for _ in 0..(bit_vec.len() % 8) {
            bit_vec.pop();
        }
        bit_vec.into_vec()
    }
}

impl Display for Words {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let vec = self.0.iter().map(|word| word.to_string()).collect::<Vec<_>>();
        write!(f, "{}", vec.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use nom::Finish;

    use super::*;

    #[test]
    fn test_encode() {
        let bytes = "已删除".as_bytes();
        let encoded = Words::parse_from_bits((bytes, 0)).finish().unwrap().1.to_string();
        assert_eq!(encoded, "多元兼容 始终创业 开放谦逊 开放谦逊 始终创业 多元兼容 开放谦逊 多元兼容 始终创业 多元兼容 务实敢为 开放谦逊 多元兼容 务实敢为 开放谦逊 始终创业 务实敢为 追求极致 始终创业 始终创业 追求极致 开放谦逊 多元兼容 开放谦逊 务实敢为 始终创业 开放谦逊 始终创业 开放谦逊 始终创业 始终创业 开放谦逊 追求极致");
    }

    #[test]
    fn test_decode() {
        println!("{:?}", "已删除".as_bytes());
        let words = "多元兼容 始终创业 开放谦逊 开放谦逊 始终创业 多元兼容 开放谦逊 多元兼容 始终创业 多元兼容 务实敢为 开放谦逊 多元兼容 务实敢为 开放谦逊 始终创业 务实敢为 追求极致 始终创业 始终创业 追求极致 开放谦逊 多元兼容 开放谦逊 务实敢为 始终创业 开放谦逊 始终创业 开放谦逊 始终创业 始终创业 开放谦逊 追求极致";
        let decoded_bytes = Words::parse_from_byte_style(words).finish().unwrap().1.to_bytes();
        let decoded = String::from_utf8(decoded_bytes).unwrap();
        assert_eq!(decoded, "已删除");
    }
}
