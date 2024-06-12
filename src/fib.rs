pub fn fibo(number: u64) -> u64 {
    if number < 2 {
       number
    }else { 
       fibo(number - 1) + fibo(number - 2) 
    }
}