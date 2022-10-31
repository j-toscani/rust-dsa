pub fn search_linear<T : PartialEq>(haystack: &[T], needle: T) -> bool {
    let mut found = false;

    for el in haystack.into_iter() {
        found = el == &needle;

        if found {
            break;
        }
    }

    found
}
