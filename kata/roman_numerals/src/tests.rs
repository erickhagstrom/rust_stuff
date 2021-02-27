use crate::*;

fn test_roman(given: usize, expected: &str) {
  assert_eq!(arabic_to_roman(given).unwrap(), expected);
}

#[test]
fn one() {
  test_roman(1, "I");
}

#[test]
fn two() {
  test_roman(2, "II");
}

#[test]
fn three() {
  test_roman(3, "III");
}

#[test]
fn four() {
  test_roman(4, "IV");
}

#[test]
fn five() {
  test_roman(5, "V");
}

#[test]
fn eight() {
  test_roman(8, "VIII");
}

#[test]
fn nine() {
  test_roman(9, "IX");
}

#[test]
fn thirty() {
  test_roman(30, "XXX");
}

#[test]
fn thirty_four() {
  test_roman(34, "XXXIV");
}

#[test]
fn thirty_eight() {
  test_roman(38, "XXXVIII");
}

#[test]
fn thirty_nine() {
  test_roman(39, "XXXIX");
}

#[test]
fn forty() {
  test_roman(40, "XL");
}

#[test]
fn forty_eight() {
  test_roman(48, "XLVIII");
}

#[test]
fn forty_nine() {
  test_roman(49, "XLIX");
}

#[test]
fn fifty() {
  test_roman(50, "L");
}

#[test]
fn eighty_eight() {
  test_roman(88, "LXXXVIII");
}

#[test]
fn eighty_nine() {
  test_roman(89, "LXXXIX");
}

#[test]
fn ninety() {
  test_roman(90, "XC");
}

#[test]
fn ninety_eight() {
  test_roman(98, "XCVIII");
}

#[test]
fn ninety_nine() {
  test_roman(99, "XCIX");
}

#[test]
fn one_hundred() {
  test_roman(100, "C");
}

#[test]
fn three_hundred_ninety_nine() {
  test_roman(399, "CCCXCIX");
}

#[test]
fn four_hundred() {
  test_roman(400, "CD");
}

#[test]
fn four_hundred_ninety_nine() {
  test_roman(499, "CDXCIX");
}

#[test]
fn five_hundred() {
  test_roman(500, "D");
}

#[test]
fn eight_hundred_ninety_nine() {
  test_roman(899, "DCCCXCIX");
}

#[test]
fn nine_hundred() {
  test_roman(900, "CM");
}

#[test]
fn nine_nundred_ninety_nine() {
  test_roman(999, "CMXCIX");
}

#[test]
fn one_thousand() {
  test_roman(1_000, "M");
}

#[test]
fn three_thousand_nine_hundred_ninety_nine() {
  test_roman(3_999, "MMMCMXCIX");
}

#[test]
fn bounds_zero() {
  assert!(match arabic_to_roman(0) {
    Ok(_) => false,
    Err(e) => "Must be greater than 0: 0" == e,
  })
}

#[test]
fn bounds_four_thousand() {
  assert!(match arabic_to_roman(4_000) {
    Ok(_) => false,
    Err(e) => "Must be less than 4,000: 4000" == e,
  })
}
