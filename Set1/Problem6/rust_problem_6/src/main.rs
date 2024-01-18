use core::cmp::Ordering::Equal;
use std::collections::HashMap;
use std::fs;
use std::str;

fn main() {
    let contents = read_file(
        "/Users/nathannass/Documents/study/cyptopals/Set1/Problem6/6.txt".to_string(),
    );
    println!("{}", decrypt_base_64_string(&contents));
}

fn read_file(file_name: String) -> String {
    let contents: &str = &fs::read_to_string(file_name).unwrap();
    let mut results = Vec::new();

    for line in contents.lines() {
        results.push(line);
    }
    results.join("").to_string()
}

fn decrypt_base_64_string(input: &str) -> String {
    // Algorithm
    // Decode base 64 string

    let mut scored_decryptions: HashMap<String, u32> = HashMap::new();

    let decoded_bytes: Vec<u8> = decode_base_64(input);

    // Find key size
    for key_size in find_key_sizes(&decoded_bytes) {
        let decrypted : String = decrypt_repeating_key(key_size, decoded_bytes.clone());
        let decrypted_bytes = decrypted.clone().into_bytes();
        *scored_decryptions
            .entry(decrypted)
            .or_insert(score_combinations(&decrypted_bytes)) = score_combinations(&decrypted_bytes);
    }

    let mut scored_vec: Vec<(&String, &u32)> = scored_decryptions.iter().collect();
    scored_vec.sort_by(|a, b| b.1.cmp(a.1)); 
    scored_vec[0].0.to_string()
}

fn decrypt(to_encrypt: &Vec<u8>, hash: Vec<u8>) -> Vec<u8> {
    let mut hash_repeat = hash.iter().cloned().cycle().into_iter();
    to_encrypt
        .iter()
        .map(|e| e ^ &hash_repeat.next().unwrap())
        .collect()
}

fn partition_buckets(size: u8, input: &Vec<u8>) -> Vec<Vec<u8>> {
    let mut buckets = Vec::new();

    // Break sequence into chunks of key size
    for _bucket in 1..=size {
        buckets.push(Vec::new());
    }

    // Segment chunks into individual parts and associate parts by cardinality accross chunks
    for (idx, elem) in input.into_iter().enumerate() {
        buckets.get_mut(idx % size as usize).unwrap().push(*elem);
    }

    return buckets;
}

fn decrypt_repeating_key(size: u8, input: Vec<u8>) -> String {
    let buckets: Vec<Vec<u8>> = partition_buckets(size, &input);
    // For each associated segment find individual cipher using frequency analysis
    let repeating_key: Vec<u8> = buckets.into_iter().map(find_cipher).collect();

    // Decrypt using found repeating XOR
    let result: Vec<u8> = decrypt(&input, repeating_key);
    return utf8_to_string(&result);
}

static FREQUENT_CHARACTERS: [char; 26] = [' ', 'e', 't', 'a', 'o', 'i', 'n','s','h','r','d','l','u',
                                          ' ', 'E', 'T', 'A', 'O', 'I', 'N','S','H','R','D','L','U'];

fn FREQUENT_COMBINATIONS() -> Vec<String> { ["th", "er", "on", "an",
                                              "re", "he", "in", "ed", "nd", "ha",
                                              "at", "en", "es", "of", "or", "nt",
                                              "ea", "ti", "to", "it", "st", "io",
                                              "le", "is", "ou", "ar", "as", "de",
                                              "rt", "ve"].into_iter().map(|a| a.to_string()).collect()
}

fn score_combinations(character_vector: &Vec<u8>) -> u32 {
    let mut character_iterator = character_vector.into_iter();
    let mut char1 : &u8 = character_iterator.next().unwrap();
    let mut char2 : &u8 = character_iterator.next().unwrap();
    let mut score: u32 = 0;
    for char3 in character_iterator {
        let combination : String= str::from_utf8(&[*char1, *char2]).unwrap().to_string();
        if FREQUENT_COMBINATIONS().contains(&combination){
            let rank : u32 = ( FREQUENT_COMBINATIONS().into_iter().position(|r| r == combination).unwrap()) as u32;
            score += rank;
        }
        char1 = char2;
        char2 = char3;
    }
    let combination : String= str::from_utf8(&[*char1, *char2]).unwrap().to_string();
    if FREQUENT_COMBINATIONS().contains(&combination){
        let rank : u32 = (FREQUENT_COMBINATIONS().into_iter().position(|r| r == combination).unwrap()) as u32;
        score += rank;
    }
    return score;

}

