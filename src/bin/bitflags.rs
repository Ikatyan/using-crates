#[macro_use]
extern crate bitflags;


bitflags! {
    struct Flags: u8{
        const FLAG_A = 0b10000001;
        const FLAG_B = 0b10000010;
        const FLAG_C = 0b10000100;
        const ABC = Self::FLAG_A.bits | Self::FLAG_B.bits | Self::FLAG_C.bits;
    }
}

fn main() {
    let flag = Flags::FLAG_A;
    let flag2 = Flags::FLAG_B;
    println!("{:b}", flag);
    println!("{:b}", flag | flag2);
    println!("{}", flag == Flags::FLAG_A);
    println!("{}", flag == Flags::FLAG_B);
}