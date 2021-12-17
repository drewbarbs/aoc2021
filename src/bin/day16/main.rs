use std::error::Error;

#[derive(PartialEq, Debug)]
enum Expr {
    Literal(u64),
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Minimum(Vec<Packet>),
    Maximum(Vec<Packet>),
    GreaterThan(Vec<Packet>),
    LessThan(Vec<Packet>),
    EqualTo(Vec<Packet>),
}

#[derive(PartialEq, Debug)]
struct Packet {
    version: u32,
    value: Expr,
}

struct BitIterator {
    bytes: Vec<u8>,
    bit_idx: usize,
}

impl BitIterator {
    fn new(bytes: Vec<u8>) -> BitIterator {
        BitIterator { bytes, bit_idx: 0 }
    }
}

impl Iterator for BitIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let byte_idx = self.bit_idx / 8;
        if byte_idx >= self.bytes.len() {
            return None;
        }
        let bit_offset = (7 - (self.bit_idx % 8)) as u32;
        self.bit_idx += 1;
        Some(self.bytes[byte_idx].checked_shr(bit_offset).unwrap() & 0x1)
    }
}

fn parse_to_bytes(input: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    if input.len() % 2 == 1 {
        Err("Invalid input length")?;
    }

    (0..input.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&input[i..i + 2], 16))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.into())
}

fn read_int<I>(bits: &mut I, n: usize) -> Result<u64, Box<dyn Error>>
where
    I: Iterator<Item = u8>,
{
    let mut n_taken = 0;
    let mut val = 0u64;
    while n_taken < n {
        if let Some(b) = bits.next() {
            val = (val << 1) | (b as u64);
            n_taken += 1;
        } else {
            Err(String::from("Not enough bits to read int"))?;
        }
    }
    Ok(val.into())
}

fn parse_packet<I>(bits: &mut I) -> Result<Packet, Box<dyn Error>>
where
    I: Iterator<Item = u8>,
{
    let version = read_int(bits, 3)? as u32;
    let typ = read_int(bits, 3)? as u32;

    match typ {
        4 => {
            let mut val = 0u64;
            loop {
                let next = read_int(bits, 5)?;
                val = (val << 4) | (next & 0xf);
                if next & 0x10 == 0 {
                    break;
                }
            }
            Ok(Packet {
                version,
                value: Expr::Literal(val),
            })
        }
        0 | 1 | 2 | 3 | 5 | 6 | 7 => {
            // Compound expression
            let length_type_id = bits.next();
            if length_type_id.is_none() {
                Err(String::from("Missing length type id"))?;
            }

            let mut sub_packets: Vec<Packet> = Vec::new();
            match length_type_id.unwrap() {
                0 => {
                    let n_bits = read_int(bits, 15)?;
                    let mut sub_packet_bits = bits
                        .by_ref()
                        .take(n_bits as usize)
                        .collect::<Vec<_>>()
                        .into_iter();
                    loop {
                        let packet = parse_packet(&mut sub_packet_bits);
                        if packet.is_err() {
                            break;
                        }

                        sub_packets.push(packet.unwrap());
                    }
                }
                1 => {
                    let n_packets = read_int(bits, 11)?;
                    sub_packets = (0..n_packets)
                        .map(|_| parse_packet(bits))
                        .collect::<Result<Vec<Packet>, _>>()?;
                    assert_eq!(n_packets as usize, sub_packets.len());
                }
                _ => panic!("Unexpected bit from bit iterator"),
            }

            let expr = match typ {
                0 => Expr::Sum(sub_packets),
                1 => Expr::Product(sub_packets),
                2 => Expr::Minimum(sub_packets),
                3 => Expr::Maximum(sub_packets),
                5 => {
                    assert_eq!(sub_packets.len(), 2);
                    Expr::GreaterThan(sub_packets)
                }
                6 => {
                    assert_eq!(sub_packets.len(), 2);
                    Expr::LessThan(sub_packets)
                }
                7 => {
                    assert_eq!(sub_packets.len(), 2);
                    Expr::EqualTo(sub_packets)
                }
                _ => panic!("Invalid packet type"),
            };

            Ok(Packet {
                version,
                value: expr,
            })
        }
        _ => panic!("Invalid packet type"),
    }
}

fn version_sum(packet: &Packet) -> u64 {
    match &packet.value {
        Expr::Literal(_) => packet.version.into(),
        Expr::Sum(pkts)
        | Expr::Product(pkts)
        | Expr::Minimum(pkts)
        | Expr::Maximum(pkts)
        | Expr::GreaterThan(pkts)
        | Expr::LessThan(pkts)
        | Expr::EqualTo(pkts) => {
            let subsum: u64 = pkts.iter().map(|p| version_sum(p)).sum();
            (packet.version as u64) + subsum
        }
    }
}

