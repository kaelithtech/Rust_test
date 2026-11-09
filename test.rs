// ============================================================
//  TEST SUITE - Do not modify
//  Run:  rustc --edition 2021 test.rs -o test && ./test
// ============================================================

#[allow(dead_code, unused_variables)]
mod solution;
use solution::*;

fn main() {
    // --- the basics ---
    assert_eq!(first_available(&[], 3, 7), Ok(7), "nothing booked");

    let stays = [(10, 12), (0, 3), (5, 8)]; // not sorted. Free: [3,5) [8,10) [12,..)
    assert_eq!(first_available(&stays, 2, 0), Ok(3), "a gap of exactly 2 nights fits");
    assert_eq!(first_available(&stays, 3, 0), Ok(12), "no gap of 3 nights before day 12");
    assert_eq!(first_available(&stays, 1, 1), Ok(3), "`from` inside a stay");
    assert_eq!(first_available(&stays, 5, 20), Ok(20), "`from` after every stay");
    println!("✅ basic cases");

    // --- stays that overlap or nest ---
    let nested = [(6, 12), (0, 10), (2, 4)]; // (2,4) is inside (0,10): everything up to day 12 is taken
    assert_eq!(first_available(&nested, 2, 0), Ok(12), "the gap [4,6) is covered by (0,10)");
    println!("✅ nested stays");

    // --- edge cases ---
    assert_eq!(first_available(&[], 0, 5), Err(FindError::InvalidNights));

    // Day numbers are u32: `d + nights` can overflow. That is an error, not a wrap-around.
    const END: Day = u32::MAX;
    assert_eq!(first_available(&[], 1, END), Err(FindError::NoRoomLeft));
    let late = [(END - 5, END)]; // free before it: END-8, END-7, END-6
    assert_eq!(first_available(&late, 3, END - 8), Ok(END - 8));
    assert_eq!(first_available(&late, 4, END - 8), Err(FindError::NoRoomLeft), "the only way out is past the end");
    println!("✅ edge cases");

    println!("\n🎉 All passed!");
}
