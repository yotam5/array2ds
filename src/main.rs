use array2ds::Array2d;

#[derive(Debug, Clone)]
struct W {
    s: String,
    q: i32,
}

fn main() {
    //let mut mm = Array2d::<W>::new(5,10);

    let cap = 5000;
    let mut mf = Vec::<W>::with_capacity(cap);
    let mut mn = Vec::<Option<W>>::with_capacity(cap);
    let mut ml = Array2d::<W>::filled_with(
        W {
            s: "hello world".to_string(),
            q: 100,
        },
        500,
        10,
    );
    let mut wq = Vec::<i32>::new();
    for i in 0..cap {
        mf.push(W {
            s: "hello world".to_string(),
            q: 100,
        });

        mn.push(Some(W {
            s: "hello world".to_string(),
            q: 100,
        }));
    }
    mf.shrink_to_fit();
    mn.shrink_to_fit();

    let mq = mf.into_boxed_slice();
    println!("{}", std::mem::size_of_val(&*mq));
    println!("{}", std::mem::size_of_val(&*mn));
    println!("{:?}", &ml[(1, 1)]);
    // println!("{}", std::mem::size_of_val(&**ml));
}
