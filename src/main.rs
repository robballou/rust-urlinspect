use std::{ env, io::{ self, Write } };
use url::{ Url };

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        // No args were passed, so prompt for a URL:
        let mut input = String::new();
        print!("URL: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Invalid URL");

        // try to output what they wrote
        let url_result = inspect_url(input);
        match url_result {
            Ok(url) => output_url(url, 0),
            Err(_err) => panic!("Invalid URL"),
        }
    } else {
        for possible_url in args[1..].to_vec() {
            let url_result = inspect_url(possible_url);
            match url_result {
                Ok(url) => output_url(url, 0),
                Err(_err) => panic!("Invalid URL"),
            }
        }
    }
}

/// Parse the URL (if possible)
fn inspect_url(possible_url: String) -> Result<Url, url::ParseError> {
    let url_data = Url::parse(&possible_url.trim());
    match url_data {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Output URL information
fn output_url(url: Url, indent: usize) {
    let url_as_string = url.as_str();
    let indent_as_string = "\t".repeat(indent);
    println!("{}{}", indent_as_string, truncate(url_as_string, 64));
    println!("{}- scheme: {}", indent_as_string, url.scheme());

    if let Some(host) = url.host() {
        println!("{}- host: {}", indent_as_string, host);
    }

    let pairs = url.query_pairs();
    if pairs.count() > 0 {
        println!("{}- query:", indent_as_string);
        for pair in pairs {
            println!("{}\t{}: {}", indent_as_string, pair.0, pair.1);
            let value_is_url_result = inspect_url(pair.1.to_string());
            if let Ok(value_url) = value_is_url_result {
                output_url(value_url, indent + 2);
            }
        }
    }

    if let Some(fragment) = url.fragment() {
        println!("{}- fragment: {}", indent_as_string, fragment);
    }
}

/// Truncate a string at a max size, adding ellipsis if the string is truncated.
fn truncate(s: &str, max_chars: usize) -> String {
    match s.char_indices().nth(max_chars) {
        None => String::from(s),
        Some((idx, _)) => {
            let mut truncated_string = String::from(&s[..idx]);
            truncated_string.push_str("…");
            truncated_string
        }
    }
}
