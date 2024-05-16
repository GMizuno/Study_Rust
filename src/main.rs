fn do_something<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn main() {}
