use std::str;
use std::collections::HashMap;

static FREQUENT_CHARACTERS : [char; 6] = ['e', 't', 'a' ,'o', 'i', 'n'];

fn main() {
    //let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let input = "29151c095d140e5d1e0f1c07045d09151813535d32131e185d345d1a18095d1c5d1f0813135d345a11115d1e15181e165d091518105d120809535d320f5d17080e095d151410515d0e14131e185d14095d0e1818100e5d111416185d17080e095d1c5d1a08045d14135d15140e5d1a1c0f1c1a1853";
    let result = decrypt(hex_input_to_string(input));
    println!("{}", result);
}

fn decrypt(input: String) -> String {
    let mut scored_decryptions : HashMap<String, u8> = HashMap::new();
    for ch in input.chars() {
        for one_ch in FREQUENT_CHARACTERS.iter() {
            let match_ch : u8 = ch as u8;
            let cipher : u8 = *one_ch as u8 ^ match_ch;
            let decrypted : Vec<u8> = input.chars().map(|c| c as u8 ^ cipher).collect();
            *scored_decryptions.entry(utf8_to_string(&decrypted)).or_insert(score(&decrypted)) = score(&decrypted);  
        }
    }
    let mut scored_vec: Vec<(&String, &u8)> = scored_decryptions.iter().collect();
    scored_vec.sort_by(|a, b| b.1.cmp(a.1));
    scored_vec[0].0.to_string()
}

fn score(character_vector: &Vec<u8>) -> u8 {
    let mut score: u8 = 0;

    for ch in str::from_utf8(&character_vector).unwrap().to_string().chars() {
        if FREQUENT_CHARACTERS.contains(&ch) {
            score += 1;
        }
    }
    score
}

fn hex_input_to_string(input: &str) -> String {
    let byte_vector : Vec<u8> = to_bytes(input)
        .into_iter()
        .map(utf8_hex_to_dec)
        .collect();
    let eight_bit : Vec<u8> = hex_to_octet(byte_vector);
    utf8_to_string(&eight_bit)
}

fn utf8_to_string(input: &Vec<u8>) -> String {
    match str::from_utf8(input)  {
        Ok(result) => result.to_string(),
        Err(e) => {
            println!("{:?}", e);
            "Err".to_string()
        },
    }
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