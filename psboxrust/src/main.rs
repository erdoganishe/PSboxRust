mod s_box;
mod p_box;
use s_box::{generate_s_table, generate_inverse_s_table, s_box_encrypt, s_box_decrypt};
use p_box::{p_block, p_block_decrypt};

fn main() {
    let p_table: [u8; 8] = [3, 5, 2, 1, 0, 6, 7, 4];

    let input_data: u8 = 0b11001101;

    let s_table = generate_s_table();
    let encrypted_data_s_box = s_box_encrypt(input_data, &s_table);

    let permuted_data = p_block(encrypted_data_s_box, &p_table);

    println!("Вхідні дані: {:08b}", input_data);
    println!("Зашифровані дані (S-блок): {:08b}", encrypted_data_s_box);
    println!("Після P-блоку: {:08b}", permuted_data);

    let decrypted_data_p_block = p_block_decrypt(permuted_data, &p_table);

    println!("Розшифровані дані (P-блок): {:08b}", decrypted_data_p_block);

    let decrypted_data_s_block = s_box_encrypt(decrypted_data_p_block, &s_table);

    println!("Розшифровані дані (S-блок): {:08b}", decrypted_data_s_block);
}