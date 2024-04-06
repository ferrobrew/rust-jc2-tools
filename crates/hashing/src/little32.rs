const INITIAL_VALUE: u32 = 0xDEADBEEF;

#[inline(always)]
pub const fn hash(data: &[u8]) -> u32 {
    let l = data.len();
    let mut a = INITIAL_VALUE.wrapping_add(l as u32);
    let mut b = a;
    let mut c = a;

    let mut i = 0;

    while l - i > 12 {
        a = a.wrapping_add(
            data[i] as u32
                | ((data[i + 1] as u32) << 8)
                | ((data[i + 2] as u32) << 16)
                | ((data[i + 3] as u32) << 24),
        );
        b = b.wrapping_add(
            data[i + 4] as u32
                | ((data[i + 5] as u32) << 8)
                | ((data[i + 6] as u32) << 16)
                | ((data[i + 7] as u32) << 24),
        );
        c = c.wrapping_add(
            data[i + 8] as u32
                | ((data[i + 9] as u32) << 8)
                | ((data[i + 10] as u32) << 16)
                | ((data[i + 11] as u32) << 24),
        );

        a = a.wrapping_sub(c);
        a ^= c.rotate_left(4);
        c = c.wrapping_add(b);
        b = b.wrapping_sub(a);
        b ^= a.rotate_left(6);
        a = a.wrapping_add(c);
        c = c.wrapping_sub(b);
        c ^= b.rotate_left(8);
        b = b.wrapping_add(a);
        a = a.wrapping_sub(c);
        a ^= c.rotate_left(16);
        c = c.wrapping_add(b);
        b = b.wrapping_sub(a);
        b ^= a.rotate_left(19);
        a = a.wrapping_add(c);
        c = c.wrapping_sub(b);
        c ^= b.rotate_left(4);
        b = b.wrapping_add(a);

        i += 12;
    }

    use const_for::const_for;
    const_for!(o in 0..l - i => {
        match o / 4 {
            0 => {
                a = a.wrapping_add((data[i + o] as u32) << (o % 4 * 8));
            }
            1 => {
                b = b.wrapping_add((data[i + o] as u32) << (o % 4 * 8));
            }
            2 => {
                c = c.wrapping_add((data[i + o] as u32) << (o % 4 * 8));
            }
            _ => unreachable!(),
        }
    });

    c ^= b;
    c = c.wrapping_sub(b.rotate_left(14));
    a ^= c;
    a = a.wrapping_sub(c.rotate_left(11));
    b ^= a;
    b = b.wrapping_sub(a.rotate_left(25));
    c ^= b;
    c = c.wrapping_sub(b.rotate_left(16));
    a ^= c;
    a = a.wrapping_sub(c.rotate_left(4));
    b ^= a;
    b = b.wrapping_sub(a.rotate_left(14));
    c ^= b;
    c = c.wrapping_sub(b.rotate_left(24));

    c
}

const_assert_eq!(hash(b"rico"), 0x6041E481);
const_assert_eq!(hash(b"jc2"), 0xCDF21378);
