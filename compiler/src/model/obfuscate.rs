// pub fn obfuscate(string: &str) -> String {
//     string.chars().map(|c| {
//         match c {
//             'a'..='m' | 'A'..='M' => ((c as u8) + 13) as char,
//             'n'..='z' | 'N'..='Z' => ((c as u8) - 13) as char,
//             _ => c
//         }
//     }).collect()
// }
