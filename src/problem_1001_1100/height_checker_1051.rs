pub fn height_checker(heights: Vec<i32>) -> i32 {
    let mut sort = heights.clone();
    sort.sort();

    let mut cnt = 0;

    for i in 0..heights.len() {
        if heights[i] != sort[i] {
            cnt += 1;
        }
    }

    cnt
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01_height_checker() {
        let input: Vec<i32> = vec![1, 1, 4, 2, 1, 3];
        let expected = 3;
        assert_eq!(expected, height_checker(input));
    }

    #[test]
    fn test02_height_checker() {
        let input: Vec<i32> = vec![5, 1, 2, 3, 4];
        let expected = 5;
        assert_eq!(expected, height_checker(input));
    }

    #[test]
    fn test03_height_checker() {
        let input: Vec<i32> = vec![1, 2, 3, 4, 5];
        let expected = 0;
        assert_eq!(expected, height_checker(input));
    }
}
