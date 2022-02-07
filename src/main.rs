mod lib;

fn main() {
    print!("String: ");
    let user_string: String = lib::read_line();
    println!("\n\n");

    println!("--------------------CONVERSION TO BINARY--------------------\n");
    let mut binary_user_string: String = lib::convert_to_binary(&user_string);
    println!("{}", binary_user_string);
    println!("\n\n");

    println!("--------------------APPEND 1 AT THE END--------------------\n");
    binary_user_string = lib::append_one(binary_user_string);
    println!("{}", binary_user_string);
    println!("\n\n");

    println!("--------------------PAD WITH ZEROS--------------------\n");
    binary_user_string = lib::pad_zeros(binary_user_string);
    println!("{}", binary_user_string);
    println!("\n\n");

    println!("--------------------APPEND BIG ENDIAN NUMBER AT END--------------------\n");
    binary_user_string = lib::append_be(binary_user_string, user_string);
    println!("{}", binary_user_string);
    println!("\n\n");

    println!("--------------------INITIALIZE HASH VALUES--------------------\n");
    let mut hash_values = lib::init_hash_values();
    for (i, val) in hash_values.iter().enumerate() {
        println!("h{} = {}", i, val);
    }
    println!("\n\n");

    println!("--------------------INITIALIZE ROUND CONSTANTS--------------------\n");
    let round_constants = lib::init_round_constants();
    for (i, val) in round_constants.iter().enumerate() {
        println!("k{} = {}", i, val);
    }
    println!("\n\n");

    println!("--------------------CONVERT TO 32 BIT--------------------\n");
    binary_user_string = lib::convert_to_32_bit(&binary_user_string);
    println!("{}", binary_user_string);
    println!("\n\n");

    println!("--------------------ADD 48 WORDS--------------------\n");
    binary_user_string = lib::add_48_words(binary_user_string);
    println!("{}", binary_user_string);
    println!("\n\n");

    println!("--------------------MODIFY END INDEX--------------------\n");
    binary_user_string = lib::modify_end_index(binary_user_string);
    println!("\n\n");

    println!("--------------------MESSAGE SCHEDULE--------------------\n");
    println!("{}", binary_user_string);
    println!("\n\n");

    println!("--------------------COMPRESSION--------------------");
    let compression_result = lib::compression(binary_user_string, hash_values, round_constants);
    hash_values = compression_result.1;
    let compress_var = compression_result.2;
    println!("\n\n");

    println!("--------------------VALUES AFTER COMPRESSION--------------------\n");
    for (i, item) in hash_values.iter().enumerate() {
        println!("h{} = {} = {}", i + 1, item, lib::hex_to_binary(item));
    }
    let temp_const: Vec<char> = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'k'].to_vec();
    println!();
    for i in 0..compress_var.len() {
        println!(
            "{} = {} = {}",
            temp_const[i],
            lib::binary_to_hex(&compress_var[i]),
            compress_var[i]
        );
    }
    println!("\n\n");

    println!("--------------------MODIFY FINAL VALUES--------------------\n");
    let len = hash_values.len();
    let final_values: Vec<String> = lib::modify_final_values(hash_values, compress_var);
    for i in 0..len {
        println!(
            "h{} = h{} + {} = {}",
            i + 1,
            i + 1,
            temp_const[i],
            final_values[i]
        );
    }
    println!("\n\n");

    println!("--------------------CONCATENATE FINAL HASH--------------------\n");
    let final_hash: String = lib::concat_final_hash(final_values);
    println!("Hash Values: {}", final_hash);
}
