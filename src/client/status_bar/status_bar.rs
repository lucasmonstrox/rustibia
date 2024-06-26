use super::constants::*;
use ndarray::ArrayView3;

pub struct StatusBar<'a> {
    pub content: ArrayView3<'a, u8>,
}

impl StatusBar<'_> {
    pub fn get_hp_percentage(&self) -> u32 {
        for &([x, y], lifebar_pixel_color, hp) in &HP_PERCENTAGE_MAPPER {
            // Get the RGB values of the current pixel from the content
            // The values are retrieved using the x, y coordinates provided by the HP_PERCENTAGE_MAPPER
            let pixel_color = unsafe {
                (
                    *self.content.get_ptr([x, y, 0]).unwrap(),
                    *self.content.get_ptr([x, y, 1]).unwrap(),
                    *self.content.get_ptr([x, y, 2]).unwrap(),
                )
            };
            // Compare the extracted pixel color with the expected lifebar color from HP_PERCENTAGE_MAPPER
            // If they match, it means the health percentage (HP) associated with this color is present
            // and the expected percentage will be returned
            if pixel_color == lifebar_pixel_color {
                return hp;
            }
        }
        // If no matching color is found, return 0.
        // This is unlikely to happen because it would mean the character is dead, making this value not very useful.
        0
    }
}
