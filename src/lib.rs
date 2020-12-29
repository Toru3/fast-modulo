#![no_std]
#![feature(asm)]
pub mod reference {
    #[inline]
    /// calculate `a * b % m`
    ///
    /// ```
    /// use fast_modulo::reference::mulmod_u64;
    /// assert_eq!(mulmod_u64(3, 4, 5), 2);
    /// assert_eq!(mulmod_u64((1 << 63) - 1, (1 << 63) - 3, (1 << 63) + 1), 8);
    /// ```
    pub fn mulmod_u64(a: u64, b: u64, m: u64) -> u64 {
        let aa = a as u128;
        let bb = b as u128;
        let mm = m as u128;
        let rr = aa * bb % mm;
        rr as _
    }

    #[inline]
    /// calcurate `a % m`
    ///
    /// ```
    /// use fast_modulo::reference::mod_u128u64;
    /// assert_eq!(mod_u128u64(17, 3), 2);
    /// assert_eq!(mod_u128u64((1 << 127) - 1, (1 << 61) - 1), 31);
    /// ```
    pub fn mod_u128u64(a: u128, m: u64) -> u64 {
        let mm = m as u128;
        let rr = a % mm;
        rr as _
    }

    #[inline]
    /// calculate $`a ^ p \bmod m`$
    ///
    /// ```
    /// use fast_modulo::reference::powmod_u64;
    /// assert_eq!(powmod_u64(2, 10, 13), 10);
    /// assert_eq!(powmod_u64(31, 41, 59), 39);
    /// ```
    pub fn powmod_u64(mut a: u64, mut p: u64, m: u64) -> u64 {
        let mut y = 1;
        while p > 0 {
            if p % 2 == 1 {
                y = mulmod_u64(y, a, m);
            }
            a = mulmod_u64(a, a, m);
            p /= 2;
        }
        y
    }
}

mod internal {
    #[inline]
    #[cfg(target_arch = "x86_64")]
    pub fn mulmod_u64(a: u64, mut b: u64, m: u64) -> u64 {
        unsafe {
            asm!(
                "mul rdx",
                "div {}",
                in(reg) m,
                inout("rax") a => _,
                inout("rdx") b,
            );
        }
        b
    }
    #[inline]
    #[cfg(not(target_arch = "x86_64"))]
    pub fn mulmod_u64(a: u64, b: u64, m: u64) -> u64 {
        // fallback
        super::reference::mulmod_u64(a, b, m)
    }

    #[inline]
    #[cfg(target_arch = "x86_64")]
    pub fn mod_u128u64(a: u128, m: u64) -> u64 {
        let qb = 65 + m.leading_zeros() - a.leading_zeros();
        if qb > 64 {
            let s = qb - 64;
            let mask = (1 << s) - 1;
            let r = mod_u128u64_unchecked(a >> s, m) as u128;
            let na = (r << s) + (a & mask);
            mod_u128u64_unchecked(na, m)
        } else {
            mod_u128u64_unchecked(a, m)
        }
    }
    #[inline]
    #[cfg(not(target_arch = "x86_64"))]
    pub fn mod_u128u64(a: u128, m: u64) -> u64 {
        // fallback
        super::reference::mod_u128u64(a, m)
    }

    #[inline]
    #[cfg(target_arch = "x86_64")]
    pub fn mod_u128u64_unchecked(a: u128, m: u64) -> u64 {
        let hi = (a >> 64) as u64;
        let lo = (a & 0xFFFFFFFFFFFFFFFF) as u64;
        let r;
        unsafe {
            asm!(
                "div {}",
                in(reg) m,
                inout("rax") lo => _,
                inout("rdx") hi => r,
            );
        }
        r
    }
    #[inline]
    #[cfg(not(target_arch = "x86_64"))]
    pub fn mod_u128u64_unchecked(a: u128, m: u64) -> u64 {
        // fallback
        super::reference::mod_u128u64(a, m)
    }
}

#[inline]
/// calculate `a * b % m`
///
/// required `a < m && b < m`.
/// ```
/// use fast_modulo::mulmod_u64;
/// assert_eq!(mulmod_u64(3, 4, 5), 2);
/// assert_eq!(mulmod_u64((1 << 63) - 1, (1 << 63) - 3, (1 << 63) + 1), 8);
/// ```
pub fn mulmod_u64(a: u64, b: u64, m: u64) -> u64 {
    internal::mulmod_u64(a, b, m)
}

#[inline]
/// calcurate `a % m`
///
/// ```
/// use fast_modulo::mod_u128u64;
/// assert_eq!(mod_u128u64(17, 3), 2);
/// assert_eq!(mod_u128u64((1 << 127) - 1, (1 << 61) - 1), 31);
/// ```
pub fn mod_u128u64(a: u128, m: u64) -> u64 {
    internal::mod_u128u64(a, m)
}

#[inline]
/// calcurate `a % m`
///
/// This function doesn't check quotient less than $`2^{64}`$.
/// required $`a < 2^{64}m`$
/// ```
/// use fast_modulo::mod_u128u64_unchecked;
/// assert_eq!(mod_u128u64_unchecked(17, 3), 2);
/// assert_eq!(mod_u128u64_unchecked((1 << 107) - 1, (1 << 61) - 1), 70368744177663);
/// ```
pub fn mod_u128u64_unchecked(a: u128, m: u64) -> u64 {
    internal::mod_u128u64_unchecked(a, m)
}

#[inline]
/// calculate $`a ^ p \bmod m`$
///
/// required `a < m`.
/// ```
/// use fast_modulo::powmod_u64;
/// assert_eq!(powmod_u64(2, 10, 13), 10);
/// assert_eq!(powmod_u64(31, 41, 59), 39);
/// ```
pub fn powmod_u64(mut a: u64, mut p: u64, m: u64) -> u64 {
    let mut y = 1;
    while p > 0 {
        if p % 2 == 1 {
            y = mulmod_u64(y, a, m);
        }
        a = mulmod_u64(a, a, m);
        p /= 2;
    }
    y
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn modulo_u64_mul() {
        use rand::prelude::*;
        let mut rng = rand::thread_rng();
        for _ in 0..1_000_000 {
            let m = rng.gen();
            let a = rng.gen::<u64>() % m;
            let b = rng.gen::<u64>() % m;
            assert_eq!(reference::mulmod_u64(a, b, m), mulmod_u64(a, b, m));
        }
    }
    #[test]
    fn modulo_u128u64() {
        use rand::prelude::*;
        let mut rng = rand::thread_rng();
        for _ in 0..1_000_000 {
            let m = rng.gen();
            let a = rng.gen();
            assert_eq!(reference::mod_u128u64(a, m), mod_u128u64(a, m));
        }
    }
    #[test]
    fn modulo_u128u64_small_divisor() {
        use rand::prelude::*;
        let mut rng = rand::thread_rng();
        for _ in 0..1_000_000 {
            let m = rng.gen::<u32>().into();
            let a = rng.gen();
            assert_eq!(reference::mod_u128u64(a, m), mod_u128u64(a, m));
        }
    }
    #[test]
    fn modulo_u64_pow() {
        use rand::prelude::*;
        let mut rng = rand::thread_rng();
        for _ in 0..1_000_000 {
            let m = rng.gen();
            let a = rng.gen::<u64>() % m;
            let p = rng.gen::<u64>();
            assert_eq!(reference::powmod_u64(a, p, m), powmod_u64(a, p, m));
        }
    }
}
