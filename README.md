# About
Fake-Rest is a fake API generator using a config file to help you develop clients.

It's EASY AS HELL.

# Usage
It's very simple to use. just create a `whatever.toml` file and put these lines in it:
```
[config]
port = 7000
host = "127.0.0.1"

[[data]]
method = "GET"
path = "/hello"
result_type = "direct"
result = ""
status_code = 200
```
Then, start the program like: 
`./fake-rest -c /path/to/file`
check the [`example/server.toml`](https://github.com/graymind75/fake-rest/blob/main/examples/server.toml) file for more advance options.

That's it, have fun.

# what's next?
FakeRest support `headers, query-strings` except `body` for now.

I will add `form, multipart, etc` body as soon as possible to check and condition on those fields too.

### Features:
- [x] Serve String as response
- [x] Serve file as response
- [x] Serve file for download
- [x] Get host address from config file
- [ ] Path values
- [ ] Body Parsing:
    - [ ] Form
    - [ ] Multipart
- [x] Header value checking
