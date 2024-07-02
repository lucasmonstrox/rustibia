use super::constants::EQUIPPED_ACTION_COLOR;
use ndarray::ArrayView3;

pub struct Action<'a> {
    pub content: ArrayView3<'a, u8>,
}

impl Action<'_> {
    pub fn get_count(&self) -> Option<u32> {
        Some(100)
    }

    pub fn get_hotkey(&self) -> Option<&str> {
        Some("f1")
    }

    pub fn get_item_name(&self) -> Option<&str> {
        Some("strong health potion")
    }

    // TODO: Consider caching this method to avoid unnecessary recomputation on subsequent calls
    pub fn is_equipped(&self) -> bool {
        unsafe {
            *self.content.get_ptr([0, 0, 0]).unwrap() == EQUIPPED_ACTION_COLOR.0
                && *self.content.get_ptr([0, 0, 1]).unwrap() == EQUIPPED_ACTION_COLOR.1
                && *self.content.get_ptr([0, 0, 2]).unwrap() == EQUIPPED_ACTION_COLOR.2
        }
    }

    pub fn is_usable(&self) -> bool {
        true
    }
}
