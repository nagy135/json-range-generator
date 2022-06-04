# json-range-generator

Idea is to generate json (pretty/minified) from json-like structure that allows ranges.

```
json-range-generator '{"<1-3>":"20.000"}'
json-range-generator '{"<1-3>":{"<1-2>": "20.000"}}'
json-range-generator '{"settings":{"<1-3>": "20.000"}}'
json-range-generator '{"settings":{"<1-2>": "20.000"}}'
```

where `<x-y>` produces multiple keys with same values.

For example:
```
json-range-generator '{"<1-3>":{"<1-2>": "20.000"}}'
```
Produces:
```json
{
    "1": {
        "1": "20.000",
        "2": "20.000",
    },
    "2": {
        "1": "20.000",
        "2": "20.000",
    },
    "3": {
        "1": "20.000",
        "2": "20.000",
    }
}
```

## Usage

either provide argument like
```sh
$ json-range-generator '{"<1-2>": 1}'
```

or pipe it in stdin
```sh
$ echo '{"<1-2>": 1}' | json-range-generator
```

using `-p` or `--pretty` you get pretty printed json

Ranges can have text around them, so that
```sh
$ json-range-generator '{"before_<1-2>_after": 1}' -p
```
produces

```json
{
    "before_1_after": 1,
    "before_2_after": 1,
}
```

### limitations and edge cases
* Only first range in key is taken into account, others are treated as text
* Invalid ranges result in key/value removal
