use rand::Rng;

use array2ds::array2d::*;

#[test]
fn test_filled_with()
{
    let mut rng = rand::thread_rng();
    for _ in 0..10
    {
        let num = rng.gen_range(0..2048);
        let arr = Array2d::filled_with(num, rng.gen_range(1..2048),
                                       rng.gen_range(1..2048));

        for row in arr.iter_rows() {
            for val in row {
                assert_eq!(*val, num)
            }
        }
    }
}

#[test]
fn iter_rows()
{
    let (r, c) = (20, 30);
    let n = 10;
    let arr = Array2d::filled_with(n, r, c);
    assert_eq!(arr.iter_rows().count(), r);
    arr.iter_rows().for_each(|rr|
        {
            let roc = rr.iter().filter(|rn| **rn == n).count();
            assert_eq!(c, roc)
        }
    );
}

#[test]
fn test_column_mut()
{
    let r = 10;
    let c = 20;
    let v = 15;
    let vr = 20;
    let mut arr = Array2d::filled_with(v, r, c);

    for val in arr.column_mut(c - 1)
    {
        *val = vr;
    }

    for val in arr.column(c - 1){
        assert_eq!(*val,vr);
    }
}


#[test]
fn test_column()
{
    let r = 10;
    let c = 20;
    let v = 15;
    let vr = 20;
    let mut arr = Array2d::filled_with(v, r, c);

    for qr in 0..r
    {
       arr[[qr,c - 1]] = vr;
    }

    for val in arr.column(c - 1){
        assert_eq!(*val,vr);
    }
}

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

#[test]
fn test_iter()
{
    let r = 10;
    let c = 5;
    let n = 8;
    let board = Array2d::filled_with(n, r, c);
    let n_count = board.iter().filter(|x| **x == n).count();
    assert_eq!(n_count, r * c);
}

#[test]
fn fill_with_default()
{
    let r = 10;
    let c = 13;
    let num_default = i32::default();
    let arr: Array2d<i32> = Array2d::filled_with_default(r, c);
    let count = arr.iter().filter(|x| **x == num_default).count();
    assert_eq!(count, r * c);
}

#[test]
fn iter_mut_rows()
{
    let fill_num = 1;
    let replace_with = 24;
    let r = 10;
    let c = 6;
    let mut arr = Array2d::filled_with(fill_num, r, c);
    for row in arr.iter_mut_rows()
    {
        for num in row {
            *num = replace_with;
        }
    }

    let count_replace_with = arr.iter().filter(|&n| *n == replace_with).count();
    assert_eq!(count_replace_with, r * c);
}
