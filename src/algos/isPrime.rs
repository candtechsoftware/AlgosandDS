
fn is_really_prime(num: u64) -> bool {
    if num <= 1 { return false }
    firstfac(num) == num
} 


fn firstfac(num: u64) -> u64 {
   if num % 2 == 0 { return 2; } 
   for n in (3..).step_by(2).take_while(|n| n*n <= num) {
    if num % n == 0 { return n; }
   }
   num 
}

pub fn is_prime(a: u64) -> u32 {
    if is_really_prime(a) {
        return 1 as u32
    }

    firstfac(a) as u32
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn small_numbers() {
        let small_number: u64 = 10 as u64;
        assert_eq!(is_prime(small_number), 2);
    }

    #[test]
    fn large_non_prime_number() {
        let large_number: u64 = 123023403297;
        assert_eq!(is_prime(large_number), 3);
    }

}