fn find_cipher(input: Vec<u8>) -> u8 {
    let mut scored_decryptions: HashMap<u8, u32> = HashMap::new();
    let iter = input.into_iter();
    for ch in iter.clone() {
        for one_ch in FREQUENT_CHARACTERS.iter() {
            let match_ch: u8 = ch;
            let cipher: u8 = *one_ch as u8 ^ match_ch;
            let decrypted: Vec<u8> = iter.clone().map(|c| c ^ cipher).collect();
            *scored_decryptions
                .entry(cipher)
                .or_insert(score(&decrypted)) = score(&decrypted);
        }
    }
    let mut scored_vec: Vec<(&u8, &u32)> = scored_decryptions.iter().collect();
    scored_vec.sort_by(|a, b| b.1.cmp(a.1));
    *scored_vec[0].0
}

fn score(character_vector: &Vec<u8>) -> u32 {
    let mut score: u32 = 0;

    for ch in str::from_utf8(&character_vector)
        .unwrap()
        .to_string()
        .chars()
    {
        if FREQUENT_CHARACTERS.contains(&ch) {
            let rank : u32 = (26 as usize - FREQUENT_CHARACTERS.into_iter().position(|&r| r == ch).unwrap()) as u32;
            score += if rank > 13 {
                rank + 13
            } else {
                rank
            };
        }
    }
    score
}

fn find_key_size(encrypted_input: &Vec<u8>) -> u8 {
    // For i from 2 -> 40
    // Take n i chunks of bytes
    // Find distance "d" between chunks
    // normalized = d / i
    // Key size = i with lowest normalized value
    let encrypted_size = encrypted_input.len() / 2;

    let sizes_to_test: u8 = if encrypted_size > 40 {
        40
    } else {
        encrypted_size as u8
    };

    let mut scored_distances: HashMap<u8, f64> = HashMap::new();
    for i in 2..=sizes_to_test {
        *scored_distances
            .entry(i)
            .or_insert(get_normalized_distance(i, &encrypted_input)) =
            get_normalized_distance(i, &encrypted_input);
    }
    let mut scored_vec: Vec<(&u8, &f64)> = scored_distances.iter().collect();
    scored_vec.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(Equal));
    let mut top_5 : Vec<u8> = scored_vec.clone().into_iter().take(5).map(|(a, _b)| *a).collect();
    top_5.sort_by(|a, b| a.cmp(b));
    top_5[0]
}


fn find_key_sizes(encrypted_input: &Vec<u8>) -> Vec<u8> {
    // For i from 2 -> 40
    // Take n i chunks of bytes
    // Find distance "d" between chunks
    // normalized = d / i
    // Key size = i with lowest normalized value
    let encrypted_size = encrypted_input.len() / 2;

    let sizes_to_test: u8 = if encrypted_size > 40 {
        40
    } else {
        encrypted_size as u8
    };

    let mut scored_distances: HashMap<u8, f64> = HashMap::new();
    for i in 2..=sizes_to_test {
        *scored_distances
            .entry(i)
            .or_insert(get_normalized_distance(i, &encrypted_input)) =
            get_normalized_distance(i, &encrypted_input);
    }
    let mut scored_vec: Vec<(&u8, &f64)> = scored_distances.iter().collect();
    scored_vec.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(Equal));
    let mut top_5 : Vec<u8> = scored_vec.clone().into_iter().take(5).map(|(a, _b)| *a).collect();
    top_5.sort_by(|a, b| a.cmp(b));
    top_5
}



fn get_normalized_distance(size: u8, encrypted_input: &Vec<u8>) -> f64 {
    let mut distance: u32 = 0;
    let mut iterator = encrypted_input.into_iter();

    let mut times_to_avg: u8 = 4;

    for n in 1..=times_to_avg {
        let mut first_chunk: Vec<u8> = Vec::new();
        let mut second_chunk: Vec<u8> = Vec::new();
        for i in 1..=size {
            match iterator.next() {
                Some(elem) => first_chunk.push(*elem),
                None => (),
            }
        }

        for i in 1..=size {
            match iterator.next() {
                Some(elem) => second_chunk.push(*elem),
                None => (),
            }
        }

        if first_chunk.len() == size as usize && second_chunk.len() == size as usize {
            distance += hamming_distance(first_chunk.clone(), second_chunk.clone());
        } else {
            times_to_avg = times_to_avg - 1;
        }
    }
    let normalized: f64 = distance as f64 / (size as f64) / times_to_avg as f64;
    return normalized;
}



fn utf8_to_string(input: &Vec<u8>) -> String {
    str::from_utf8(input).unwrap().to_string()
}

