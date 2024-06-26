// HP_PERCENTAGE_MAPPER maps pixel coordinates (x, y) on the lifebar and their RGB colors to corresponding HP percentages.
// [([usize; 2], (u8, u8, u8), u32); 11] format ensures each tuple holds lifebar pixel coordinates, expected color, and the resulting HP value.
pub const HP_PERCENTAGE_MAPPER: [([usize; 2], (u8, u8, u8), u32); 12] = [
    ([5, 107], (100, 46, 49), 100),
    ([5, 98], (219, 79, 79), 90),
    ([5, 88], (219, 79, 79), 80),
    ([5, 79], (219, 79, 79), 70),
    ([5, 69], (219, 79, 79), 60),
    ([5, 60], (219, 79, 79), 50),
    ([5, 51], (219, 79, 79), 40),
    ([5, 41], (219, 79, 79), 30),
    ([5, 32], (219, 79, 79), 20),
    ([5, 22], (219, 79, 79), 10),
    ([5, 18], (219, 79, 79), 5),
    ([5, 14], (120, 61, 64), 1),
];

// MANA_PERCENTAGE_MAPPER maps pixel coordinates (x, y) on the manabar and their RGB colors to corresponding Mana percentages.
// [([usize; 2], (u8, u8, u8), u32); 11] format ensures each tuple holds lifebar pixel coordinates, expected color, and the resulting Mana value.
pub const MANA_PERCENTAGE_MAPPER: [([usize; 2], (u8, u8, u8), u32); 12] = [
    ([18, 107], (45, 45, 105), 100),
    ([18, 98], (83, 80, 218), 90),
    ([18, 88], (83, 80, 218), 80),
    ([18, 79], (83, 80, 218), 70),
    ([18, 69], (83, 80, 218), 60),
    ([18, 60], (83, 80, 218), 50),
    ([18, 51], (83, 80, 218), 40),
    ([18, 41], (83, 80, 218), 30),
    ([18, 32], (83, 80, 218), 20),
    ([18, 22], (83, 80, 218), 10),
    ([18, 18], (83, 80, 218), 5),
    ([18, 14], (61, 61, 125), 1),
];
