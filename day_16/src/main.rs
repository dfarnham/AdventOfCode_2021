use general::read_data_lines;
use structopt::StructOpt;

const PUZZLE_NAME: &str = "Advent of Code: Day 16 -- Version:";
const PUZZLE_ABOUT: &str = "Packet Decoder: https://adventofcode.com/2021/day/16";

const MIN_PACKET_BITS: usize = 11;
const PACKET_HEADER: usize = 6;

// Literal value packets encode a single binary number.
// To do this, the binary number is padded with leading zeroes until its length is a multiple of four bits,
// and then it is broken into groups of four bits. Each group is prefixed by a 1 bit except the last group,
// which is prefixed by a 0 bit. These groups of five bits immediately follow the packet header.
// For example, the hexadecimal string D2FE28 becomes:
//
//         0111  1110  0101
// 110100 10111 11110 00101000
// VVVTTT AAAAA BBBBB CCCCC
//
// 110100101111111000101000
// VVVTTTAAAAABBBBBCCCCC
//
// Below each bit is a label indicating its purpose:
//
//  The three bits labeled V (110) are the packet version, 6.
//  The three bits labeled T (100) are the packet type ID, 4, which means the packet is a literal value.
//  The five bits labeled A (10111) start with a 1 (not the last group, keep reading) and contain the first four bits of the number, 0111.
//  The five bits labeled B (11110) start with a 1 (not the last group, keep reading) and contain four more bits of the number, 1110.
//  The five bits labeled C (00101) start with a 0 (last group, end of packet) and contain the last four bits of the number, 0101.
//  The three unlabeled 0 bits at the end are extra due to the hexadecimal representation and should be ignored.
//
// So, this packet represents a literal value with binary representation 011111100101, which is 2021 in decimal.
//
// Every other type of packet (any packet with a type ID other than 4) represent an operator that performs some calculation on one
// or more sub-packets contained within. Right now, the specific operations aren't important; focus on parsing the hierarchy of sub-packets.
//
// An operator packet contains one or more packets. To indicate which subsequent binary data represents its sub-packets, an operator
// packet can use one of two modes indicated by the bit immediately after the packet header; this is called the length type ID:
//
//  If the length type ID is 0, then the next 15 bits are a number that represents the total length in bits of the sub-packets contained by this packet.
//  If the length type ID is 1, then the next 11 bits are a number that represents the number of sub-packets immediately contained by this packet.
//
// Finally, after the length type ID bit and the 15-bit or 11-bit field, the sub-packets appear.
//
// For example, here is an operator packet (hexadecimal string 38006F45291200) with length type ID 0 that contains two sub-packets:
//
// 00111000000000000110111101000101001010010001001000000000
// VVVTTTILLLLLLLLLLLLLLLAAAAAAAAAAABBBBBBBBBBBBBBBB
//
//     The three bits labeled V (001) are the packet version, 1.
//     The three bits labeled T (110) are the packet type ID, 6, which means the packet is an operator.
//     The bit labeled I (0) is the length type ID, which indicates that the length is a 15-bit number representing the number of bits in the sub-packets.
//     The 15 bits labeled L (000000000011011) contain the length of the sub-packets in bits, 27.
//     The 11 bits labeled A contain the first sub-packet, a literal value representing the number 10.
//     The 16 bits labeled B contain the second sub-packet, a literal value representing the number 20.
//
// After reading 11 and 16 bits of sub-packet data, the total length indicated in L (27) is reached, and so parsing of this packet stops.

#[derive(Debug, PartialEq)]
struct Packet {
    version: u8,
    id: TypeId,
    op: Op,
    sub_packets: Option<Vec<Packet>>,
}

#[derive(Debug, PartialEq)]
pub enum TypeId {
    Literal(usize), // number of bits to encode NUM
    Operator(Payload),
}

#[derive(Debug, PartialEq)]
pub enum Op {
    NUM(u64),
    SUM,
    PROD,
    MIN,
    MAX,
    GT,
    LT,
    EQ,
}

#[derive(Debug, PartialEq)]
pub enum Payload {
    BitLen(usize),       // number of bits in embedded packet
    SubPacketLen(usize), // number of following packets
}

fn decode_packet(bits: &[u8]) -> Packet {
    let version = bits2num(bits, 0, 3) as u8;
    let type_id = bits2num(bits, 3, 3) as u8;

    // temporary assignment, set when type_id == 4
    let mut literal_num = Op::NUM(0);
    let id = match type_id == 4 {
        true => {
            let mut nibbles = vec![];
            for (i, bit) in bits.iter().skip(PACKET_HEADER).enumerate() {
                if i % 5 == 0 {
                    if *bit == 0 {
                        nibbles.extend(&bits[(i + PACKET_HEADER + 1)..(i + PACKET_HEADER + 5)]);
                        break;
                    }
                    continue;
                }
                nibbles.push(*bit);
            }
            literal_num = Op::NUM(bits2num(&nibbles, 0, nibbles.len()));
            TypeId::Literal(nibbles.len() + nibbles.len() / 4)
        }
        false => match bits[PACKET_HEADER] == 0 {
            true => TypeId::Operator(Payload::BitLen(bits2num(bits, PACKET_HEADER + 1, 15) as usize)),
            false => TypeId::Operator(Payload::SubPacketLen(bits2num(bits, PACKET_HEADER + 1, 11) as usize)),
        },
    };

    let op = match id {
        TypeId::Literal(_) => literal_num,
        TypeId::Operator(_) => match type_id {
            0 => Op::SUM,
            1 => Op::PROD,
            2 => Op::MIN,
            3 => Op::MAX,
            5 => Op::GT,
            6 => Op::LT,
            7 => Op::EQ,
            _ => panic!("invalid type_id = {}", type_id),
        },
    };

    let sub_packets: Option<Vec<Packet>> = match id {
        TypeId::Literal(_) => None,
        TypeId::Operator(Payload::SubPacketLen(_)) => None,
        TypeId::Operator(Payload::BitLen(n)) => {
            Some(get_packets(&bits[(PACKET_HEADER + 16)..(PACKET_HEADER + 16 + n)]))
        }
    };

    Packet {
        version,
        id,
        op,
        sub_packets,
    }
}

