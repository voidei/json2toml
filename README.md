# json2toml

A simple CLI tool that converts json files to toml files.

Inspired heavily by [toml2json](https://github.com/woodruffw/toml2json/).

**Usage:**

```sh
json2toml [filename]
```

**Example:**

Input:

```json
// file.json
{
    "key": "value"
}
```

Output:

```toml
# output.toml
key = "value"

```

## IMPORTANT NOTE

The output file is **always** named `output.toml`, I might fix that later to match the input filename if I can figure out how
