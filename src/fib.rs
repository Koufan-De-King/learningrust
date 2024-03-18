pub fn fibo(number: u32) -> u32 {
    if number < 2 {
       return number
    }
    else {
        return fibo(number-1) + fibo(number-2)
    }
}