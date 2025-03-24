/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: Ord + Clone>(array: &mut [T]) {
    let mid = if array.len() <= 1 {
        return;
    } else {
        array.len() / 2
    };
    let mut left = array[..mid].to_vec();
    let mut right = array[mid..].to_vec();

    sort(&mut left);
    sort(&mut right);

    merge(array, &left, &right);
}

fn merge<T: Ord + Clone>(arr: &mut [T], left: &[T], right: &[T]) {
    let (mut i, mut j, mut k) = (0, 0, 0);

    while let (Some(x), Some(y)) = (left.get(i), right.get(j)) {
        arr[k] = if x <= y {
            i += 1;
            x.clone()
        } else {
            j += 1;
            y.clone()
        };
        k += 1;
    }

    while let Some(x) = left.get(i) {
        arr[k] = x.clone();
        k += 1;
        i += 1;
    }

    while let Some(y) = right.get(j) {
        arr[k] = y.clone();
        k += 1;
        j += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
