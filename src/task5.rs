use cap::Cap;
use std::alloc;

struct Solution;

#[global_allocator]
static ALLOCATOR: Cap<alloc::System> = Cap::new(alloc::System, 30 * 1024 * 1024);

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let len = s.len();
        let mut maxlen = 0;
        let mut maxpal = s.chars().next().unwrap_or('a').to_string();
        for left in 0..len - 1 {
            for right in left..len + 1 {
                let substr = s[left..right].to_string();
                let substr_len = substr.len();
                if is_palyndrome(substr.clone()) {
                    if substr_len > maxlen {
                        maxlen = substr_len;
                        maxpal = substr;
                    }
                }
            }
        }
        maxpal
    }
}

#[inline(always)]
fn is_palyndrome(s: String) -> bool {
    s.clone() == reverse(s)
}

#[inline(always)]
fn reverse(s: String) -> String {
    s.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use crate::task5::Solution;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::longest_palindrome("babad".to_string()),
            "bab".to_string()
        );

        assert_eq!(
            Solution::longest_palindrome("cbbd".to_string()),
            "bb".to_string()
        );

        let long_input = "klvxwqyzugrdoaccdafdfrvxiowkcuedfhoixzipxrkzbvpusslsgfjocvidnpsnkqdfnnzzawzsslwnvvjyoignsfbxkgrokzyusxikxumrxlzzrnbtrixxfioormoyyejashrowjqqzifacecvoruwkuessttlexvdptuvodoavsjaepvrfvbdhumtuvxufzzyowiswokioyjtzzmevttheeyjqcldllxvjraeyflthntsmipaoyjixygbtbvbnnrmlwwkeikhnnmlfspjgmcxwbjyhomfjdcnogqjviggklplpznfwjydkxzjkoskvqvnxfzdrsmooyciwulvtlmvnjbbmffureoilszlonibbcwfsjzguxqrjwypwrskhrttvnqoqisdfuifqnabzbvyzgbxfvmcomneykfmycevnrcsyqclamfxskmsxreptpxqxqidvjbuduktnwwoztvkuebfdigmjqfuolqzvjincchlmbrxpqgguwuyhrdtwqkdlqidlxzqktgzktihvlwsbysjeykiwokyqaskjjngovbagspyspeghutyoeahhgynzsyaszlirmlekpboywqdliumihwnsnwjc";
        assert_eq!(
            Solution::longest_palindrome("cbbd".to_string()),
            "bb".to_string()
        );
    }
}
