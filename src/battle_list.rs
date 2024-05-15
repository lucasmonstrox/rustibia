use ndarray::Array3;

pub struct BattleList {
    pub content: Array3<u8>
}

impl BattleList {
    pub fn get_creatures_count(&self) -> u8 {
        let mut creatures_count = 0;
        for i in 0..(self.content.shape()[0] / 22) {
            let y: usize = (i * 22) + 15;
            if self.content[[y, 22, 0]] != 0 {
                break;
            }
            creatures_count += 1;
        }
        creatures_count
    }
}
