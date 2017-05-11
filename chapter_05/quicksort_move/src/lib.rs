pub fn quicksort(list: Vec<i32>) -> Vec<i32> {
    if list.len() <= 1 {
        return list;
    } else if list.len() == 2 {
        if list[0] > list[1] {
            return vec![list[1], list[0]];
        } else {
            return list;
        }
    }

    let mut left = Vec::new();
    let mut right = Vec::new();

    let pivot = list[0];

    for el in &list[1..] {
        if *el <= pivot {
            left.push(*el);
        } else {
            right.push(*el);
        }
    }

    let mut left = quicksort(left);
    let right = quicksort(right);

    left.push(pivot);
    left.extend(right);

    left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v1 = vec![5, 3, 4, 1, 6, 2];
        let v2 = quicksort(v1.clone());
        assert_eq!(&v2, &[1, 2, 3, 4, 5, 6]);
    }
}
