use nbitmask::BitMask;

#[test]
fn test_print() {
    let mask: BitMask<u64> = BitMask::zeros(5);
    assert_eq!(mask.to_string(), "00000".to_string());

    let mask: BitMask<u64> = BitMask::zeros(64);
    assert_eq!(mask.to_string(), String::from_utf8(vec![b'0'; 64]).unwrap());

    let mask: BitMask<u64> = BitMask::zeros(75);
    assert_eq!(mask.to_string(), String::from_utf8(vec![b'0'; 75]).unwrap());

    let mut mask: BitMask<u64> = BitMask::zeros(3);
    mask.set(1, true).unwrap();
    assert_eq!(mask.to_string(), "010".to_string());
}

#[test]
fn test_get_set_u64() {
    let mut mask: BitMask<u64> = BitMask::zeros(5);
    mask.set(1, true).unwrap();
    assert_eq!(mask.get(0).unwrap(), false);
    assert_eq!(mask.get(1).unwrap(), true);
    assert_eq!(mask.to_string(), "01000".to_string());

    let mut mask: BitMask<u64> = BitMask::zeros(5);
    mask.set_all(true);
    mask.set(1, false).unwrap();
    assert_eq!(mask.get(0).unwrap(), true);
    assert_eq!(mask.get(1).unwrap(), false);
    assert_eq!(mask.to_string(), "10111".to_string());
}

#[test]
fn test_get_set_error() {
    let mut mask: BitMask<u8> = BitMask::zeros(4);
    assert!(mask.set(3, false).is_ok());
    assert!(mask.set(4, false).is_err()); //Bitmask contains only 4 bits (index 0-3)
    assert!(mask.set(8, false).is_err());
    assert!(mask.get(3).is_ok());
    assert!(mask.get(4).is_err());
    assert!(mask.get(8).is_err());
}

#[test]
fn test_get_set_u16() {
    let mut mask: BitMask<u16> = BitMask::zeros(17);
    mask.set(1, true).unwrap();
    assert_eq!(mask.get(0).unwrap(), false);
    assert_eq!(mask.get(1).unwrap(), true);
    assert_eq!(mask.to_string(), "01000000000000000".to_string());

    let mut mask: BitMask<u16> = BitMask::zeros(17);
    mask.set_all(true);
    mask.set(1, false).unwrap();
    assert_eq!(mask.get(0).unwrap(), true);
    assert_eq!(mask.get(1).unwrap(), false);
    assert_eq!(mask.to_string(), "10111111111111111".to_string());
}

#[test]
fn test_get_set_u8() {
    let mut mask: BitMask<u8> = BitMask::zeros(10);
    mask.set(1, true).unwrap();
    assert_eq!(mask.get(0).unwrap(), false);
    assert_eq!(mask.get(1).unwrap(), true);
    assert_eq!(mask.to_string(), "0100000000".to_string());

    let mut mask: BitMask<u8> = BitMask::zeros(10);
    mask.set_all(true);
    mask.set(1, false).unwrap();
    assert_eq!(mask.get(0).unwrap(), true);
    assert_eq!(mask.get(1).unwrap(), false);
    assert_eq!(mask.to_string(), "1011111111".to_string());
}

#[test]
fn test_equals() {
    let mut mask1: BitMask<u8> = BitMask::zeros(5);
    mask1.set(1, true).unwrap();

    let mut mask2: BitMask<u8> = BitMask::zeros(5);
    mask2.set(1, true).unwrap();

    assert_eq!(mask1, mask2);
}

#[test]
fn test_not_equals() {
    let mut mask1: BitMask<u8> = BitMask::zeros(6);
    mask1.set(1, true).unwrap();

    let mut mask2: BitMask<u8> = BitMask::zeros(5);
    mask2.set(1, true).unwrap();

    assert_ne!(mask1, mask2);
}

#[test]
fn test_or() {
    let mut a: BitMask<u64> = BitMask::zeros(3);
    let mut b: BitMask<u64> = BitMask::zeros(3);

    a.set(1, true).unwrap();
    b.set(2, true).unwrap();

    assert_eq!((&a | &b).to_string(), "011".to_string());
}

#[test]
fn test_or_assign() {
    let mut a: BitMask<u64> = BitMask::ones(3);
    let mut b: BitMask<u64> = BitMask::ones(4);
    a.set(1, false).unwrap();
    b.set(1, false).unwrap();

    a |= &b;

    assert_eq!(a.to_string(), "1011".to_string());
}

#[test]
fn test_and() {
    let mut a: BitMask<u64> = BitMask::zeros(3);
    let mut b: BitMask<u64> = BitMask::zeros(3);
    a.set_all(true);
    b.set_all(true);
    a.set(1, false).unwrap();
    b.set(2, false).unwrap();

    assert_eq!((&a & &b).to_string(), "100".to_string());
}

