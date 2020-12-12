#![feature(type_alias_impl_trait)]

#[cutlass::curry]
fn concat<T1: Copy, T2: Copy, T3: Copy>(x: T1, y: T2, z: T3) -> (T1, T2, T3) {
    return (x, y, z);
}

#[test]
fn concat_works() {
    assert_eq!(concat(1)(2 as u8)(3 as i16), (1, 2 as u8, 3 as i16));
}
