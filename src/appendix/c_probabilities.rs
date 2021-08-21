#[cfg(test)]
mod tests {
    use itertools::Itertools;

    #[test]
    fn c2_1() {
        // WRONG
        // s = { h, t };
        // rosencrantz, A = { h }, all outcomes = { h, t };
        // guildenstern, B = { tt }, all outcomes = { hh, ht, th, tt };
        // P(A|B) = P(AB)/P(B) = (P(A)P(B))/P(B) = P(A) = 1/2


        // RIGHT
        // S = { hhh, hht, hth, >htt<, thh, tht, tth, ttt };
    }

    #[test]
    fn c2_2() {
        // P(A1+A2+A3+...) <= P(A1) + P(A2) + P(A3) ...
    }

    fn fact(a: u32) -> f32 {
        let mut res = 1;
        for i in 1..=a {
            res *= i;
        }
        res as f32
    }

    #[test]
    fn fact_test() {
        assert_eq!(fact(3), 6.0);
        assert_eq!(fact(5), 120.0);
    }

    #[test]
    fn c2_3() {
        use itertools::Itertools;
        let a = [ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 ];

        // all k-permutations from a
        let all_3 = fact(10)/fact(10-3);

        // all permutations from a
        let den = fact(10);

        println!("{} {}", all_3, den);

        // P(A) * P(B|A) * P(C|B|A)
        // A = { 1 .. 8 }; P(A) = 1/8
        // B = { A+1 .. 9 }; if A == 1 => B = { 2 .. 9 } P(B|A) = 1/8; if A == 8 => B = { 9 .. 9 } P(B|A) = 1
        // C = { B+1 .. 10 }; if B == 2 => C = { 3 .. 10 } P(C|B|A) = 1/8; if B == 9 => C = { 10 .. 10 } P(C|B|A) = 1
    }
}