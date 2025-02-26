fn count_permutations(n: usize) -> usize {
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    
    for i in 1..=n {
        for j in (1..=i).rev() {
            dp[i] += dp[i - j];
        }
    }
    
    dp[n]
}

fn main() {
    let number = 2;
    println!("Number of permutations to form {}: {}", number, count_permutations(number));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_count_permutations() {
        assert_eq!(count_permutations(1), 1);  // [1]
        assert_eq!(count_permutations(2), 2);  // [2], [1+1]
        assert_eq!(count_permutations(3), 4);  // [3], [2+1], [1+2], [1+1+1]
        assert_eq!(count_permutations(4), 8);  // [4], [3+1], [1+3], [2+2], [2+1+1], [1+2+1], [1+1+2], [1+1+1+1]
    }
}
