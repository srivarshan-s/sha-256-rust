use std::io;
use std::io::Write;

pub fn read_line() -> String {
    io::stdout().flush().unwrap();
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn convert_to_binary(string: &str) -> String {
    let mut binary_string: String = String::new();
    for ele in string.to_string().into_bytes() {
        // Checking whether ele is 0
        if ele == 32 {
            binary_string += "0";
        }
        binary_string += &format!("0{:b} ", ele);
    }
    binary_string
}

pub fn append_one(mut string: String) -> String {
    string += "1";
    string
}

pub fn find_length(length: usize) -> usize {
    let mut idx: usize = 64;
    loop {
        if length < (idx - 8) {
            return idx - 8;
        }
        idx += 64;
    }
}

pub fn pad_zeros(mut string: String) -> String {
    string += "0000000 ";
    let mut string_array: Vec<&str> = string.split(' ').collect();
    string_array.pop();
    let array_length = string_array.len();
    let new_array_length = find_length(array_length);
    for _idx in array_length..new_array_length {
        string += "00000000 ";
    }
    string
}

pub fn append_be(mut string: String, initial_string: String) -> String {
    let length: usize = initial_string.len() * 8;
    println!("Length of original string = {}", length);
    println!("Length in binary = {}", &format!("{:b}", length));
    let mut binary_length: String = String::new();
    binary_length += &format!("{:b}", length);
    if binary_length.len() != 8 {
        binary_length = "0".to_string() + &binary_length;
    }
    for _idx in 0..7 {
        binary_length = "00000000 ".to_owned() + &binary_length;
    }
    string += &binary_length;
    string
}

pub fn convert_to_32_bit(string: &str) -> String {
    let combined_string: String = string.split(' ').collect();
    let n = 32;
    let mut combined_string = combined_string
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if i != 0 && i % n == 0 {
                Some(' ')
            } else {
                None
            }
            .into_iter()
            .chain(std::iter::once(c))
        })
        .collect::<String>();
    combined_string += " ";
    combined_string
}

pub fn add_48_words(mut string: String) -> String {
    for _idx in 0..48 {
        string += "00000000000000000000000000000000 ";
    }
    string
}

pub fn modify_end_index(string: String) -> String {
    let mut string_array: Vec<&str> = string.split(' ').collect();
    string_array.pop();
    let mut temp_heap: Vec<String> = Vec::new();
    for i in string_array {
        temp_heap.push(i.to_string());
    }
    for idx in 16..64 {
        println!("Iteration {}", idx - 15);
        let mut s0 = right_rotate(temp_heap[idx - 15].to_string(), 7);
        s0 = xor(&s0, &right_rotate(temp_heap[idx - 15].to_string(), 18));
        s0 = xor(&s0, &right_shift(temp_heap[idx - 15].to_string(), 3));
        let mut s1 = right_rotate(temp_heap[idx - 2].to_string(), 17);
        s1 = xor(&s1, &right_rotate(temp_heap[idx - 2].to_string(), 19));
        s1 = xor(&s1, &right_shift(temp_heap[idx - 2].to_string(), 10));
        let temp_val1: String = temp_heap[idx - 16].to_string();
        let temp_val2: String = temp_heap[idx - 7].to_string();
        let mut final_val: String = binary_add(&temp_val1, &s0);
        final_val = binary_add(&final_val, &temp_val2);
        final_val = binary_add(&final_val, &s1);
        if idx == 16 {
            println!();
            println!("w[1] rightrotate 7:");
            println!(
                "{} -> {}",
                temp_heap[idx - 15],
                right_rotate(temp_heap[idx - 15].to_string(), 7)
            );
            println!("w[1] rightrotate 18:");
            println!(
                "{} -> {}",
                temp_heap[idx - 15],
                right_rotate(temp_heap[idx - 15].to_string(), 18)
            );
            println!("w[1] rightshift 3:");
            println!(
                "{} -> {}",
                temp_heap[idx - 15],
                right_shift(temp_heap[idx - 15].to_string(), 3)
            );
            println!();
            println!(
                "s0 = {} XOR {} XOR {}",
                right_rotate(temp_heap[idx - 15].to_string(), 7),
                right_rotate(temp_heap[idx - 15].to_string(), 18),
                right_shift(temp_heap[idx - 15].to_string(), 3)
            );
            println!("s0 = {}", s0);
            println!();
            println!("w[14] rightrotate 17:");
            println!(
                "{} -> {}",
                temp_heap[idx - 2],
                right_rotate(temp_heap[idx - 2].to_string(), 17)
            );
            println!("w[14] rightrotate 19:");
            println!(
                "{} -> {}",
                temp_heap[idx - 2],
                right_rotate(temp_heap[idx - 2].to_string(), 19)
            );
            println!("w[14] rightshift 10:");
            println!(
                "{} -> {}",
                temp_heap[idx - 2],
                right_shift(temp_heap[idx - 2].to_string(), 10)
            );
            println!();
            println!(
                "s1 = {} XOR {} XOR {}",
                right_rotate(temp_heap[idx - 2].to_string(), 17),
                right_rotate(temp_heap[idx - 2].to_string(), 19),
                right_shift(temp_heap[idx - 2].to_string(), 10)
            );
            println!("s1 = {}", s1);
            println!();
            println!("w[16] = w[0] + s0 + w[9] + s1");
            println!(
                "w[16] = {} + {} + {} + {}",
                temp_heap[idx - 16],
                s0,
                temp_heap[idx - 7],
                s1
            );
            println!("w[16] = {}", final_val);
            println!();
        }
        println!("s0 = {} s1 = {} w[{}] = {}", s0, s1, idx, final_val);
        println!();
        temp_heap[idx] = final_val;
    }
    let mut return_string: String = "".to_string();
    for i in temp_heap {
        return_string += &i;
        return_string += " ";
    }
    return_string
}

