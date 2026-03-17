use std::{ env, io::{ self, Write }, process };
use url::{ Url, form_urlencoded::Parse };
use serde::{Serialize, Deserialize};

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
        if let Ok(url) = url_result {
            let encoded = encode_url(url);
            match encoded {
                Ok(url_as_string) => println!("{}", url_as_string),
                Err(_err) => {
                    eprintln!("Invalid URL");
                    process::exit(1)
                },
            }
        }
    } else {
        for possible_url in args[1..].to_vec() {
            let url_result = inspect_url(possible_url);
            if let Ok(url) = url_result {
               let encoded = encode_url(url);
                match encoded {
                    Ok(url_as_string) => println!("{}", url_as_string),
                    Err(_err) => {
                        eprintln!("Invalid URL");
                        process::exit(1)
                    },
                }
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

#[derive(Serialize, Deserialize)]
struct UrlQueryParameter {
    key: String,
    value: String,
    url_details: Option<UrlDetails>
}
#[derive(Serialize, Deserialize)]
struct UrlDetails {
    url: String,
    scheme: String,
    host: Option<String>,
    fragment: Option<String>,
    query: Vec<UrlQueryParameter>
}

fn option_to_option_string<T:ToString>(option: Option<T>) -> Option<String> {
    let mut option_string: Option<String> = None;
    if let Some(option_value) = option {
        option_string = Some(option_value.to_string());
    }
    option_string
}

fn query_pairs_to_params(pairs: Parse<'_>) -> Vec<UrlQueryParameter> {
    let mut params: Vec<UrlQueryParameter> = vec![];
    if pairs.count() > 0 {
        for pair in pairs {
            let value_is_url_result = inspect_url(pair.1.to_string());
            let mut url_details: Option<UrlDetails> = None;
            if let Ok(url) = value_is_url_result {
                url_details = Some(url_to_url_details(url))
            }

            params.push(UrlQueryParameter {
                key: pair.0.to_string(),
                value: pair.1.to_string(),
                url_details: url_details
            });

        }
    }
    params
}

fn url_to_url_details(url: Url) -> UrlDetails {
    let url_as_string = url.as_str();

    let pairs = url.query_pairs();
    let params: Vec<UrlQueryParameter> = query_pairs_to_params(pairs);

    let details = UrlDetails {
        url: url_as_string.to_string(),
        scheme: url.scheme().to_string(),
        host: option_to_option_string(url.host()),
        fragment: option_to_option_string(url.fragment()),
        query: params
    };
    details
}

/// Output URL information
fn encode_url(url: Url) -> Result<String, serde_yml::Error> {
    serde_yml::to_string(&url_to_url_details(url))
}
