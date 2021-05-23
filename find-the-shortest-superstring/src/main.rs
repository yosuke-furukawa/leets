impl Solution {
    fn find_common_word_count(word1: &String, word2: &String) -> (usize, String) {
        let w1_len = word1.len();
        let w2_len = word2.len();
        let mut max = 0;
        let mut word = String::new();
        // check suffix word1 and prefix word2
        for i in 0..w1_len.min(w2_len) {
            if word1.get((w1_len - i)..w1_len) == word2.get(0..i) {
                if max < i {
                    max = i;
                    word = word1.clone() + word2.get(i..w2_len).unwrap();
                }
            }
        }

        // check suffix word2 and prefix word1
        for i in 0..w1_len.min(w2_len) {
            if word1.get(0..i) == word2.get((w2_len - i)..w2_len) {
                if max < i {
                    max = i;
                    word = word2.clone() + word1.get(i..w1_len).unwrap();
                }
            }
        }

        (max, word)
    }
    pub fn shortest_superstring(words: Vec<String>) -> String {
        let mut words = words;
        let mut n = words.len();

        while n > 1 {
            let mut max = 0;
            let mut max_str = String::new();
            let mut l = 0;
            let mut r = 0;
            for i in 0..n {
                for j in i+1..n {
                    let (count, s) = Solution::find_common_word_count(&words[i], &words[j]);
                    if max <= count {
                        max = count;
                        max_str = s;
                        l = i;
                        r = j;
                    }
                }
            }

            n -= 1;

            if max == 0 {
                let w = words[0].clone() + &words[n];
                words[0] = w;
            } else {
                words[l] = max_str;
                words[r] = words[n].clone();
            }
        }
        words[0].clone()
    }
}

struct Solution;

fn stringify(words: Vec<&str>) -> Vec<String> {
    words.into_iter().map(|s| s.to_string()).collect()
}

fn main() {
    println!("{}", Solution::shortest_superstring(stringify(vec!["catg","ctaagt","gcta","ttca","atgcatc"])));
    println!("{}", Solution::shortest_superstring(stringify(vec!["alex","loves","leetcode"])));
}
