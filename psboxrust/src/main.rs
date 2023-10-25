mod s_box;
mod p_box;
use s_box::{generate_s_table, generate_inverse_s_table, s_box_encrypt, s_box_decrypt};
use p_box::{p_block, p_block_decrypt};

fn main() {
    let p_table: [u8; 32] = [7, 14, 22, 1, 0, 6, 20, 16, 27, 15, 12, 10, 8, 23, 28, 5, 29, 2, 9, 26, 19, 25, 11, 18, 4, 13, 17, 21, 3, 24, 30, 31];

    let input_data: u32 = 0b11001101010111010001110010110100;

    let s_table = generate_s_table();
    let encrypted_data_s_box = s_box_encrypt(input_data, &s_table);

    let permuted_data = p_block(encrypted_data_s_box, &p_table);

    println!("Вхідні дані: {:032b}", input_data);
    println!("Зашифровані дані (S-блок): {:032b}", encrypted_data_s_box);
    println!("Після P-блоку: {:032b}", permuted_data);

    let decrypted_data_p_block = p_block_decrypt(permuted_data, &p_table);

    println!("Розшифровані дані (P-блок): {:032b}", decrypted_data_p_block);

    let decrypted_data_s_block = s_box_encrypt(decrypted_data_p_block, &s_table);

    println!("Розшифровані дані (S-блок): {:032b}", decrypted_data_s_block);
}
