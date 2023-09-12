enum Option2<T> {
    None,
    Some(T),
}

impl<T> Option2<T> {
    pub fn map(&self, f: fn(&T) -> T) -> Option2<T> {
        return match self {
            Option2::None => Option2::None,
            Option2::Some(value) => Option2::Some(f(value)),
        }
    }

    pub fn is_some(&self) -> bool {
        return match self {
            Option2::None => false,
            Option2::Some(_) => true,
        }
    }

}


fn main() {
    let opt = Some(5);
    let foo = Option2::Some(5);

    opt.map(|x| x + 5);
    let foo = foo.map(|x| x + 5);

    if foo.is_some() {
        println!("foo is some");
    } else {
        println!("foo is none");
    }
}

