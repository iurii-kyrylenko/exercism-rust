use variable_length_quantity::*;

fn main() {
  from_bytes(&[
    0x00,
    0xc0, 0x00,
    0xc8, 0xe8, 0x56,
    0xff, 0xff, 0xff, 0x7f,
    0x00,
    0xff, 0x7f,
    0x81, 0x80, 0x00,
  ]);
}