pub fn compression(
    string: String,
    hash_values: Vec<String>,
    round_constants: Vec<String>,
) -> (String, Vec<String>, Vec<String>) {
    let mut compress_var: Vec<String> = Vec::new();
    let mut string_array: Vec<&str> = string.split(' ').collect();
    string_array.pop();
    for val in &hash_values {
        compress_var.push(hex_to_binary(&val.to_string()))
    }
    for idx in 0..64 {
        println!("\nIteration {}", idx + 1);
        if idx == 0 {
            println!();
            println! {"a = {} = {}", binary_to_hex(&compress_var[0]), compress_var[0]};
            println! {"b = {} = {}", binary_to_hex(&compress_var[1]), compress_var[1]};
            println! {"c = {} = {}", binary_to_hex(&compress_var[2]), compress_var[2]};
            println! {"d = {} = {}", binary_to_hex(&compress_var[3]), compress_var[3]};
            println! {"e = {} = {}", binary_to_hex(&compress_var[4]), compress_var[4]};
            println! {"f = {} = {}", binary_to_hex(&compress_var[5]), compress_var[5]};
            println! {"g = {} = {}", binary_to_hex(&compress_var[6]), compress_var[6]};
            println! {"h = {} = {}", binary_to_hex(&compress_var[7]), compress_var[7]};
            println!();
        }
        let mut s1 = right_rotate(compress_var[4].to_string(), 6);
        s1 = xor(&s1, &right_rotate(compress_var[4].to_string(), 11));
        s1 = xor(&s1, &right_rotate(compress_var[4].to_string(), 25));
        if idx == 0 {
            println!("e rightrotate 6");
            println!(
                "{} -> {}",
                compress_var[4],
                right_rotate(compress_var[4].to_string(), 6)
            );
            println!("e rightrotate 11");
            println!(
                "{} -> {}",
                compress_var[4],
                right_rotate(compress_var[4].to_string(), 11)
            );
            println!("e rightrotate 25");
            println!(
                "{} -> {}",
                compress_var[4],
                right_rotate(compress_var[4].to_string(), 25)
            );
            println!(
                "S1 = {} XOR {} XOR {}",
                right_rotate(compress_var[4].to_string(), 6),
                right_rotate(compress_var[4].to_string(), 11),
                right_rotate(compress_var[4].to_string(), 25)
            );
            println!("S1 = {}", s1);
            println!();
        }
        let mut ch = binary_and(&compress_var[4].to_string(), &compress_var[5].to_string());
        ch = xor(
            &ch,
            &binary_and(
                &binary_not(&compress_var[4].to_string()),
                &compress_var[6].to_string(),
            ),
        );
        if idx == 0 {
            println!("e AND f");
            println!(
                "{} AND {} -> {}",
                compress_var[4],
                compress_var[5],
                binary_and(&compress_var[4].to_string(), &compress_var[5].to_string())
            );
            println!("NOT e");
            println!(
                "{} -> {}",
                compress_var[4],
                binary_not(&compress_var[4].to_string())
            );
            println!("(NOT e) AND g");
            println!(
                "{} AND {} -> {}",
                binary_not(&compress_var[4].to_string()),
                compress_var[6],
                binary_and(
                    &binary_not(&compress_var[4].to_string()),
                    &compress_var[6].to_string(),
                )
            );
            println!("ch = (e AND f) XOR ((NOT e) AND G)");
            println!(
                "ch = {} XOR {}",
                binary_and(&compress_var[4].to_string(), &compress_var[5].to_string()),
                binary_and(
                    &binary_not(&compress_var[4].to_string()),
                    &compress_var[6].to_string(),
                )
            );
            println!("ch = {}", ch);
            println!();
        }
        let mut temp1 = binary_add(&compress_var[7].to_string(), &s1);
        temp1 = binary_add(&temp1, &ch);
        temp1 = binary_add(&temp1, &hex_to_binary(&round_constants[idx].to_string()));
        temp1 = binary_add(&temp1, &string_array[idx].to_string());
        if idx == 0 {
            println!("temp1 = h + S1 + ch + k[0] + w[0]");
            println!(
                "temp1 = {} + {} + {} + {} + {}",
                compress_var[7],
                s1,
                ch,
                hex_to_binary(&round_constants[idx].to_string()),
                string_array[idx]
            );
            println!("temp1 = {}", temp1);
            println!();
        }
        if idx != 0 {
            println!("S1 = {} ch = {} temp1 = {}", s1, ch, temp1);
        }
        let mut s0 = right_rotate(compress_var[0].to_string(), 2);
        s0 = xor(&s0, &right_rotate(compress_var[0].to_string(), 13));
        s0 = xor(&s0, &right_rotate(compress_var[0].to_string(), 22));
        if idx == 0 {
            println!("a rightrotate 2");
            println!(
                "{} -> {}",
                compress_var[0],
                right_rotate(compress_var[0].to_string(), 2)
            );
            println!("a rightrotate 13");
            println!(
                "{} -> {}",
                compress_var[0],
                right_rotate(compress_var[0].to_string(), 13)
            );
            println!("a rightrotate 22");
            println!(
                "{} -> {}",
                compress_var[0],
                right_rotate(compress_var[0].to_string(), 22)
            );
            println!(
                "S0 = {} XOR {} XOR {}",
                right_rotate(compress_var[0].to_string(), 2),
                right_rotate(compress_var[0].to_string(), 13),
                right_rotate(compress_var[0].to_string(), 22)
            );
            println!("S0 = {}", s0);
            println!();
        }
        let mut maj = binary_and(&compress_var[0].to_string(), &compress_var[1].to_string());
        maj = xor(
            &maj,
            &binary_and(&compress_var[0].to_string(), &compress_var[2].to_string()),
        );
        maj = xor(
            &maj,
            &binary_and(&compress_var[1].to_string(), &compress_var[2].to_string()),
        );
        if idx == 0 {
            println!("a AND b");
            println!(
                "{} AND {} -> {}",
                compress_var[0],
                compress_var[1],
                binary_and(&compress_var[0].to_string(), &compress_var[1].to_string())
            );
            println!("a AND c");
            println!(
                "{} AND {} -> {}",
                compress_var[0],
                compress_var[2],
                binary_and(&compress_var[0].to_string(), &compress_var[2].to_string())
            );
            println!("b AND c");
            println!(
                "{} AND {} -> {}",
                compress_var[1],
                compress_var[2],
                binary_and(&compress_var[1].to_string(), &compress_var[2].to_string())
            );
            println!("maj = (a AND b) XOR (a AND c) XOR (b AND c)");
            println!(
                "maj = {} XOR {} XOR {}",
                binary_and(&compress_var[0].to_string(), &compress_var[1].to_string()),
                binary_and(&compress_var[0].to_string(), &compress_var[2].to_string()),
                binary_and(&compress_var[1].to_string(), &compress_var[2].to_string())
            );
            println!("maj = {}", maj);
            println!();
        }
        let temp2 = binary_add(&s0, &maj);
        if idx == 0 {
            println!("temp2 = S0 + maj");
            println!("temp2 = {} + {}", s0, maj);
            println!("temp2 = {}", temp2);
            println!();
        }
        if idx != 0 {
            println!("S0 = {} maj = {} temp2 = {}", s0, maj, temp2);
        }
        if idx == 0 {
            println!("h = {}", compress_var[6]);
            println!("g = {}", compress_var[5]);
            println!("f = {}", compress_var[4]);
            println!("e = {} + {}", compress_var[3], temp1);
            println!("e = {}", binary_add(&compress_var[3].to_string(), &temp1));
            println!("d = {}", compress_var[2]);
            println!("c = {}", compress_var[1]);
            println!("b = {}", compress_var[0]);
            println!("a = {} + {}", temp1, temp2);
            println!("a = {}", binary_add(&temp1, &temp2));
            println!();
        }
        compress_var[7] = compress_var[6].to_string();
        compress_var[6] = compress_var[5].to_string();
        compress_var[5] = compress_var[4].to_string();
        compress_var[4] = binary_add(&compress_var[3].to_string(), &temp1);
        compress_var[3] = compress_var[2].to_string();
        compress_var[2] = compress_var[1].to_string();
        compress_var[1] = compress_var[0].to_string();
        compress_var[0] = binary_add(&temp1, &temp2);
        if idx != 0 {
            println!(
                "a = {} b = {} c = {} d = {}",
                compress_var[0], compress_var[1], compress_var[2], compress_var[3]
            );
            println!(
                "e = {} f = {} g = {} h = {}",
                compress_var[4], compress_var[5], compress_var[6], compress_var[7]
            );
        }
    }
    (string, hash_values, compress_var)
}

