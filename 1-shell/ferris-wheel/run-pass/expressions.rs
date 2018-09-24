// FIXME: Make me pass! Diff budget: 10 lines.
// Do not `use` any items.

// Do not change the following two lines.
#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
struct IntWrapper(isize);

fn max<T>(v1: T, v2: T) -> T where T: PartialOrd {
    if v1 > v2 {
        v1
    } else if v1 < v2 {
        v2
    } else {
        v1
    }
}

pub fn main() {
    assert_eq!(max(1usize, 3), 3);
    assert_eq!(max(1u8, 3), 3);
    assert_eq!(max(1u8, 3), 3);
    assert_eq!(max(IntWrapper(120), IntWrapper(248)), IntWrapper(248));
}
