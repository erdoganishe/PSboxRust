mod s_box;
use s_box::generate_s_table;
use s_box::generate_inverse_s_table;
use s_box::s_box_encrypt;
use s_box::s_box_decrypt;

// fn p_block(input: u32) -> u32 {
// }

fn main() {
    let s_table = generate_s_table();
    let inverse_s_table = generate_inverse_s_table(&s_table);

    let plaintext: u8 = 0b1111;

    let encrypted_data = s_box_encrypt(plaintext, &s_table);

    let decrypted_data = s_box_decrypt(encrypted_data, &inverse_s_table);

    println!("Вхідні дані: {:08b}", plaintext);
    println!("Зашифровані дані: {:08b}", encrypted_data);
    println!("Розшифровані дані: {:08b}", decrypted_data);
}