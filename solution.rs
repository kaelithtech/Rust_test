// ============================================================
//  SOLUTION - Implement your code here
//  Keep the public types and signature. Everything else is yours.
// ============================================================

pub type Day = u32; // a day number, no date library needed

#[derive(Debug, PartialEq, Eq)]
pub enum FindError {
    InvalidNights,
    NoRoomLeft,
}

/// Returns the earliest day `d >= from` such that the nights `d .. d + nights`
/// do not overlap any stay.
///
/// `stays` are `(check_in, check_out)` pairs with `check_in < check_out` and
/// `check_out` exclusive. They may come in ANY order and may overlap or nest.
pub fn first_available(stays: &[(Day, Day)], nights: u32, from: Day) -> Result<Day, FindError> {
    todo!()
}
