use std::convert::Infallible;

use nvim_oxi::{Dictionary, Function, Object};

#[nvim_oxi::plugin]
fn calc() -> Dictionary {
    let add =
        Function::from_fn(|(a, b): (i32, i32)| Ok::<_, Infallible>(a + b));

    let multiply =
        Function::from_fn(|(a, b): (i32, i32)| Ok::<_, Infallible>(a * b));

    let compute = Function::from_fn(
        |(fun, a, b): (Function<(i32, i32), i32>, i32, i32)| fun.call((a, b)),
    );

    Dictionary::from_iter([
        ("add", Object::from(add)),
        ("multiply", Object::from(multiply)),
        ("compute", Object::from(compute)),
    ])
}
