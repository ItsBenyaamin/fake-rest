# fake-rest
Fake rest is a fake API generator using a config file to help you develop clients.

# Usage
It's very simple to use. just create a `whatever.toml` file and put these lines in it:
check out 
```
[config]
port = 7000

[[data]]
method = "GET"
path = "/hello"
result_type = "direct"
result = ""
status_code = 200
```
Then, start the program like: 
`./fake-rest -c /path/to/file`
check the `example/server.toml` file for more advance options.
That's it, have fun.

# what next?
FakeRest support `headers, query-strings` and not `body` for now.
I will add `form, multipart, etc` body as soon as possible to check and condition on those fields too.