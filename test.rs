fn main() {
    let mut num: u8 = 192; //1100 0000
    println!("{}", num);
    num = num << 1; //1000 0000
    println!("{}", num);
    num = num >> 1; //0100 0000
    println!("{}", num);
}
