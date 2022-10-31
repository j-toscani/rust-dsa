pub fn search_binary<T: Ord>(haystack: &[T], needle: T) -> bool {
    let mut high = haystack.len();
    let mut low: usize = 0;
    let mut found = false;

    while low < high {
        let m = low + (high - low) / 2;
        let v = haystack.get(m).expect("Calculating 'm' wrong!");

        if v == &needle {
            found = true;
            break;
        } else if v > &needle {
            high = m;
        } else {
            low = m;
        }
        
        if high - low == 1 && v != &needle {
            break;
        }
    }

    return found
}