pub fn modify_final_values(hash_values: Vec<String>, compress_var: Vec<String>) -> Vec<String> {
    let mut final_values: Vec<String> = Vec::new();
    for i in 0..hash_values.len() {
        final_values.push(binary_add(
            &hex_to_binary(&hash_values[i]),
            &compress_var[i],
        ));
    }
    final_values
}

pub fn concat_final_hash(final_values: Vec<String>) -> String {
    let mut final_hash: String = String::new();
    /* for idx in 0..final_values.len() {
        final_hash += &final_values[idx];
    } */
    for val in &final_values {
        final_hash += val;
    }
    binary_to_hex(&final_hash)[2..].to_string()
}

fn binary_and(val1: &str, val2: &str) -> String {
    let mut result: String = String::new();
    for idx in 0..val1.len() {
        if val1.chars().nth(idx).unwrap() == '1' && val2.chars().nth(idx).unwrap() == '1' {
            result += "1";
        } else {
            result += "0";
        }
    }
    result
}

fn binary_not(val1: &str) -> String {
    let mut result: String = String::new();
    for idx in 0..val1.len() {
        if val1.chars().nth(idx).unwrap() == '0' {
            result += "1";
        } else {
            result += "0";
        }
    }
    result
}

pub fn init_hash_values() -> Vec<String> {
    let hash_values = vec![
        "0x6a09e667".to_string(),
        "0xbb67ae85".to_string(),
        "0x3c6ef372".to_string(),
        "0xa54ff53a".to_string(),
        "0x510e527f".to_string(),
        "0x9b05688c".to_string(),
        "0x1f83d9ab".to_string(),
        "0x5be0cd19".to_string(),
    ];
    hash_values
}

