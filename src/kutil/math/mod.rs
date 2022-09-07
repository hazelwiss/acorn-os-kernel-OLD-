macro_rules! impl_for_primitives {
    ($m:ident) => {
        $m!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);
    };
}

pub trait Alignment {
    fn align_up(&self, val: &Self) -> Self;

    fn align_floor(&self, val: &Self) -> Self;

    fn is_aligned(&self, val: &Self) -> bool;
}

pub fn align_up<T: Alignment>(align: &T, val: &T) -> T {
    align.align_up(val)
}

pub fn align_floor<T: Alignment>(align: &T, val: &T) -> T {
    align.align_floor(val)
}

pub fn is_aligned<T: Alignment>(align: &T, val: &T) -> bool {
    align.is_aligned(val)
}

macro_rules! impl_alignment {
    ($($t:ty)*) => {
        $(
            impl Alignment for $t{
                fn align_up(&self, val: &Self) -> $t{
                    *self + (val - (*self % val)) % val
                }

                fn align_floor(&self, val: &Self) -> $t{
                    *self - *self % val
                }

                fn is_aligned(&self, val: &Self) -> bool{
                    *self % val == 0
                }
            }
        )*
    };
}

impl_for_primitives!(impl_alignment);
