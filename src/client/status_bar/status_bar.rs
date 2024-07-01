use super::constants::{HP_PERCENTAGE_MAPPER, MANA_PERCENTAGE_MAPPER};
use ndarray::ArrayView3;

pub struct StatusBar<'a> {
    pub content: ArrayView3<'a, u8>,
}

impl StatusBar<'_> {
    // TODO: Enhance the function to return additional percentages at 5, 15, 25, 35, 45,
    // 55, 65, 75, 85, and 95 percent intervals for more detailed analysis.
    fn get_percentage(&self, mapper: &[([usize; 2], (u8, u8, u8), u32)]) -> u32 {
        for &([x, y], expected_color, percentage) in mapper {
            // Get the RGB values of the current pixel from the content
            // The values are retrieved using the x, y coordinates provided by the mapper
            let pixel_color = unsafe {
                (
                    *self.content.get_ptr([x, y, 0]).unwrap(),
                    *self.content.get_ptr([x, y, 1]).unwrap(),
                    *self.content.get_ptr([x, y, 2]).unwrap(),
                )
            };
            // Compare the extracted pixel color with the expected bar color from mapper
            // If they match, it means the value percentage associated with this color is present
            // and the expected percentage will be returned
            if pixel_color == expected_color {
                return percentage;
            }
        }
        // If no matching color is found, return 0.
        // For HP, this likely means the character is dead.
        // For mana, it means the character cannot cast spells or heal.
        0
    }

    pub fn get_hp_percentage(&self) -> u32 {
        self.get_percentage(&HP_PERCENTAGE_MAPPER)
    }

    pub fn get_mana_percentage(&self) -> u32 {
        self.get_percentage(&MANA_PERCENTAGE_MAPPER)
    }
}
