pub fn add(left: usize, right: usize) -> usize {
    println!("d1 0.1.0");
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
