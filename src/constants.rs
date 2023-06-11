/// Used in creating the autotile tileset.
/// 20 x 10; which turns into a 10 x 5 grid;
pub const AUTOTILE_REORDERING: [[u8; 4]; 48] = [
	// first row
	[26, 27, 32, 32],
	[30, 35, 6, 7],
	[15, 1, 45, 7],
	[4, 35, 45, 7],
	[40, 41, 46, 47],
	[0, 14, 6, 44],
	[30, 5, 6, 44],
	[36, 37, 42, 43],
	// second row
	[14, 15, 44, 45],
	[4, 5, 44, 45],
	[26, 5, 44, 45],
	[4, 27, 44, 45],
	[26, 27, 44, 45],
	[0, 1, 30, 35],
	[24, 29, 30, 35],
	[15, 1, 10, 35],
	// third row
	[4, 29, 10, 35],
	[26, 29, 10, 35],
	[0, 14, 24, 11],
	[24, 5, 30, 11],
	[24, 22, 30, 11],
	[14, 15, 10, 11],
	[4, 5, 10, 11],
	[26, 5, 10, 33],
	// forth row
	[4, 27, 10, 11],
	[26, 27, 10, 11],
	[16, 17, 22, 23],
	[4, 29, 32, 35],
	[26, 29, 32, 35],
	[14, 15, 32, 11],
	[4, 5, 32, 11],
	[26, 5, 32, 11],
	// fifth row
	[4, 27, 32, 11],
	[26, 27, 32, 11],
	[12, 13, 18, 19],
	[24, 5, 30, 33],
	[24, 27, 30, 33],
	[14, 15, 10, 33],
	[4, 5, 10, 33],
	[26, 5, 10, 33],
	// sixth row
	[4, 27, 10, 33],
	[26, 27, 10, 33],
	[14, 15, 32, 33],
	[4, 5, 32, 33],
	[26, 5, 32, 33],
	[4, 27, 32, 33],
	[26, 27, 32, 33],
	[0, 1, 6, 7],
];

/// Used for selecting a sprite number _after_ having run the marching squares
/// algorithm. 0 is unused (as in impossible to get to), some numbers are still superfluous but
/// I can't be bothered to remove them
pub const MARCHING_TILES: [u8; 256] = [
	47, 1, 1, 3, 4, 5, 6, 7, 2, 9, 3, 4, 12, 13, 14, 15, 5, 17, 6, 19, 20, 21, 7, 23, 8, 25, 9, 10,
	28, 29, 11, 12, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 0, 0, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 14, 0, 0, 0, 0, 0, 15, 0, 16, 17, 0, 0, 0, 0, 18, 0, 19,
	0, 0, 0, 20, 0, 21, 0, 22, 23, 0, 0, 24, 25, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 27, 28, 0, 0, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 0, 29, 0, 30, 31, 0, 0, 32, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	0, 0, 34, 0, 35, 0, 0, 0, 36, 0, 37, 0, 38, 39, 0, 0, 40, 41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 0, 43, 44, 0, 0, 45, 46,
];
