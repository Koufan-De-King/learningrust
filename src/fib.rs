pub fn fibo(number: u64) -> u64 {
    if number < 2 {
       return number
    }
    else {
        return fibo(number-1) + fibo(number-2)
    }
}