pub fn hex_to_binary(hex_string: &str) -> String {
    // hex_string[2..].chars().map(to_binary).collect()
    let temp_string = hex_string[2..].to_string();
    let temp_array = temp_string.chars();
    let mut binary_string: String = String::new();
    for ch in temp_array {
        match ch {
            '0' => binary_string += "0000",
            '1' => binary_string += "0001",
            '2' => binary_string += "0010",
            '3' => binary_string += "0011",
            '4' => binary_string += "0100",
            '5' => binary_string += "0101",
            '6' => binary_string += "0110",
            '7' => binary_string += "0111",
            '8' => binary_string += "1000",
            '9' => binary_string += "1001",
            'a' => binary_string += "1010",
            'b' => binary_string += "1011",
            'c' => binary_string += "1100",
            'd' => binary_string += "1101",
            'e' => binary_string += "1110",
            'f' => binary_string += "1111",
            _ => binary_string += "",
        }
    }
    binary_string
}

pub fn binary_to_hex(binary_string: &str) -> String {
    let n = 4;
    let combined_string = binary_string
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if i != 0 && i % n == 0 {
                Some(' ')
            } else {
                None
            }
            .into_iter()
            .chain(std::iter::once(c))
        })
        .collect::<String>();
    let string_array: Vec<&str> = combined_string.split(' ').collect();
    let mut hex_string: String = "0x".to_string();
    for ele in string_array {
        match ele {
            "0000" => hex_string += "0",
            "0001" => hex_string += "1",
            "0010" => hex_string += "2",
            "0011" => hex_string += "3",
            "0100" => hex_string += "4",
            "0101" => hex_string += "5",
            "0110" => hex_string += "6",
            "0111" => hex_string += "7",
            "1000" => hex_string += "8",
            "1001" => hex_string += "9",
            "1010" => hex_string += "a",
            "1011" => hex_string += "b",
            "1100" => hex_string += "c",
            "1101" => hex_string += "d",
            "1110" => hex_string += "e",
            "1111" => hex_string += "f",
            _ => hex_string += "",
        };
    }
    hex_string
}

