# csv_validator
A basic CLI for constraining and validation CSV values

## Examples
All files are found in the [examples](./examples) directory

#### Constraints example file
The constraint names are found in [constraints.rs](./src/constraints.rs)

```json
{
    "foo": ["NotEmpty", "IsInteger"],
    "bar": ["NotEmpty"]
}
```

#### Valid CSV
```csv
foo,bar,baz
1,2,3
4,5,6
```

```
$ cat examples/valid.csv | cargo run -- -c examples/constraints.json
```

#### Assorted invalid CSV
```csv
foo,bar,baz
"hi",2,
4,,6
```

```
$ cat examples/assortment.csv | cargo run -- -c examples/constraints.json
row_num,col_name,value,message
0,foo,hi,Must be an integer
1,bar,,Must be non-empty
```
