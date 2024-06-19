use ndarray::ArrayView3;

pub struct BattleList<'a> {
    pub content: ArrayView3<'a, u8>,
}

const CREATURE_SLOT_HEIGHT: u32 = 22;
const LIFEBAR_START_Y: u32 = 15;
const LIFEBAR_START_X: u32 = 22;
const TARGET_COLOR: (u8, u8, u8) = (255, 0, 0);
const TARGET_HOVER_COLOR: (u8, u8, u8) = (255, 128, 128);

impl BattleList<'_> {
    pub fn get_creatures_count(&self) -> u32 {
        let mut creatures_count = 0;
        for creature_index in 0..self.get_max_creatures_count() {
            let lifebar_start_y =
                ((creature_index * CREATURE_SLOT_HEIGHT) + LIFEBAR_START_Y) as usize;
            // Check if the first pixel of the lifebar is different from black (0, 0, 0), indicating the absence of a lifebar
            // Since the background is gray, it is safe to check only the first value of the pixel
            let lifebar_not_present = unsafe {
                *self
                    .content
                    .get_ptr([lifebar_start_y, LIFEBAR_START_X as usize, 0])
                    .unwrap()
                    != 0
            };
            if lifebar_not_present {
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
        let creatures_count = self.get_creatures_count();
        if creatures_count == 0 {
            return false;
        }
        for i in 0..creatures_count {
            let lifebar_start_y = (i * CREATURE_SLOT_HEIGHT) as usize;
            let has_creature_in_target = unsafe {
                // Obtain the RGB values of the pixel at the specific lifebar position
                // Accessing the first pixel of the lifebar in both X and Y axes
                let red = *self.content.get_ptr([lifebar_start_y, 0, 0]).unwrap();
                let green = *self.content.get_ptr([lifebar_start_y, 0, 1]).unwrap();
                let blue = *self.content.get_ptr([lifebar_start_y, 0, 2]).unwrap();
                let pixel_color = (red, green, blue);
                // Check if the pixel color matches TARGET_COLOR (creature is targeted)
                // or TARGET_HOVER_COLOR (creature is highlighted due to mouse hover)
                pixel_color == TARGET_COLOR || pixel_color == TARGET_HOVER_COLOR
            };
            if has_creature_in_target {
                return true;
            }
        }
        false
    }
}
