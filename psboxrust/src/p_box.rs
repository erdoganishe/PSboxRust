pub fn p_block(input: u32, p_table: &[u8; 32]) -> u32 {
    let mut result: u32 = 0;

    for i in 0..32 {
        let bit = (input >> p_table[i]) & 1;
        result |= bit << i;
    }

    result
}

pub fn p_block_decrypt(input: u32, p_table: &[u8; 32]) -> u32 {
    let mut reverse_table: [u8; 32] = [0; 32];

    for i in 0..32 {
        reverse_table[p_table[i] as usize] = i as u8;
    }

    let mut result: u32 = 0;

    for i in 0..32 {
        let bit = (input >> reverse_table[i]) & 1;
        result |= bit << i;
    }

    result
}
