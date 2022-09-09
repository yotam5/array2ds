# array2ds
## May God let rust have better constant generics


Array2ds is a none-bloated library in rust for rust by rust for handling staticly sized 2d array/grids
to run the examples:
cargo run --example 'example_name'

- Uses only Safe Rust, no usage of "unsafe"
- Works I guess
- ✨Magic ✨

## Features

- creating 2d arrays
- iterating over rows both mutable and not
- grid sized cant be changed after created
- can index with (row,column) or [row,column] and easily overload to other ways
- UPCOMING: iterating over columns, and diagnoly

```rust
#[test]
fn test_index()
{
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(1..4096);
    let r = rng.gen_range(1..4096);
    let c = rng.gen_range(1..4096);
    let arr = Array2d::filled_with(n, r, c);
    //println!("{:?}", &arr);
    for _ in 0..70
    {
        let cc = rng.gen_range(0..(c - 1));
        let rr = rng.gen_range(0..(r - 1));
        //println!("len: {} rr: {} cc: {}", arr.column_count() * arr.row_count(), &rr, &cc);
        //println!("index: {} is [{},{}]",arr.d2_index_d1(&[rr,cc]),&rr,&cc);
        assert_eq!(arr[[rr, cc]], arr[(rr, cc)]);
    }
}
```

## License

MIT


