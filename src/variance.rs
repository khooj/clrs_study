fn input_static(a: &'static str) {
    println!("{}", a);
}

fn input_notstatic<'a>(a: &'a str) {
    println!("{}", a);
}

fn run_func<F>(input: F)
where
    for<'a> F: Fn(&'a str),
{
    input("asd");
}

fn run_func_contra<F>(input: F)
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
        run_func(input_notstatic);

        run_func_contra(input_static);
        run_func_contra(input_notstatic);
    }
}
