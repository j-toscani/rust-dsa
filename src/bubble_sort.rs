pub fn sort<T: Ord>(arr: &mut [T]) {
    for run in 0..arr.len() {
        for item in 0..arr.len() - run - 1 {
            if arr[item] > arr[item + 1] {
                arr.swap(item,item+1)
            }
        }
    }
}
