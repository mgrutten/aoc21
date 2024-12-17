use std::error::Error;
use std::fs;


fn hex_to_bits(hex: &str) -> String {
    hex.chars()
        .flat_map(|c| {
            let digit = c.to_digit(16).expect("Invalid hexadecimal character");
            format!("{:04b}", digit).chars().collect::<Vec<_>>()
        })
        .collect()
}


fn bits_to_decimal(bits: &str) -> u64 {
    u64::from_str_radix(bits, 2).expect("Invalid binary string")
}


fn decode_packet(binary: &str, idx: &mut usize) -> u64 {
    //let mut version_sum = 0;

    let packet_version = bits_to_decimal(&binary[*idx..*idx + 3]);
    *idx += 3;
    //println!("Packet version: {}", packet_version);
    //version_sum += packet_version;

    let packet_type = bits_to_decimal(&binary[*idx..*idx + 3]);
    *idx += 3;
    //println!("Packet type: {}", packet_type);

    if packet_type == 4 {
        // Label
        let mut label_bits = String::new();
        loop {
            let keep_bit = bits_to_decimal(&binary[*idx..*idx + 1]);
            *idx += 1;

            label_bits.push_str(&binary[*idx..*idx + 4]);
            *idx += 4;

            if keep_bit == 0 {
                break;
            }
        }

        bits_to_decimal(&label_bits)
        //println!("Label: {} {}", label_bits, bits_to_decimal(&label_bits));
    } else {
        // Operator
        let length_type = bits_to_decimal(&binary[*idx..*idx + 1]);
        *idx += 1;

        let mut values = Vec::new();

        if length_type == 0 {
            // Length of sub-packets in bit
            let length = bits_to_decimal(&binary[*idx..*idx + 15]);
            *idx += 15;
            //println!("Length: {}", length);

            let end_idx = *idx + length as usize;
            //let mut start_idx = idx;
            while *idx < end_idx {
                values.push(decode_packet(&binary, idx));
                //start_idx = idx;
            }
        } else if length_type == 1 {
            // Number of sub-packets
            let packets = bits_to_decimal(&binary[*idx..*idx + 11]);
            *idx += 11;
            //println!("Packets: {}", packets);

            for _ in 0..packets {
                values.push(decode_packet(&binary, idx));
            }
        } else {
            unreachable!();
        }

        match packet_type {
            0 => values.iter().sum(),
            1 => values.iter().fold(1, |acc, x| acc * x),
            2 => *values.iter().min().unwrap(),
            3 => *values.iter().max().unwrap(),
            4 => unreachable!(),
            5 => if values[0] > values[1] { 1 } else { 0 },
            6 => if values[0] < values[1] { 1 } else { 0 },
            7 => if values[0] == values[1] { 1 } else { 0 },
            _ => unreachable!()
        }
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    // Read in example
    let file_str: String = fs::read_to_string("data/day16/day16.txt")?;

    let binary_digits = file_str.chars()
        .flat_map(|c| {
            let digit = c.to_digit(16).expect("Invalid hexadecimal character");
            format!("{:04b}", digit).chars().collect::<Vec<_>>()
        })
        .collect::<String>();

    //println!("{}", file_str);
    //println!("{}", binary_digits);

    //decode_packet(binary_digits.as_str());
    let mut idx = 0;
    //decode_packet("110100101111111000101000", &mut idx);
    //println!();
    //decode_packet("00111000000000000110111101000101001010010001001000000000");
    //println!();
    //decode_packet("11101110000000001101010000001100100000100011000001100000");
    //println!();
    //decode_packet(&hex_to_bits("8A004A801A8002F478"));
    //println!();
    //decode_packet(&hex_to_bits("620080001611562C8802118E34"));
    //println!();
    //decode_packet(&hex_to_bits("C0015000016115A2E0802F182340"));
    //println!();
    //let version_sum = decode_packet(&hex_to_bits("A0016C880162017C3686B18A3D4780"), &mut idx);
    //
    //let version_sum = decode_packet(&binary_digits, &mut idx);
    //println!("Version sum: {}", version_sum);

    //let value = decode_packet(&hex_to_bits("9C0141080250320F1802104A08"), &mut idx);
    let value = decode_packet(&binary_digits, &mut idx);
    println!("Value: {}", value);

    Ok(())
}