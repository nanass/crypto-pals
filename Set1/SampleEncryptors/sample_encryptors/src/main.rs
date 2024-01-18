use std::str;

fn main() {
    let sample_string = "That is crazy then. Once I get a bunn I'll check them out. Or just him, since it seems like just a guy in his garage.";
    let sample_hash : u8 = 0b0111_1101;
    let encoded_bytes : Vec<u8> = to_hex(
                encrypt(
                    to_bytes(sample_string), 
                    sample_hash
                )
            ).into_iter().map(dec_to_utf_hex).collect();
    let result = utf8_to_string(&encoded_bytes);
    println!("{}", result);

}

fn to_bytes(input: &str) -> Vec<u8>{
    input.bytes().collect::<Vec<u8>>()
}

fn encrypt(to_encrypt: Vec<u8>, hash: u8) -> Vec<u8> {
    to_encrypt.iter().map(|e| e ^ hash).collect()
}

fn to_hex(input: Vec<u8>) -> Vec<u8> {
    let first_half : u8 = 0b1111_0000;
    let second_half : u8 = 0b0000_1111;

    let mut output : Vec<u8> = Vec::new();
    for element in input.iter() {
        output.push((element & first_half) >> 4);
        output.push(element & second_half);
    }
    output
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

fn utf8_to_string(input: &Vec<u8>) -> String {
    str::from_utf8(input).unwrap().to_string()
}