enum RSEnum {
    Foo2(Option<i32>),
    Foo(fn() -> i32),
    Bar(String),
    Baz(Vec<String>)
}

fn bar() -> i32 {
    return 5;
}

fn main() {
    let foo = RSEnum::Foo(bar);

    if let RSEnum::Foo(value) = foo {
    }

    match foo{
        RSEnum::Foo2(Some(value)) => { },
        RSEnum::Foo2(None) => { },
        RSEnum::Foo(value) => { },
        _ => {}
    }
}