fn decode_base_64(input: &str) -> Vec<u8> {
    let input_bytes: Vec<u8> = input.chars().map(base_64_to_6_bit).collect();
    let decoded_bytes = six_bit_base_64_to_8_bit(input_bytes);
    return decoded_bytes;
}

fn six_bit_base_64_to_8_bit(input: Vec<u8>) -> Vec<u8> {
    let iterator = input.iter();
    let mut output: Vec<u8> = Vec::new();
    let mut buffer: Vec<u8> = Vec::new();
    let default = 0b0000_0000;
    let place_holder = 64 as u8;

    for item in iterator {
        if buffer.len() < 4 {
            buffer.push(*item);
        } else {
            let mut first = buffer.get(0).unwrap();
            let mut second = buffer.get(1).unwrap();
            let mut third = buffer.get(2).unwrap();
            let mut fourth = buffer.get(3).unwrap();

            if first == &place_holder {
                second = &default;
            }

            if second == &place_holder {
                second = &default;
            }

            if third == &place_holder {
                third = &default;
            }

            if fourth == &place_holder {
                fourth = &default;
            }

            let decoded_first = (first << 2) | (second >> 4);
            let decoded_second = (second << 4) | (third >> 2);
            let decoded_third = (third << 6) | fourth;

            if first != &place_holder || second != &place_holder {
                output.push(decoded_first);
            }

            if second != &place_holder || third != &place_holder {
                output.push(decoded_second);
            }

            if third != &place_holder || fourth != &place_holder {
                output.push(decoded_third);
            }

            buffer.drain(0..4);
            buffer.push(*item);
        }
    }

    if buffer.len() > 0 {
        let first = buffer.get(0).unwrap();
        let mut second = buffer.get(1).unwrap();
        let mut third = buffer.get(2).unwrap();
        let mut fourth = buffer.get(3).unwrap();

        if second == &place_holder {
            second = &default;
        }
        if third == &place_holder {
            third = &default;
        }
        if fourth == &place_holder {
            fourth = &default;
        }

        let decoded_first = (first << 2) | (second >> 4);
        let decoded_second = (second << 4) | (third >> 2);
        let decoded_third = (third << 6) | fourth;

        if first != &place_holder || second != &place_holder && decoded_first != default {
            output.push(decoded_first);
        }

        if second != &place_holder || third != &place_holder && decoded_second != default {
            output.push(decoded_second);
        }

        if (third != &place_holder || fourth != &place_holder) && decoded_third != default {
            output.push(decoded_third);
        }
    }
    return output;
}

fn base_64_to_6_bit(input: char) -> u8 {
    let out: u8 = match input {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        'E' => 4,
        'F' => 5,
        'G' => 6,
        'H' => 7,
        'I' => 8,
        'J' => 9,
        'K' => 10,
        'L' => 11,
        'M' => 12,
        'N' => 13,
        'O' => 14,
        'P' => 15,
        'Q' => 16,
        'R' => 17,
        'S' => 18,
        'T' => 19,
        'U' => 20,
        'V' => 21,
        'W' => 22,
        'X' => 23,
        'Y' => 24,
        'Z' => 25,
        'a' => 26,
        'b' => 27,
        'c' => 28,
        'd' => 29,
        'e' => 30,
        'f' => 31,
        'g' => 32,
        'h' => 33,
        'i' => 34,
        'j' => 35,
        'k' => 36,
        'l' => 37,
        'm' => 38,
        'n' => 39,
        'o' => 40,
        'p' => 41,
        'q' => 42,
        'r' => 43,
        's' => 44,
        't' => 45,
        'u' => 46,
        'v' => 47,
        'w' => 48,
        'x' => 49,
        'y' => 50,
        'z' => 51,
        '0' => 52,
        '1' => 53,
        '2' => 54,
        '3' => 55,
        '4' => 56,
        '5' => 57,
        '6' => 58,
        '7' => 59,
        '8' => 60,
        '9' => 61,
        '+' => 62,
        '/' => 63,
        '=' => 64,
        _ => 64,
    };
    return out as u8;
}

fn hamming_distance_for_strings(first_string: &str, second_string: &str) -> u32 {
    let bytes_1 = to_bytes(first_string);
    let bytes_2 = to_bytes(second_string);
    return hamming_distance(bytes_1, bytes_2);
}

fn hamming_distance(bytes_1: Vec<u8>, bytes_2: Vec<u8>) -> u32 {
    let mut result: u32 = 0;

    for (byte_1, byte_2) in bytes_1.into_iter().zip(bytes_2.into_iter()) {
        result += hamming_distance_per_byte(byte_1, byte_2);
    }

    return result;
}

