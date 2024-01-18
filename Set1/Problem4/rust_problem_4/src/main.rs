use std::fs;
use std::str;
use std::collections::HashMap;

fn main() {
    let contents = read_file("/Users/nathannass/Dropbox/BackupFromMacBook/HomeFolder/study/cyptopals/Set1/Problem4/4.txt".to_string());
    let result = search(&contents);
    println!("{}", find_single_char_encryption(result))
}

fn find_single_char_encryption<'a>(file_lines: Vec<&'a str>) -> String {
    let mut scored_decryptions : HashMap<String, u8> = HashMap::new();

    for line in file_lines.iter() {
        let decrypted : String = decrypt(hex_input_to_string(line));
        *scored_decryptions.entry(decrypted.to_string()).or_insert(score(&to_bytes(&decrypted))) = score(&to_bytes(&decrypted));  
    }

    let mut scored_vec: Vec<(&String, &u8)> = scored_decryptions.iter().collect();
    scored_vec.sort_by(|a, b| b.1.cmp(a.1));
    scored_vec[0].0.to_string()
}


fn search<'a>(contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        results.push(line);   
    }

    results
}

fn read_file(file_name: String) -> String {
    let contents = fs::read_to_string(file_name);
    return contents.unwrap();
}

static FREQUENT_CHARACTERS : [char; 6] = ['e', 't', 'a' ,'o', 'i', 'n'];


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
            //println!("{:?}", e);
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