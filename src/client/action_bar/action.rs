pub struct Action;

impl Action {
    pub fn get_count(&self) -> Option<u32> {
        Some(100)
    }

    pub fn get_hotkey(&self) -> Option<&str> {
        Some("f1")
    }

    pub fn get_item_name(&self) -> Option<&str> {
        Some("strong health potion")
    }

    pub fn is_equipped(&self) -> bool {
        true
    }

    pub fn is_usable(&self) -> bool {
        if !self.is_equipped() {
            return false;
        }
        true
    }
}
