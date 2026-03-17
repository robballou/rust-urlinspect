# urlinspect

This creates a simple CLI tool for inspecting URLs.

Created as an exercise in learning and practicing Rust programming.

## Usage

```shell
# provide one or more URLs
urlinspect [URL...]

# provide no URLs to get prompted to input the URL to inspect
# (useful for avoiding having a URL in your shell history)
urlinspect
```

## Example

Output information about this URL _and_ the URL contained in the query parameters:

```shell
urlinspect https://example.com/\?redirect\=https%3A%2F%2Fexample.com%3Fpage%3Dsomething
```

```
url: https://example.com/?redirect=https%3A%2F%2Fexample.com%3Fpage%3Dsomething
scheme: https
host: example.com
fragment: null
query:
- key: redirect
  value: https://example.com?page=something
  url_details:
    url: https://example.com/?page=something
    scheme: https
    host: example.com
    fragment: null
    query:
    - key: page
      value: something
      url_details: null
```

## Build

```shell
cargo build
```