#[test]
fn test_and_assign() {
    let mut a: BitMask<u64> = BitMask::zeros(3);
    let mut b: BitMask<u64> = BitMask::zeros(100);
    a.set_all(true);
    b.set_all(true);
    a.set(1, false).unwrap();
    b.set(2, false).unwrap();

    a &= &b;

    assert_eq!(a.to_string(), "100".to_string());
}

#[test]
fn test_xor() {
    let mut a: BitMask<u8> = BitMask::zeros(3);
    a.set_all(true);
    assert_eq!(&a ^ &BitMask::ones(3), BitMask::zeros(3));

    let c = &a ^ &BitMask::zeros(5);
    assert_eq!(c.to_string(), "11100".to_string());

    let d = &a ^ &BitMask::ones(6);
    assert_eq!(d.to_string(), "000111".to_string());
}

#[test]
fn test_xor_assign() {
    let mut a: BitMask<u8> = BitMask::zeros(3);
    a.set_all(true);
    a ^= &BitMask::ones(3);
    assert_eq!(a, BitMask::zeros(3));
    a ^= &BitMask::ones(3);
    assert_eq!(a, BitMask::ones(3));

    a ^= &BitMask::zeros(5);
    assert_eq!(a.to_string(), "11100".to_string());

    a ^= &BitMask::ones(6);
    assert_eq!(a.to_string(), "000111".to_string());
}

#[test]
fn test_not() {
    let mut a: BitMask<u8> = BitMask::ones(3);
    assert_eq!(!&a, BitMask::zeros(3));

    let b: BitMask<u8> = BitMask::zeros(20);
    assert_eq!(!&b, BitMask::ones(20));

    a.set(0, false).unwrap();

    assert_eq!((!&a).to_string(), "100".to_string());
}

#[test]
fn test_shr() {
    let mut a: BitMask<u8> = BitMask::zeros(3);
    a.set(0, true).unwrap();
    a.set(2, true).unwrap();

    assert_eq!((&a >> 1).to_string(), "010".to_string());

    let mut b: BitMask<u8> = BitMask::zeros(14);
    b.set(1, true).unwrap();
    b.set(5, true).unwrap();
    b.set(10, true).unwrap();

    assert_eq!((&b >> 3).to_string(), "00100001000000".to_string());

    let c: BitMask<u8> = BitMask::ones(40);
    assert_eq!(&c >> 50, BitMask::zeros(40));
}

#[test]
fn test_shr_assign() {
    let mut a: BitMask<u8> = BitMask::zeros(3);
    a.set(0, true).unwrap();
    a.set(2, true).unwrap();

    a >>= 1;

    assert_eq!(a.to_string(), "010".to_string());

    let mut b: BitMask<u8> = BitMask::zeros(14);
    b.set(1, true).unwrap();
    b.set(5, true).unwrap();
    b.set(10, true).unwrap();

    b >>= 3;
    assert_eq!(b.to_string(), "00100001000000".to_string());

    let mut c: BitMask<u8> = BitMask::ones(40);
    c >>= 50;
    assert_eq!(c, BitMask::zeros(40));
}

#[test]
fn test_shl() {
    let mut a: BitMask<u8> = BitMask::zeros(3);
    a.set(0, true).unwrap();
    a.set(2, true).unwrap();

    assert_eq!((&a << 1).to_string(), "010".to_string());

    let mut b: BitMask<u8> = BitMask::zeros(14);
    b.set(1, true).unwrap();
    b.set(5, true).unwrap();
    b.set(10, true).unwrap();

    assert_eq!((&b << 3).to_string(), "00001000100001".to_string());

    let c: BitMask<u8> = BitMask::ones(40);
    assert_eq!(&c << 50, BitMask::zeros(40));
}

#[test]
fn test_example() {
    let ones: BitMask<u64> = BitMask::ones(7);
    let mut mask: BitMask<u64> = BitMask::zeros(4);

    mask.set(0, true).unwrap();
    // Will display an out of bound error as bit 4 doesn't exist in mask
    mask.set(4, true).unwrap_or_else(|err| println!("{}", err));

    mask &= &ones;
    println!("mask size didn't change : {}", mask.length());

    let mask_copy = mask.clone();

    mask <<= 1;
    assert_eq!(mask.to_string(), "0100".to_string());

    mask >>= 1;
    assert_eq!(mask.to_string(), mask_copy.to_string());
    assert_eq!(mask, mask_copy);
}

#[test]
fn test_example2() {
    let mut mask: BitMask<u8> = BitMask::zeros(14);
    mask.set(5, true).unwrap();
    mask.set(10, true).unwrap();

    let mut mask2: BitMask<u8> = BitMask::zeros(14);
    mask2.set(2, true).unwrap();
    mask2.set(7, true).unwrap();

    mask >>= 3;

    assert_eq!(mask.to_string(), mask2.to_string());
    assert_eq!(mask, mask2);
}
