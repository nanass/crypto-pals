use std::str;

fn main() {
    let input = "1c0111001f010100061a024b53535009181c";
    let xor_string = "686974207468652062756c6c277320657965";
    let expected_output = "746865206b696420646f6e277420706c6179";
    let result = xor_strings(input, xor_string);
    println!("{}",result.to_string());
    assert_eq!(result, expected_output);
}

// function that XORs equal length strings
fn xor_strings(input_1: &str, input_2: &str) -> String {
    let hex_bytes_1 : Vec<u8> = to_bytes(input_1).into_iter().map(utf8_hex_to_dec).collect();
    let hex_bytes_2 : Vec<u8> = to_bytes(input_2).into_iter().map(utf8_hex_to_dec).collect();

    let xored_vector : Vec<u8> = xor_u8_buffers(hex_bytes_1, hex_bytes_2).into_iter().map(dec_to_utf_hex).collect();

    str::from_utf8(&xored_vector).unwrap().to_string()
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

fn dec_to_utf_hex(input: u8) -> u8 {
    let return_value: u8;
    if input <= 9 {
        return_value = input + 48;
    }
    else if input >= 10 && input <= 15 {
        return_value = input + 87;
    } else {
        return_value = 0;
    }
    return return_value;
}

fn to_bytes(input: &str) -> Vec<u8>{
    return input.bytes().collect::<Vec<u8>>();
}

fn xor_u8_buffers(input_1: Vec<u8>, input_2: Vec<u8>) -> Vec<u8> {
    assert_eq!(input_1.len(), input_2.len());
    let zipped_vectors = input_1.iter().zip(input_2.iter());
    zipped_vectors.map(|(first, second)| first ^ second).collect()
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