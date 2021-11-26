use std::io;

fn main() {
    println!("Please input your morse.");

    let mut morse = String::new();
    io::stdin()
        .read_line(&mut morse)
        .expect("Failed to read line");

    let result = possibilities(&morse[0..morse.len() - 1]);
    println!("Result: {:?}", result);
}

fn possibilities(morse: &str) -> Vec<String> {
    let orign_str = "ETIANMSURWDKGO";
    let mut result = vec![];

    let mut morses = vec![morse.to_string()];
    loop {
        let mut flag = false;
        let mut tmp = vec![];
        for (_pos, _morse) in morses.iter().enumerate() {
            if _morse.contains("?") {
                tmp.push(_morse.replacen("?", ".", 1));
                tmp.push(_morse.replacen("?", "-", 1));
                flag = true;
            } else {
                tmp.push(_morse.to_string());
            }
        }

        morses = tmp;
        if flag == false {
            break;
        }
    }

    for (_pos, _morse) in morses.iter().enumerate() {
        let level = _morse.len();
        let mut current_cursor = 0;
        for l in 1..(level + 1) {
            let one_morse = _morse.as_bytes()[l - 1 as usize] as char;
            for i in current_cursor..(current_cursor + 2) {
                let val = orign_str.as_bytes()[i as usize] as char;
                
                if i % 2 == 0 && one_morse == '.' || 
                    i % 2 == 1 && one_morse == '-' || 
                    one_morse == '?' {

                    if l == level {
                        result.push(val.to_string());
                    }

                    current_cursor = 2 * (i + 1);
                }
                
            }
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_single_signal() {
        assert_eq!(possibilities("."), vec!["E"]);
        assert_eq!(possibilities(".-"), vec!["A"]);
    }

    #[test]
    fn test_a_word_with_a_single_unknown_signal() {
        assert_eq!(possibilities("?"), vec!["E", "T"]);
        assert_eq!(possibilities("?."), vec!["I", "N"]);
        assert_eq!(possibilities("?-?"), vec!["R","W","G","O"]);
    }
}
