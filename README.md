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
json-range-generator '{<1-3>:{<1-2>: "20.000"}}'
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
