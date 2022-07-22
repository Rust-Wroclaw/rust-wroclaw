pub fn sum(x: *const [u8; 32]) -> u16 {
    unsafe {
        let mut sum = 0;
        let x = &*x;

        for i in 0..32 {
            sum += x[i] as u16;
        }

        sum
    }
}
