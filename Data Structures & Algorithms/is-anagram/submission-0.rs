impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut count = [0; 26];
        for (a, b) in s.bytes().zip(t.bytes()) {
            let index_a = (a - b'a') as usize;
            let index_b = (b - b'a') as usize;

            count[index_a] += 1;
            count[index_b] -= 1;
        }

        count.iter().all(|&v| v == 0)
    }
}
