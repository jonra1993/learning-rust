/*

Integer types
| Length   | Signed | Unsigned |
| -------- | ------ | -------- |
| 8-bit    | i8     | u8       |
| 16-bit   | i16    | u16      |
| 32-bit   | i32    | u32      |
| 64-bit   | i64    | u64      |
| 128-bit  | i128   | u128     |
| arch     | isize  | usize    |

Floating-Point Types
| Length   | type |
| -------- | ------ |
| 32-bit   | f32    |
| 64-bit   | f64    |
 */

fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    let t = true;

    let f: bool = false; // with explicit type annotation

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let _heart_eyed_cat = '😻';
}