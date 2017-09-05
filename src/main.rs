extern crate encoding;

use encoding::{Encoding, DecoderTrap, EncoderTrap};
use encoding::all::{ISO_2022_JP, ISO_8859_1};

fn main() {
    // match ISO_2022_JP.decode(&[130,160,130,162,130,164,130,166,130,168,10], DecoderTrap::Strict) {
    // match ISO_2022_JP.decode(&[148, 132, 143, 227, 144, 172, 146, 183, 151, 166, 32, 50, 49, 48, 54, 37, 13, 10, 112, 114, 111, 103, 138, 214, 140, 87, 13, 10, 130, 160, 130, 162, 130, 164, 130, 166, 130, 168, 13, 10], DecoderTrap::Ignore) {
    // match ISO_2022_JP.decode(&[97, 130, 160], DecoderTrap::Ignore) {
    let enc = match ISO_2022_JP.encode("あああ", EncoderTrap::Ignore) {
        Ok(x)    => {x},
        Err(err) => {panic!(err)},
    };
    println!("{:?}", enc);

    match ISO_2022_JP.decode(&enc, DecoderTrap::Ignore) {
        Ok(x)    => {println!("out: {}", x)},
        Err(err) => {panic!(err)},
    }
}
