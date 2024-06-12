fn main() {
    for i in 0..5 {
        let value = i * 2;
        let address = &value as *const i32;
        println!("Address of value {} is {:p}", value, address); //{:p} >> pointer
    }
}