fn get_packets(bits: &[u8]) -> Vec<Packet> {
    let mut offset = 0;
    let mut packets = vec![];
    while offset + MIN_PACKET_BITS <= bits.len() {
        let pack = decode_packet(&bits[offset..]);
        offset += match pack.id {
            TypeId::Literal(n) => PACKET_HEADER + n,
            TypeId::Operator(Payload::BitLen(n)) => PACKET_HEADER + 16 + n,
            TypeId::Operator(Payload::SubPacketLen(_)) => PACKET_HEADER + 12,
        };
        packets.push(pack);
    }
    packets
}

fn bits2num(bits: &[u8], index: usize, n: usize) -> u64 {
    bits.iter().skip(index).take(n).fold(0, |acc, b| acc << 1 | *b as u64)
}

fn get_bits(msg: &str) -> Vec<u8> {
    let mut bits = Vec::<u8>::new();
    for c in msg.chars() {
        let nibble = u8::from_str_radix(&c.to_string(), 16).unwrap();
        bits.push(nibble >> 3 & 1);
        bits.push(nibble >> 2 & 1);
        bits.push(nibble >> 1 & 1);
        bits.push(nibble & 1);
    }
    bits
}

fn operator_value(op: &Op, values: &[u64]) -> u64 {
    let mut stack = values.to_vec();

    match op {
        Op::SUM => stack.iter().sum::<u64>(),
        Op::PROD => stack.iter().product::<u64>(),
        Op::MIN => *stack.iter().min().unwrap(),
        Op::MAX => *stack.iter().max().unwrap(),
        Op::GT => match stack.pop() < stack.pop() {
            true => 1,
            false => 0,
        },
        Op::LT => match stack.pop() > stack.pop() {
            true => 1,
            false => 0,
        },
        Op::EQ => match stack.pop() == stack.pop() {
            true => 1,
            false => 0,
        },
        _ => panic!("operator_value"),
    }
}

// counts the actual number of packets needed for an input packet range
// taking into account SubPacketLen packets extend the count
fn packets_needed(n: usize, packets: &[Packet]) -> usize {
    let mut target = n;
    let mut index = 0;
    while index < target {
        if let TypeId::Operator(Payload::SubPacketLen(c)) = packets[index].id {
            target += c;
        }
        index += 1;
    }
    index
}

fn eval(packets: &[Packet]) -> Vec<u64> {
    let mut stack = Vec::<u64>::new();
    let mut index = 0;
    while index < packets.len() {
        match &packets[index].id {
            TypeId::Literal(_) => {
                let num = match packets[index].op {
                    Op::NUM(n) => n,
                    _ => panic!("Literal Op = {:?}", packets[index].op),
                };
                stack.push(num);
            }
            TypeId::Operator(Payload::BitLen(_)) => {
                stack.push(operator_value(
                    &packets[index].op,
                    &eval(packets[index].sub_packets.as_ref().unwrap()),
                ));
            }
            TypeId::Operator(Payload::SubPacketLen(n)) => {
                let count = packets_needed(*n, &packets[(index + 1)..]);
                stack.push(operator_value(
                    &packets[index].op,
                    &eval(&packets[(index + 1)..(index + 1 + count)]),
                ));
                index += count;
            }
        };
        index += 1;
    }
    stack
}

fn solution1(packets: &[Packet]) -> u64 {
    let mut total = 0;
    for p in packets.iter() {
        total += p.version as u64;
        if let Some(sp) = &p.sub_packets {
            total += solution1(sp);
        }
    }
    total
}

fn solution2(packets: &[Packet]) -> u64 {
    eval(packets)[0]
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[derive(StructOpt)]
    #[structopt(name = PUZZLE_NAME, about = PUZZLE_ABOUT)]
    struct Cli {
        #[structopt(short, long, parse(from_os_str), help = "file|stdin -- puzzle input")]
        input: Option<std::path::PathBuf>,
    }
    let args = Cli::from_args();

    // ==============================================================

    let data = read_data_lines::<String>(args.input)?;
    let bits = get_bits(&data[0]);
    let packets = get_packets(&bits);
    //println!("main packets = {:#?}", packets);
    println!("Answer Part 1 = {:?}", solution1(&packets));
    println!("Answer Part 2 = {:?}", solution2(&packets));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_data(filename: &str) -> Vec<String> {
        let file = Some(std::path::PathBuf::from(filename));
        read_data_lines::<String>(file).unwrap()
    }

    #[test]
    fn part1_example() {
        let data = get_data("input-example");
        let bits = get_bits(&data[0]);
        let packets = get_packets(&bits);
        assert_eq!(solution1(&packets), 6);
    }

    #[test]
    fn part1_actual() {
        let data = get_data("input-actual");
        let bits = get_bits(&data[0]);
        let packets = get_packets(&bits);
        assert_eq!(solution1(&packets), 866);
    }

    #[test]
    fn part2_example() {
        let data = get_data("input-example");
        let bits = get_bits(&data[0]);
        let packets = get_packets(&bits);
        assert_eq!(solution2(&packets), 2021);
    }

    #[test]
    fn part2_actual() {
        let data = get_data("input-actual");
        let bits = get_bits(&data[0]);
        let packets = get_packets(&bits);
        assert_eq!(solution2(&packets), 1392637195518);
    }
}