fn eval_packet(pkt: &Packet) -> u64 {
    match &pkt.value {
        Expr::Literal(v) => *v as u64,
        Expr::Sum(pkts) => pkts.iter().map(|p| eval_packet(p)).sum(),
        Expr::Product(pkts) => pkts
            .iter()
            .map(|p| eval_packet(p))
            .fold(1u64, |acc, v| acc * v),
        Expr::Minimum(pkts) => pkts.iter().map(|p| eval_packet(p)).min().unwrap(),
        Expr::Maximum(pkts) => pkts.iter().map(|p| eval_packet(p)).max().unwrap(),
        Expr::GreaterThan(pkts) => (eval_packet(&pkts[0]) > eval_packet(&pkts[1])).into(),

        Expr::LessThan(pkts) => (eval_packet(&pkts[0]) < eval_packet(&pkts[1])).into(),

        Expr::EqualTo(pkts) => (eval_packet(&pkts[0]) == eval_packet(&pkts[1])).into(),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = aoc2021::get_input_string()?;
    let mut bit_iter = BitIterator::new(parse_to_bytes(input.trim())?);
    let packet = parse_packet(&mut bit_iter)?;

    println!("Part 1: {}", version_sum(&packet));
    println!("Part 2: {}", eval_packet(&packet));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_to_bytes() {
        let result = parse_to_bytes("EE00D40C823060").unwrap();
        assert_eq!(
            result,
            vec![0xeeu8, 0x0u8, 0xd4u8, 0x0cu8, 0x82u8, 0x30u8, 0x60u8]
        );
    }

    #[test]
    fn test_bit_iterator() {
        let result = BitIterator::new(vec![0xa3u8]).collect::<Vec<u8>>();
        assert_eq!(
            result,
            vec![0x1u8, 0x0u8, 0x1u8, 0x0u8, 0x0u8, 0x0u8, 0x1u8, 0x1u8]
        );
    }

    #[test]
    fn test_parse_literal() {
        let mut bit_iter = BitIterator::new(parse_to_bytes("D2FE28").unwrap());
        let result = parse_packet(&mut bit_iter);

        assert_eq!(
            result.unwrap(),
            Packet {
                version: 6,
                value: Expr::Literal(2021)
            }
        );
    }

    #[test]
    fn test_parse_operator_0() {
        let mut bit_iter = BitIterator::new(parse_to_bytes("38006F45291200").unwrap());
        let result = parse_packet(&mut bit_iter);

        assert_eq!(
            result.unwrap(),
            Packet {
                version: 1,
                value: Expr::LessThan(vec![
                    Packet {
                        version: 6,
                        value: Expr::Literal(10)
                    },
                    Packet {
                        version: 2,
                        value: Expr::Literal(20)
                    }
                ])
            }
        );
    }

    #[test]
    fn test_parse_operator_1() {
        let mut bit_iter = BitIterator::new(parse_to_bytes("EE00D40C823060").unwrap());
        let result = parse_packet(&mut bit_iter);

        assert_eq!(
            result.unwrap(),
            Packet {
                version: 7,
                value: Expr::Maximum(vec![
                    Packet {
                        version: 2,
                        value: Expr::Literal(1)
                    },
                    Packet {
                        version: 4,
                        value: Expr::Literal(2)
                    },
                    Packet {
                        version: 1,
                        value: Expr::Literal(3)
                    }
                ])
            }
        );
    }

    #[test]
    fn test_version_sum() {
        let mut bit_iter = BitIterator::new(parse_to_bytes("8A004A801A8002F478").unwrap());
        assert_eq!(version_sum(&parse_packet(&mut bit_iter).unwrap()), 16);

        let mut bit_iter = BitIterator::new(parse_to_bytes("620080001611562C8802118E34").unwrap());
        assert_eq!(version_sum(&parse_packet(&mut bit_iter).unwrap()), 12);

        let mut bit_iter =
            BitIterator::new(parse_to_bytes("C0015000016115A2E0802F182340").unwrap());
        assert_eq!(version_sum(&parse_packet(&mut bit_iter).unwrap()), 23);

        let mut bit_iter =
            BitIterator::new(parse_to_bytes("A0016C880162017C3686B18A3D4780").unwrap());
        assert_eq!(version_sum(&parse_packet(&mut bit_iter).unwrap()), 31);
    }

    #[test]
    fn test_eval_packet() {
        let mut bit_iter = BitIterator::new(parse_to_bytes("C200B40A82").unwrap());
        assert_eq!(eval_packet(&parse_packet(&mut bit_iter).unwrap()), 3);

        let mut bit_iter = BitIterator::new(parse_to_bytes("04005AC33890").unwrap());
        assert_eq!(eval_packet(&parse_packet(&mut bit_iter).unwrap()), 54);

        let mut bit_iter = BitIterator::new(parse_to_bytes("880086C3E88112").unwrap());
        assert_eq!(eval_packet(&parse_packet(&mut bit_iter).unwrap()), 7);

        let mut bit_iter = BitIterator::new(parse_to_bytes("CE00C43D881120").unwrap());
        assert_eq!(eval_packet(&parse_packet(&mut bit_iter).unwrap()), 9);

        let mut bit_iter = BitIterator::new(parse_to_bytes("D8005AC2A8F0").unwrap());
        assert_eq!(eval_packet(&parse_packet(&mut bit_iter).unwrap()), 1);

        let mut bit_iter = BitIterator::new(parse_to_bytes("F600BC2D8F").unwrap());
        assert_eq!(eval_packet(&parse_packet(&mut bit_iter).unwrap()), 0);

        let mut bit_iter = BitIterator::new(parse_to_bytes("9C005AC2F8F0").unwrap());
        assert_eq!(eval_packet(&parse_packet(&mut bit_iter).unwrap()), 0);

        let mut bit_iter = BitIterator::new(parse_to_bytes("9C0141080250320F1802104A08").unwrap());
        assert_eq!(eval_packet(&parse_packet(&mut bit_iter).unwrap()), 1);
    }
}
