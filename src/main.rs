use array2ds::array2d::*;

#[allow(unused_variables)]

#[derive(Debug, Clone)]
struct W {
    s: String,
    q: i32,
}

fn main() {
    let mut mm = Array2d::filled_with(
        W {
            s: "hello".to_string(),
            q: 19,
        },
        5_usize,
        4_usize,
    );
    //println!("{:?}",&mm) ;
    mm[[1_usize, 3_usize]] = W {
        s: "after".to_string(),
        q: 20,
    };

    /*for ro in 0..mm.row_count() {
        for f in mm.iter_mut_row(ro) {
            *f = W {
                s: "ok".to_string(),
                q: 324,
            };
        }
    }*/

    for m in mm.iter_mut_rows()
    {
        for r in m {
            *r = W { s: "works".to_string(), q: 22 };
        }
    }

    println!("{:?}", &mm);
}
