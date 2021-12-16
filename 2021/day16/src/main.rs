use std::fs;

enum PacketType {
    Literal,
    Sum,
    Product,
    Min,
    Max,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl PacketType {
    fn from_integer(i: u64) -> Self {
        match i {
            0 => Self::Sum,
            1 => Self::Product,
            2 => Self::Min,
            3 => Self::Max,
            4 => Self::Literal,
            5 => Self::GreaterThan,
            6 => Self::LessThan,
            7 => Self::EqualTo,
            _ => panic!("Unsupported type"),
        }
    }
}

struct Packet {
    version: u64,
    _type: PacketType,
    sub_packets: Vec<Packet>,
    value: Option<u64>,
}

impl Packet {
    fn from_chars(mut chars: &mut Vec<char>) -> Self {
        let version = string_to_integer(chars.drain(0..3).collect());
        let packet_type =
            PacketType::from_integer(string_to_integer(chars.drain(0..3).into_iter().collect()));

        match packet_type {
            PacketType::Literal => {
                let mut value: Vec<char> = vec![];
                loop {
                    let p = chars.drain(0..5).collect::<Vec<_>>();
                    value.extend(p[1..5].into_iter());

                    if p[0] != '1' {
                        break;
                    }
                }

                Self {
                    sub_packets: vec![],
                    _type: packet_type,
                    version,
                    value: Some(string_to_integer(value.into_iter().collect())),
                }
            }
            _ => {
                let length_type_id = chars.remove(0);
                let sub_packets = match length_type_id {
                    '0' => {
                        let sub_packets_length =
                            string_to_integer(chars.drain(0..15).into_iter().collect());
                        let mut encoded_sub_packets = chars
                            .drain(0..sub_packets_length as usize)
                            .collect::<Vec<_>>();
                        let mut sub_packets = vec![];
                        while !encoded_sub_packets.is_empty() {
                            sub_packets.push(Packet::from_chars(&mut encoded_sub_packets));
                        }

                        sub_packets
                    }
                    _ => {
                        let sub_packets_count =
                            string_to_integer(chars.drain(0..11).into_iter().collect());
                        (0..sub_packets_count as usize).fold(vec![], |mut acc, _| {
                            acc.push(Packet::from_chars(&mut chars));
                            acc
                        })
                    }
                };

                Self {
                    sub_packets,
                    _type: packet_type,
                    version,
                    value: None,
                }
            }
        }
    }

    fn sum_versions(&self) -> u32 {
        self.version as u32
            + self
                .sub_packets
                .iter()
                .fold(0, |acc, p| acc + p.sum_versions())
    }

    fn get_value(self) -> u64 {
        match self._type {
            PacketType::Literal => self.value.unwrap(),
            PacketType::Sum => self
                .sub_packets
                .into_iter()
                .fold(0, |acc, sub_packet| acc + sub_packet.get_value()),
            PacketType::Product => self
                .sub_packets
                .into_iter()
                .fold(1, |acc, sub_packet| acc * sub_packet.get_value()),
            PacketType::Min => self
                .sub_packets
                .into_iter()
                .fold(u64::MAX, |acc, sub_packet| {
                    let sub_packet_value = sub_packet.get_value();
                    if sub_packet_value < acc {
                        sub_packet_value
                    } else {
                        acc
                    }
                }),
            PacketType::Max => self.sub_packets.into_iter().fold(0, |acc, sub_packet| {
                let sub_packet_value = sub_packet.get_value();
                if sub_packet_value > acc {
                    sub_packet_value
                } else {
                    acc
                }
            }),
            PacketType::GreaterThan => {
                let mut sub_packets = self.sub_packets.into_iter();
                (sub_packets.nth(0).unwrap().get_value() > sub_packets.nth(0).unwrap().get_value())
                    .into()
            }
            PacketType::LessThan => {
                let mut sub_packets = self.sub_packets.into_iter();
                (sub_packets.nth(0).unwrap().get_value() < sub_packets.nth(0).unwrap().get_value())
                    .into()
            }
            PacketType::EqualTo => {
                let mut sub_packets = self.sub_packets.into_iter();
                (sub_packets.nth(0).unwrap().get_value() == sub_packets.nth(0).unwrap().get_value())
                    .into()
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let packet = parse_input(&input);

    println!("PART 1 -> Packet versions sum {:?}", packet.sum_versions());
    println!("PART 2 -> Packet value {:?}", packet.get_value());
}

fn parse_input(input: &str) -> Packet {
    Packet::from_chars(
        &mut input
            .chars()
            .map(|c| hex_char_to_binary(c).chars().collect::<Vec<_>>())
            .flatten()
            .collect(),
    )
}

fn hex_char_to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

fn string_to_integer(s: String) -> u64 {
    u64::from_str_radix(s.as_str(), 2).unwrap()
}
