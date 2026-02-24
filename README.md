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

```shell
urlinspect https://example.com/\?redirect\=https%3A%2F%2Fexample.com%3Fpage%3Dsomething
```

```
https://example.com/?redirect=https%3A%2F%2Fexample.com%3Fpage%3…
- scheme: https
- host: example.com
- query:
	redirect: https://example.com?page=something
		https://example.com/?page=something
		- scheme: https
		- host: example.com
		- query:
			page: something
```
