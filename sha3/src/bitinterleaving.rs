#[cfg(target_arch = "arm")]
use crate::bitinterleaving_armv8m;

pub mod bitinterleaving_generic {
    pub fn conv_into_bi(r1: u32, r2: u32) -> (u32, u32) {
        let mut t = r1 & 0x5555_5555;
        t = (t | (t >> 1)) & 0x3333_3333;
        t = (t | (t >> 2)) & 0x0f0f_0f0f;
        t = (t | (t >> 4)) & 0x00ff_00ff;
        t |= t << 8;
        let mut s1 = (t >> 8) & 0x0000_ffff;

        t = r2 & 0x5555_5555;
        t = (t | (t >> 1)) & 0x3333_3333;
        t = (t | (t >> 2)) & 0x0f0f_0f0f;
        t = (t | (t >> 4)) & 0x00ff_00ff;
        t |= t >> 8;
        s1 ^= t << 16;

        t = r1 & 0xaaaa_aaaa;
        t = (t | (t << 1)) & 0xcccc_cccc;
        t = (t | (t << 2)) & 0xf0f0_f0f0;
        t = (t | (t << 4)) & 0xff00_ff00;
        t |= t << 8;
        let mut s2 = t >> 16;

        t = r2 & 0xaaaa_aaaa;
        t = (t | (t << 1)) & 0xcccc_cccc;
        t = (t | (t << 2)) & 0xf0f0_f0f0;
        t = (t | (t << 4)) & 0xff00_ff00;
        t |= t << 8;
        s2 ^= (t >> 16) << 16;

        (s1, s2)
    }

    pub fn conv_from_bi(r1: u32, r2: u32) -> (u32, u32) {
        let mut output_r1 = (r1 & 0x0000_ffff) | (r2 << 16);
        let mut output_r2 = (r2 & 0xffff_0000) | (r1 >> 16);

        output_r1 = unshuffle32(output_r1);
        output_r2 = unshuffle32(output_r2);

        (output_r1, output_r2)
    }

    fn unshuffle32(mut value: u32) -> u32 {
        let mut t: u32;

        t = (value ^ (value >> 8)) & 0x0000_ff00;
        value ^= t;
        value ^= t << 8;

        t = (value ^ (value >> 4)) & 0x00f0_00f0;
        value ^= t;
        value ^= t << 4;

        t = (value ^ (value >> 2)) & 0x0c0c_0c0c;
        value ^= t;
        value ^= t << 2;

        t = (value ^ (value >> 1)) & 0x2222_2222;
        value ^= t;
        value ^ (t << 1)
    }
}

pub fn conv_into_bi(r1: u32, r2: u32) -> (u32, u32) {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "arm")] {
            let mut s1 = 0;
            let mut s2 = 0;
            let mut t = 0;
            unsafe {
                bitinterleaving_armv8m::conv_into_bi(r1, r2, &mut s1, &mut s2, &mut t);
            }
            (s1, s2)
        } else {
            bitinterleaving_generic::conv_into_bi(r1, r2)
        }
    }
}

pub fn conv_from_bi(r1: u32, r2: u32) -> (u32, u32) {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "arm")]
        {
            let mut t = 0;
            let mut output_r1 = r1;
            let mut output_r2 = r2;
            unsafe {
                bitinterleaving_armv8m::conv_from_bi(&mut output_r1, &mut output_r2, &mut t);
            }
            (output_r1, output_r2)
        }
        else {
            bitinterleaving_generic::conv_from_bi(r1, r2)
        }
    }
}
