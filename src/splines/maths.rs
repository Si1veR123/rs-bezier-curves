pub fn factorial(num: u128) -> u128 {
    match num {
        0  => 1,
        1.. => (1..num+1).product(),
    }
}