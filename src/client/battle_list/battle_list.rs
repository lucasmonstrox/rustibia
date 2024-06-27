use super::constants::*;
use ndarray::ArrayView3;

pub struct BattleList<'a> {
    pub content: ArrayView3<'a, u8>,
}

impl BattleList<'_> {
    pub fn get_creatures_count(&self) -> u32 {
        let mut creatures_count = 0;
        for creature_index in 0..self.get_max_creatures_count() {
            // The starting Y-coordinate (first vertical pixel) of the lifebar for the current creature slot
            let lifebar_start_y =
                ((creature_index * CREATURE_SLOT_HEIGHT) + LIFEBAR_START_POSITION.1) as usize;
            // Check if the first pixel of the lifebar is different from black (0, 0, 0), indicating the absence of a lifebar
            // Since the background is gray, it is safe to check only the first value of the pixel
            let no_creature_in_slot = unsafe {
                *self
                    .content
                    .get_ptr([lifebar_start_y, LIFEBAR_START_POSITION.0 as usize, 0])
                    .unwrap()
                    != BLACK_PIXEL_COLOR.0
            };
            if no_creature_in_slot {
                break;
            }
            creatures_count += 1;
        }
        creatures_count
    }

    pub fn get_max_creatures_count(&self) -> u32 {
        self.content.shape()[0] as u32 / CREATURE_SLOT_HEIGHT
    }

    pub fn has_creature_in_target(&self) -> bool {
        for i in 0..self.get_max_creatures_count() {
            let lifebar_start_y = (i * CREATURE_SLOT_HEIGHT) as usize;
            let has_creature_in_target = {
                // Obtain the RGB values of the first pixel of the slot to determine if it matches
                // the target color or the target hover color
                let pixel_color = unsafe {
                    (
                        *self.content.get_ptr([lifebar_start_y, 0, 0]).unwrap(),
                        *self.content.get_ptr([lifebar_start_y, 0, 1]).unwrap(),
                        *self.content.get_ptr([lifebar_start_y, 0, 2]).unwrap(),
                    )
                };
                // Check if the pixel color matches TARGET_COLOR (creature is targeted)
                // or TARGET_HOVER_COLOR (creature is targeted with highlight due to mouse hover)
                pixel_color == TARGET_COLOR || pixel_color == TARGET_HOVER_COLOR
            };
            if has_creature_in_target {
                return true;
            }
        }
        false
    }
}
