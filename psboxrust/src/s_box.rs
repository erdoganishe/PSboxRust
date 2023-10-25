pub fn s_box_encrypt(input: u8, s_table: &[u8; 256]) -> u8 {
    s_table[input as usize]
}

pub fn s_box_decrypt(input: u8, inverse_s_table: &[u8; 256]) -> u8 {
    inverse_s_table[input as usize]
}


pub fn generate_s_table() -> [u8; 256] {
    let mut s_table: [u8; 256] = [0; 256];
    

    for i in 0..256 {
        s_table[i] = (i^ 0x55).try_into().unwrap() ;
    }
    
    s_table
}
pub fn generate_inverse_s_table(s_table: &[u8; 256]) -> [u8; 256] {
    let mut inverse_s_table: [u8; 256] = [0; 256];

    for i in 0..256 {
        let direct_value = s_table[i];
        inverse_s_table[direct_value as usize] = i as u8;
    }

    inverse_s_table
}