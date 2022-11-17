use bitmask::BitMask;

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
    let mut mask: BitMask<u8> = BitMask::zeros(257);
    mask.set(1, true).unwrap();
    assert_eq!(mask.get(0).unwrap(), false);
    assert_eq!(mask.get(1).unwrap(), true);
    assert_eq!(mask.to_string(), "01000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000".to_string());

    let mut mask: BitMask<u8> = BitMask::zeros(257);
    mask.set_all(true);
    mask.set(1, false).unwrap();
    assert_eq!(mask.get(0).unwrap(), true);
    assert_eq!(mask.get(1).unwrap(), false);
    assert_eq!(mask.to_string(), "10111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111".to_string());
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

    assert_eq!((a | b).to_string(), "011".to_string());
}

#[test]
fn test_or_assign() {
    let mut a: BitMask<u64> = BitMask::ones(3);
    let mut b: BitMask<u64> = BitMask::ones(4);
    a.set(1, false).unwrap();
    b.set(1, false).unwrap();

    a |= b;

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

    assert_eq!((a & b).to_string(), "100".to_string());
}

#[test]
fn test_and_assign() {
    let mut a: BitMask<u64> = BitMask::zeros(3);
    let mut b: BitMask<u64> = BitMask::zeros(100);
    a.set_all(true);
    b.set_all(true);
    a.set(1, false).unwrap();
    b.set(2, false).unwrap();

    a &= b;

    assert_eq!(a.to_string(), "100".to_string());
}
