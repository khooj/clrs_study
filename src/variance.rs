fn _input_static(a: &'static str) {
    println!("{}", a);
}

fn _input_notstatic<'a>(a: &'a str) {
    println!("{}", a);
}

fn _run_func<F>(input: F)
where
    for<'a> F: Fn(&'a str),
{
    input("asd");
}

fn _run_func_contra<F>(input: F)
where
    F: Fn(&'static str),
{
    input("asd");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_contravariance() {
        // run_func(input_static);
        _run_func(_input_notstatic);

        _run_func_contra(_input_static);
        _run_func_contra(_input_notstatic);
    }
}
