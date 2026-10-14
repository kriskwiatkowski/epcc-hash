use crate::Permute;

const ROUNDS: usize = 24;

const RHO: [u32; 25] = [
    0, 1, 62, 28, 27, //
    36, 44, 6, 55, 20, //
    3, 10, 43, 25, 39, //
    41, 45, 15, 21, 8, //
    18, 2, 61, 56, 14,
];

// Round constants in plain 64-bit lanes.
const RC: [u64; ROUNDS] = [
    0x0000_0000_0000_0001,
    0x0000_0000_0000_8082,
    0x8000_0000_0000_808a,
    0x8000_0000_8000_8000,
    0x0000_0000_0000_808b,
    0x0000_0000_8000_0001,
    0x8000_0000_8000_8081,
    0x8000_0000_0000_8009,
    0x0000_0000_0000_008a,
    0x0000_0000_0000_0088,
    0x0000_0000_8000_8009,
    0x0000_0000_8000_000a,
    0x0000_0000_8000_808b,
    0x8000_0000_0000_008b,
    0x8000_0000_0000_8089,
    0x8000_0000_0000_8003,
    0x8000_0000_0000_8002,
    0x8000_0000_0000_0080,
    0x0000_0000_0000_800a,
    0x8000_0000_8000_000a,
    0x8000_0000_8000_8081,
    0x8000_0000_0000_8080,
    0x0000_0000_8000_0001,
    0x8000_0000_8000_8008,
];

/// Apply the Keccak-f\[1600\] permutation to `state` (24 rounds of
/// Theta, Rho, Pi, Chi, Iota).
impl Permute for [u64; 25] {
    fn permute(&mut self) {
        let state = self;
        for rc in RC {
            // Theta
            let mut c = [0u64; 5];
            for x in 0..5 {
                c[x] = state[x]
                    ^ state[x + 5]
                    ^ state[x + 10]
                    ^ state[x + 15]
                    ^ state[x + 20];
            }
            let mut d = [0u64; 5];
            for x in 0..5 {
                d[x] = c[(x + 4) % 5] ^ c[(x + 1) % 5].rotate_left(1);
            }
            for y in 0..5 {
                for x in 0..5 {
                    state[x + 5 * y] ^= d[x];
                }
            }

            // Rho + Pi
            let mut b = [0u64; 25];
            for y in 0..5 {
                for x in 0..5 {
                    let new_x = y;
                    let new_y = (2 * x + 3 * y) % 5;
                    b[new_x + 5 * new_y] =
                        state[x + 5 * y].rotate_left(RHO[x + 5 * y]);
                }
            }

            // Chi
            for y in 0..5 {
                for x in 0..5 {
                    state[x + 5 * y] = b[x + 5 * y]
                        ^ (!b[(x + 1) % 5 + 5 * y] & b[(x + 2) % 5 + 5 * y]);
                }
            }

            // Iota
            state[0] ^= rc;
        }
    }
}
