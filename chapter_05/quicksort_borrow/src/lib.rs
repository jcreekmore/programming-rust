pub fn quicksort(list: &mut [i32]) {
    if list.len() <= 1 {
        return;
    } else if list.len() == 2 {
        if list[0] > list[1] {
            list.swap(0, 1);
        }
        return;
    }

    let pivot = list[0];

    let mut i = 0;
    for j in 0..list.len() {
        if list[j] < pivot {
            list.swap(i, j);
            i += 1;
        }
    }

    let (left, right) = list.split_at_mut(i);

    quicksort(left);
    quicksort(right);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut v1 = vec![5, 3, 4, 1, 6, 2];
        {
            quicksort(&mut v1);
        }
        assert_eq!(&v1, &[1, 2, 3, 4, 5, 6]);
    }
}
