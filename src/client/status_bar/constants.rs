// HP_PERCENTAGE_MAPPER maps pixel coordinates (x, y) on the lifebar and their RGB colors to corresponding HP percentages.
// [([usize; 2], (u8, u8, u8), u32); 11] format ensures each tuple holds lifebar pixel coordinates, expected color, and the resulting HP value.
pub const HP_PERCENTAGE_MAPPER: [([usize; 2], (u8, u8, u8), u32); 11] = [
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
];
