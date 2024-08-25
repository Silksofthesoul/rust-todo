pub fn strPadToEnd(str: &str, pad: char, len: usize) -> String {
    let mut res = str.to_string();
    let max = res.len() + len;
    while res.len() < max {
        res.push(pad);
    }
    res
}
