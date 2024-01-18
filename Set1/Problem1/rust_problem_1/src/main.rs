use std::str;

fn main() {
    let input : &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let expected_output: &str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    let output = hex_str_to_base64_str(input);
    assert_eq!(output, expected_output);
    println!("{}",output.to_string());

    let hobbes_out = to_base64("Man is distinguished, not only by his reason, but by this singular passion from other animals, which is a lust of the mind, that by a perseverance of delight in the continued and indefatigable generation of knowledge, exceeds the short vehemence of any carnal pleasure.");
    let expected_hobbes_out = "TWFuIGlzIGRpc3Rpbmd1aXNoZWQsIG5vdCBvbmx5IGJ5IGhpcyByZWFzb24sIGJ1dCBieSB0aGlzIHNpbmd1bGFyIHBhc3Npb24gZnJvbSBvdGhlciBhbmltYWxzLCB3aGljaCBpcyBhIGx1c3Qgb2YgdGhlIG1pbmQsIHRoYXQgYnkgYSBwZXJzZXZlcmFuY2Ugb2YgZGVsaWdodCBpbiB0aGUgY29udGludWVkIGFuZCBpbmRlZmF0aWdhYmxlIGdlbmVyYXRpb24gb2Yga25vd2xlZGdlLCBleGNlZWRzIHRoZSBzaG9ydCB2ZWhlbWVuY2Ugb2YgYW55IGNhcm5hbCBwbGVhc3VyZS4=";
    assert_eq!(hobbes_out, expected_hobbes_out);
    println!("{}",hobbes_out.to_string());

    let output_2 = hex_str_to_base64_str("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20690a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
    println!("{}",output_2.to_string());
}

fn hex_str_to_base64_str(input: &str) -> String {
    let byte_arr: Vec<u8> = to_bytes(input);
    let base64_str = hex_to_base_64(byte_arr);
    str::from_utf8(&base64_str).unwrap().to_string()
}

fn hex_to_base_64(input : Vec<u8>) -> Vec<u8> {
    let byte_array: Vec<u8> = input.into_iter().map(utf8_hex_to_dec).collect();

    let octets = hex_to_octet(byte_array);

    let base64 : Vec<u8> = eight_bit_to_base64(octets);
    let base64_bytes : Vec<u8> =  base64.into_iter().map(print_base_64).collect();
    return base64_bytes;
}

fn to_base64(input: &str) -> String {
    let byte_arr: Vec<u8> = to_bytes(input);
    let base64 : Vec<u8> = eight_bit_to_base64(byte_arr);
    let base64_bytes : Vec<u8> =  base64.into_iter().map(print_base_64).collect();
    str::from_utf8(&base64_bytes).unwrap().to_string()
}

fn to_bytes(input: &str) -> Vec<u8>{
    return input.bytes().collect::<Vec<u8>>();
}

fn utf8_hex_to_dec(input: u8) -> u8 {
    let return_value: u8;
    if input >= 48 && input <= 57 {
        return_value = input - 48;
    }
    else if input >= 97 && input <= 102 {
        return_value = input - 87;
    } else {
        return_value = 0;
    }
    return return_value;
}

fn hex_to_octet(input: Vec<u8>) -> Vec<u8> {
    let iterator = input.iter();
    let mut output: Vec<u8> = Vec::new();
    let mut buffer: Vec<u8> = Vec::new();

    for item in iterator {
        if buffer.len() < 2 {
            buffer.push(*item);
        } else {
            let first = buffer.get(0).unwrap();
            let second = buffer.get(1).unwrap();
            output.push(first << 4 | second);
            buffer.drain(0..2);
            buffer.push(*item);
        }
    }
    if buffer.len() > 0 {
        if buffer.get(1).is_some() {
            let first = buffer.get(0).unwrap();
            let second = buffer.get(1).unwrap();
            output.push(first << 4 | second);

        } else {
            let first = buffer.get(0).unwrap();
            output.push(first << 4);
        }
    }
    output
}

fn eight_bit_to_base64(input: Vec<u8>) -> Vec<u8> {
    let iterator = input.iter();
    let mut output: Vec<u8> = Vec::new();
    let mut buffer: Vec<u8> = Vec::new();

    for item in iterator {
        if buffer.len() < 3 {
            buffer.push(*item);
        } else {
            let first = buffer.get(0).unwrap();
            let second = buffer.get(1).unwrap();
            let third = buffer.get(2).unwrap();

            let base64first = first >> 2;
            let base64second = ((first << 4) as u8 & 0b0011_0000) | (second >> 4);
            let base64third = ((second << 2) as u8 & 0b0011_1100) | (third >> 6);
            let base64fourth = third & 0b0011_1111;

            output.push(base64first);
            output.push(base64second);
            output.push(base64third);
            output.push(base64fourth);

            buffer.drain(0..3);
            buffer.push(*item);
        }
    }

    if buffer.len() > 0 {
        let first = buffer.get(0).unwrap();
        let second_1 = *buffer.get(1).get_or_insert(&0b0000_0000);

        let base64first = first >> 2;
        let base64second = ((first << 4) as u8 & 0b0011_0000) | (second_1 >> 4);

        output.push(base64first);
        output.push(base64second);

        if buffer.get(1).is_some() {
            let second_2 = *buffer.get(1).unwrap();
            let third = *buffer.get(2).get_or_insert(&0b0000_0000);
            let base64third = ((second_2 << 2) & 0b0011_1100) | (third >> 6);
            output.push(base64third);
            if buffer.get(2).is_some() {
                let third = buffer.get(2).unwrap();
                let base64fourth = third & 0b0011_1111;
                output.push(base64fourth);
            } else {
                output.push(64);
            }
        } else {
            output.push(64);
            output.push(64);
        }
    }
    return output;
} 

fn print_base_64(input:u8) -> u8 {
    let out: char = match input {
        0 => 'A',
        1 => 'B',
        2 => 'C',
        3 => 'D',
        4 => 'E',
        5 => 'F',
        6 => 'G',
        7 => 'H',
        8 => 'I',
        9 => 'J',
        10 => 'K',
        11 => 'L',
        12 => 'M',
        13 => 'N',
        14 => 'O',
        15 => 'P',
        16 => 'Q',
        17 => 'R',
        18 => 'S',
        19 => 'T',
        20 => 'U',
        21 => 'V',
        22 => 'W',
        23 => 'X',
        24 => 'Y',
        25 => 'Z',
        26 => 'a',
        27 => 'b',
        28 => 'c',
        29 => 'd',
        30 => 'e',
        31 => 'f',
        32 => 'g',
        33 => 'h',
        34 => 'i',
        35 => 'j',
        36 => 'k',
        37 => 'l',
        38 => 'm',
        39 => 'n',
        40 => 'o',
        41 => 'p',
        42 => 'q',
        43 => 'r',
        44 => 's',
        45 => 't',
        46 => 'u',
        47 => 'v',
        48 => 'w',
        49 => 'x',
        50 => 'y',
        51 => 'z',
        52 => '0',
        53 => '1',
        54 => '2',
        55 => '3',
        56 => '4',
        57 => '5',
        58 => '6',
        59 => '7',
        60 => '8',
        61 => '9',
        62 => '+',
        63 => '/',
        _ => '='
    };
    return out as u8;
}

// DEBUG
#[allow(dead_code)]
fn print_hex_bits(byte: u8){
    println!("{:04b}", byte);
}

#[allow(dead_code)]
fn print_8_bits(byte: u8){
    println!("{:08b}", byte)
}

#[allow(dead_code)]
fn print_byte_int(byte: u8){
    println!("{}", byte)
}

#[allow(dead_code)]
fn print_utf8_string(byte: u8){
    println!("{}", std::str::from_utf8(vec![byte].as_slice()).unwrap().to_string());
}