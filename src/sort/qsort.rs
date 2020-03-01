
pub fn sort<T>(data: &mut Vec<T>)
where
    T: Ord,
{
    let n = data.len();
    quick_sort(data, 0, n-1);
}

fn quick_sort<T>(data: &mut Vec<T>, l: usize, r: usize)
where
    T: Ord,
{
    if l >= r {
        return;
    }
    let mid = (l+r)/2;
    data.swap(l,mid);
    let mut i = l;
    let mut j = r;
    while j > i {
        while j > i && data[j].gt(&data[l]) { j -= 1; }
        while i < j && data[i].le(&data[l]) { i += 1; }
        if i != j {
            data.swap(i,j);
            j-=1;
        }
    }
    data.swap(l, j);
    if j > l {
        quick_sort(data, l, j-1);
    }
    quick_sort(data, j+1, r);
}

#[cfg(test)]
mod tests {
    use super::sort;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_sort() {
        let mut rng = thread_rng();
        for _ in 0..500 {
            let n = rng.gen_range(1, 1000);
            let origin = (1..=n).into_iter().map(|_| rng.gen_range(-100, 100)).collect::<Vec<i32>>();
            let mut t1 = origin.clone();
            let mut t2 = origin.clone();
            sort(&mut t1);
            t2.sort();
            assert_eq!(t1, t2, "origin: {:?}, t1: {:?}, t2: {:?}", origin, t1, t2);
        }
    }
}