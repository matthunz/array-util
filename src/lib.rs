#![allow(incomplete_features)]
#![feature(
    const_evaluatable_checked,
    const_fn,
    const_generics,
    const_panic,
    const_ptr_read,
    const_refs_to_cell,
    const_trait_impl,
    maybe_uninit_uninit_array,
    maybe_uninit_array_assume_init
)]

use std::mem::MaybeUninit;

const unsafe fn assume_init<T: Copy, const N: usize>(array: [MaybeUninit<T>; N]) -> [T; N] {
    (&array as *const _ as *const [T; N]).read()
}

/// Creates an array that flattens a nested structure.
///
/// # Examples
/// ```
/// let array = [[1u8, 2], [3, 4]];
/// let flattened = array_util::flatten(array);
/// assert_eq!(flattened, [1u8, 2, 3, 4]);
/// ```
pub const fn flatten<T: Copy, const A: usize, const B: usize>(array: [[T; B]; A]) -> [T; A * B] {
    let mut data: [_; A * B] = MaybeUninit::uninit_array();
    let mut pos = 0;
    while pos < A {
        let inner = array[pos];
        let mut i = 0;

        while i < B {
            data[pos * B + i] = MaybeUninit::new(inner[i]);
            i += 1;
        }

        pos += 1
    }
    // Safety: data was fully initialized
    unsafe { assume_init(data) }
}

pub trait ArrayUtil {
    type Item: Copy;
    const LEN: usize;

    /// Creates a new array with the last element removed.
    fn pop(self) -> [Self::Item; Self::LEN - 1];

    /// Creates a new array with an additional element at the back.
    fn push(self, value: Self::Item) -> [Self::Item; Self::LEN + 1];

    /// Creates a new array without the element at position `index`,
    /// shifting all elements after it to the left.
    fn remove(self, index: usize) -> [Self::Item; Self::LEN - 1];

    /// Creates a new array with the order of elements reversed.
    fn reverse(self) -> [Self::Item; Self::LEN];

    /// Divides one array into two at an index.
    ///
    /// The first will contain all indices from `[0, POS)` (excluding
    /// the index `POS` itself) and the second will contain all
    /// indices from `[mid, len)` (excluding the index `len` itself).
    fn split<const POS: usize>(self) -> ([Self::Item; POS], [Self::Item; Self::LEN - POS]);
}

impl<T, const N: usize> const ArrayUtil for [T; N]
where
    T: Copy,
{
    type Item = T;
    const LEN: usize = N;

    fn pop(self) -> [Self::Item; Self::LEN - 1] {
        self.remove(Self::LEN - 1)
    }

    fn push(self, value: Self::Item) -> [Self::Item; Self::LEN + 1] {
        let mut data: [_; Self::LEN + 1] = MaybeUninit::uninit_array();

        let mut pos = 0;
        while pos < Self::LEN {
            data[pos] = MaybeUninit::new(self[pos]);
            pos += 1
        }
        data[N] = MaybeUninit::new(value);

        // Safety: data was fully initialized
        unsafe { assume_init(data) }
    }

    

    fn remove(self, index: usize) -> [Self::Item; Self::LEN - 1] {
        assert!(index < Self::LEN);

        let mut data: [_; Self::LEN - 1] = MaybeUninit::uninit_array();

        let mut pos = 0;
        let mut i = 0;
        while pos < Self::LEN {
            if pos != index {
                data[i] = MaybeUninit::new(self[pos]);
                i += 1;
            }
            pos += 1;
        }

        // Safety: data was fully initialized
        unsafe { assume_init(data) }
    }

    fn reverse(self) -> [Self::Item; Self::LEN] {
        let mut data: [_; Self::LEN] = MaybeUninit::uninit_array();

        let mut pos = 0;
        while pos < Self::LEN {
            data[Self::LEN - pos - 1] = MaybeUninit::new(self[pos]);
            pos += 1;
        }

        // Safety: data was fully initialized
        unsafe { assume_init(data) }
    }

    fn split<const POS: usize>(self) -> ([Self::Item; POS], [Self::Item; Self::LEN - POS]) {
        let mut a: [_; POS] = MaybeUninit::uninit_array();
        let mut b: [_; Self::LEN - POS] = MaybeUninit::uninit_array();

        let mut pos = 0;
        while pos < a.len() {
            a[pos] = MaybeUninit::new(self[pos]);
            pos += 1
        }

        while pos < Self::LEN {
            b[pos - POS] = MaybeUninit::new(self[pos]);
            pos += 1
        }

        // Safety: both a and b were fully initialized
        unsafe { (assume_init(a), assume_init(b)) }
    }
}

#[cfg(test)]
mod tests {
    use super::ArrayUtil;

    // TODO remove: The compiler currently panics when trying to
    // use `==` on the returned arrays
    fn eq<T: Eq>(a: &[T], b: &[T]) -> bool {
        a.iter().eq(b.iter())
    }

    #[test]
    fn flatten() {
        assert!(eq(&super::flatten([[1, 2], [3, 4]]), &[1, 2, 3, 4]));
    }

    #[test]
    fn pop() {
        assert!(eq(&[1, 2, 3].pop(), &[1, 2]));
    }

    #[test]
    fn push() {
        assert!(eq(&[1, 2].push(3), &[1, 2, 3]));
    }

    #[test]
    fn remove() {
        assert!(eq(&[1, 2, 3].remove(1), &[1, 3]));
    }

    #[test]
    fn reverse() {
        assert!(eq(&[1, 2, 3].reverse(), &[3, 2, 1]));
    }

    #[test]
    fn split() {
        let (a, b) = [1, 2, 3].split::<2>();
        assert!(eq(&a, &[1, 2]));
        assert!(eq(&b, &[3]));
    }
}
