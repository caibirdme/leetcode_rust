use rand::{thread_rng, Rng};
use std::cmp::Ordering;

fn qsort<T> (arr: &mut [T])
where
    T: Ord + Clone
{
    let n = arr.len();
    if n <=1 {
        return;
    }
    if n == 2 {
        if arr[0].cmp(&arr[1]) == Ordering::Greater {
            arr.swap(0, 1);
        }
        return;
    }
    let pos = thread_rng().gen_range(0, n);
    let piv = arr[pos].clone();
    let (mut l, mut r) = (0,n-1);
    while l < r {
        while l < r && !(arr[l].cmp(&piv) == Ordering::Greater) {
            l += 1;
        }
        while l < r && !(arr[r].cmp(&piv) == Ordering::Less) {
            r -= 1;
        }
        if l < r {
            arr.swap(l, r);
        }
    }
    qsort(&mut arr[..l]);
    qsort(&mut arr[l..]);
}

#[cfg(test)]
mod tests {
    use super::qsort;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_qsort() {
        let mut rng = thread_rng();
        for _ in 0..2 {
            let mut arr = vec![];
            for _ in 0..15 {
                arr.push(rng.gen_range(0, 100));
            }
            let origin = arr.clone();
            let mut expect = arr.clone();
            expect.sort();
            qsort(&mut arr);
            assert_eq!(arr, expect, "arr: {:?}", origin);
        }
    }
}