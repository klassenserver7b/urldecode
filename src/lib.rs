fn url_decode(url: &str) -> String {
    let mut decoded = String::from("");
    let mut iter = url.chars();

    while let Some(c) = iter.next() {
        if c == '%' {
            let byte = u8::from_str_radix(format!("{}{}", iter.next().unwrap(), iter.next().unwrap()).as_str(), 16).unwrap();
            decoded.push(byte as char);
        } else {
            decoded.push(c);
        }
    }
    decoded
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!("https://github.com", crate::decode(String::from("https%3A%2F%2Fgithub.com")));
    }
}