fn hamming_distance_per_byte(first: u8, second: u8) -> u32 {
    let diff: u8 = first ^ second;
    diff.count_ones()
}

fn to_bytes(input: &str) -> Vec<u8> {
    input.bytes().collect::<Vec<u8>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1_for_bit_shift() {
        let test_vector: Vec<u8> = vec![0b0011_0000, 0b0011_1111, 0b0000_0010, 0b0011_1100];
        let expected_result: Vec<u8> = vec![0b1100_0011, 0b1111_0000, 0b1011_1100];
        assert_eq!(expected_result, six_bit_base_64_to_8_bit(test_vector));
    }

    #[test]
    fn test_normalized_distance_1() {
        let test_vector = vec![0b0000_0001, 0b0000_0011, 0b0000_0011, 0b0000_0001];
        assert_eq!(get_normalized_distance(2, &test_vector), 1 as f64);
    }

    #[test]
    fn test_normalized_distance_2() {
        let test_vector = vec![
            0b0000_0001,
            0b0000_0011,
            0b0000_0011,
            0b0000_0011,
            0b0000_0001,
            0b0011_1111,
        ];
        assert_eq!(get_normalized_distance(3, &test_vector), 2 as f64);
    }

    #[test]
    fn test_case_for_base_64_decode() {
        let input = "TWFuIGlzIGRpc3Rpbmd1aXNoZWQsIG5vdCBvbmx5IGJ5IGhpcyByZWFzb24sIGJ1dCBieSB0aGlzIHNpbmd1bGFyIHBhc3Npb24gZnJvbSBvdGhlciBhbmltYWxzLCB3aGljaCBpcyBhIGx1c3Qgb2YgdGhlIG1pbmQsIHRoYXQgYnkgYSBwZXJzZXZlcmFuY2Ugb2YgZGVsaWdodCBpbiB0aGUgY29udGludWVkIGFuZCBpbmRlZmF0aWdhYmxlIGdlbmVyYXRpb24gb2Yga25vd2xlZGdlLCBleGNlZWRzIHRoZSBzaG9ydCB2ZWhlbWVuY2Ugb2YgYW55IGNhcm5hbCBwbGVhc3VyZS4=";
        let expected_result = "Man is distinguished, not only by his reason, but by this singular passion from other animals, which is a lust of the mind, that by a perseverance of delight in the continued and indefatigable generation of knowledge, exceeds the short vehemence of any carnal pleasure.";
        let result = decode_base_64(input);

        assert_eq!(utf8_to_string(&result), expected_result);
    }

    #[test]
    fn test_distance(){
        let phrase_1 = "this is a test";
        let phrase_2 = "wokka wokka!!!";

        assert_eq!(hamming_distance_for_strings(phrase_1, phrase_2), 37);
        assert_eq!(hamming_distance_per_byte(0b0000_0001, 0b0000_0001), 0);
        assert_eq!(hamming_distance_per_byte(0b0000_0011, 0b0000_0001), 1);
        assert_eq!(hamming_distance_per_byte(0b0000_0010, 0b0000_0001), 2);
    }

    #[test]
    fn test_partitioning() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let expected_output = vec![vec![1, 4, 7, 10], vec![2, 5, 8], vec![3, 6, 9]];
        let result = partition_buckets(3, &input);
        assert_eq!(result, expected_output);
    }

    #[test]
    fn find_key_size_test() {
        let encrypted_string = "CzY3JyorLmNiLC5paSojaToqPGMkIC1iPWM0PComImMkJydlJyooKy8gaQplLixlKjEkMzplPisgJ2MMaSsgKDFlKGMmMC4nKC8=";
        let expected_key_size: u8 = 3;
        let decoded_bytes: Vec<u8> = decode_base_64(encrypted_string);
        let key_size: u8 = find_key_size(&decoded_bytes);
        assert_eq!(key_size, expected_key_size);
    }

    #[test]
    fn decrypt_repeating_key_test_1() {
        let encrypted_string = "CzY3JyorLmNiLC5paSojaToqPGMkIC1iPWM0PComImMkJydlJyooKy8gaQplLixlKjEkMzplPisgJ2MMaSsgKDFlKGMmMC4nKC8=";
        let expected_result = "Burning 'em, if you ain't quick and nimble I go crazy when I hear a cymbal";
        assert_eq!(expected_result, decrypt_base_64_string(encrypted_string));
    }
}