fn right_rotate(mut string: String, num: usize) -> String {
    if num == 0 {
        return string;
    }
    let last_ele = string.pop().unwrap();
    string = last_ele.to_string() + &string;
    right_rotate(string, num - 1)
}

fn right_shift(mut string: String, num: usize) -> String {
    if num == 0 {
        return string;
    }
    string.pop();
    string = "0".to_string() + &string;
    right_shift(string, num - 1)
}

fn xor(binary_val_1: &str, binary_val_2: &str) -> String {
    let mut array_1: Vec<bool> = Vec::new();
    let mut array_2: Vec<bool> = Vec::new();
    for idx in 0..binary_val_1.len() {
        if binary_val_1.chars().nth(idx).unwrap() == '1' {
            array_1.push(true);
        } else {
            array_1.push(false);
        }
    }
    for idx in 0..binary_val_2.len() {
        if binary_val_2.chars().nth(idx).unwrap() == '1' {
            array_2.push(true);
        } else {
            array_2.push(false);
        }
    }
    let mut string_array = "".to_string();
    for idx in 0..array_1.len() {
        if array_1[idx] ^ array_2[idx] {
            string_array += "1";
        } else {
            string_array += "0";
        }
    }
    string_array
}

fn binary_add(val1: &str, val2: &str) -> String {
    let int1 = binary_to_int(val1);
    let int2 = binary_to_int(val2);
    let mut val_int = int1 + int2;
    if val_int > pow(2, 32) {
        val_int -= pow(2, 32);
    }
    let mut val_string: String = format!("{:b}", val_int);
    while val_string.len() < 32 {
        val_string = "0".to_string() + &val_string;
    }
    val_string
}

fn binary_to_int(binary_string: &str) -> usize {
    let int_val = isize::from_str_radix(binary_string, 2).unwrap();
    int_val as usize
}

fn pow(num: usize, val: usize) -> usize {
    if val == 1 {
        num
    } else {
        num * pow(num, val - 1)
    }
}

pub fn init_round_constants() -> Vec<String> {
    let round_constants = vec![
        "0x428a2f98".to_string(),
        "0x71374491".to_string(),
        "0xb5c0fbcf".to_string(),
        "0xe9b5dba5".to_string(),
        "0x3956c25b".to_string(),
        "0x59f111f1".to_string(),
        "0x923f82a4".to_string(),
        "0xab1c5ed5".to_string(),
        "0xd807aa98".to_string(),
        "0x12835b01".to_string(),
        "0x243185be".to_string(),
        "0x550c7dc3".to_string(),
        "0x72be5d74".to_string(),
        "0x80deb1fe".to_string(),
        "0x9bdc06a7".to_string(),
        "0xc19bf174".to_string(),
        "0xe49b69c1".to_string(),
        "0xefbe4786".to_string(),
        "0x0fc19dc6".to_string(),
        "0x240ca1cc".to_string(),
        "0x2de92c6f".to_string(),
        "0x4a7484aa".to_string(),
        "0x5cb0a9dc".to_string(),
        "0x76f988da".to_string(),
        "0x983e5152".to_string(),
        "0xa831c66d".to_string(),
        "0xb00327c8".to_string(),
        "0xbf597fc7".to_string(),
        "0xc6e00bf3".to_string(),
        "0xd5a79147".to_string(),
        "0x06ca6351".to_string(),
        "0x14292967".to_string(),
        "0x27b70a85".to_string(),
        "0x2e1b2138".to_string(),
        "0x4d2c6dfc".to_string(),
        "0x53380d13".to_string(),
        "0x650a7354".to_string(),
        "0x766a0abb".to_string(),
        "0x81c2c92e".to_string(),
        "0x92722c85".to_string(),
        "0xa2bfe8a1".to_string(),
        "0xa81a664b".to_string(),
        "0xc24b8b70".to_string(),
        "0xc76c51a3".to_string(),
        "0xd192e819".to_string(),
        "0xd6990624".to_string(),
        "0xf40e3585".to_string(),
        "0x106aa070".to_string(),
        "0x19a4c116".to_string(),
        "0x1e376c08".to_string(),
        "0x2748774c".to_string(),
        "0x34b0bcb5".to_string(),
        "0x391c0cb3".to_string(),
        "0x4ed8aa4a".to_string(),
        "0x5b9cca4f".to_string(),
        "0x682e6ff3".to_string(),
        "0x748f82ee".to_string(),
        "0x78a5636f".to_string(),
        "0x84c87814".to_string(),
        "0x8cc70208".to_string(),
        "0x90befffa".to_string(),
        "0xa4506ceb".to_string(),
        "0xbef9a3f7".to_string(),
        "0xc67178f2".to_string(),
    ];
    round_constants
}
