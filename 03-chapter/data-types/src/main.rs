fn main() {
    // #### Integers ####

    // They signed integers are (2^n-1) to 2^n-1 - 1
    // So basically n being the integer bit size
    // A 8-bit integer is (2^8-1) to 2^8-1 - 1 => (2^7) to 2^7 - 1
    // Can go all the way to -128 to 127
    let x: i8 = 127;
    let y: i128 = 427_000_000_000;

    // A unsigned integer is straight up just 2^8
    // Max unsigned 8-bit integer is 0 to 255
    let v: u8 = 255; // Max number on unsign 8-bit integer
    // Integer overflow
    // In debug more we get a panic if a int overflow occurs
    // But in released more we dont it wraps around so 256 => 0 and 257 => 1 and so on

    // We can use a family of  wrapping functions to deal with this
    // Wrap in all modes with the wrapping_* methods, such as wrapping_add.
    // Return the None value if there is overflow with the checked_* methods.
    // Return the value and a Boolean indicating whether there was overflow with the overflowing_* methods.
    // Saturate at the value’s minimum or maximum values with the saturating_* methods.

    // #### Floating point ####

    // We have 2 primitives types f32 and f64
    let z: f32 = 17.99;
    let c: f64 = 274.99;

    // Typical programming things (+ - * / % )

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

    // #### Booleans ####

    // Booleans are 1 byte in size
    let t = true;

    // explicit type annotation
    let f: bool = false;

    // #### Characters ####
    // - Declared with single quotes ''
    // char type is 4 bytes in size
    // and represents a Unicode scalar value (can represent a lot more than just ASCII)
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // #### TUPLES ####
    let tup: (i32, f32, bool) = (39, 3.5, false);
    let one: i32 = tup.0;
    let two: f32 = tup.1;
    let three: bool = tup.2;
}
