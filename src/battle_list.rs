use ndarray::Array2;


#[inline(always)]
pub fn get_creatures_count(battle_list_content: &Array2<u8>) -> u8 {
    let mut count = 0;
    let shape: &[usize] = battle_list_content.shape();
    let possible_count = shape[0] / 22;
    for i in 0..possible_count {
        let y = (i * 22) + 15;
        if battle_list_content[[y, 22]] != 0 {
            break;
        }
        count += 1;
    }
    count
}