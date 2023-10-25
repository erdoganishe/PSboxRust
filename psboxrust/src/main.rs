mod s_box;
mod p_box;
use s_box::generate_s_table;
use s_box::generate_inverse_s_table;
use s_box::s_box_encrypt;
use s_box::s_box_decrypt;
use p_box::p_block;



fn main() {
    let p_table: [u8; 8] = [2, 3, 0, 1, 4, 6, 5, 7];

    let input_data: u8 = 0b0101010;

    let permuted_data = p_block(input_data, &p_table);

    println!("Вхідні дані: {:08b}", input_data);
    println!("Після P-блоку: {:08b}", permuted_data);
}