
pub fn p_block(input: u8, p_table: &[u8; 8]) -> u8 {
    let mut result: u8 = 0;

    for i in 0..8 {
        let bit = (input >> p_table[i]) & 1;
        result |= bit << i;
    }

    result
}


pub fn p_block_decrypt(input: u8, p_table: &[u8; 8]) -> u8 {
    let mut reverse_table: [u8; 8] = [0; 8];

    for i in 0..8 {
        reverse_table[p_table[i] as usize] = i as u8;
    }

    let mut result: u8 = 0;

    for i in 0..8 {
        let bit = (input >> reverse_table[i]) & 1;
        result |= bit << i;
    }

    result
}