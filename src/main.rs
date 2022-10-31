mod linear_search;
mod binary_search;
mod bubble_sort;

fn main() {
    let mut haystack1 = [1,4,5,32,55,2,3];
    let haystack2 = ["e", "b", "c"];

    let found1 = binary_search::search_binary(&haystack1, 9);
    let found2 = linear_search::search_linear(&haystack2, "e");
    let number: i32 = 5/2;  
    
    println!("{}", number);
    println!("The element is in haystack: {}", found1);
    println!("The element is in haystack: {}", found2);
    println!("{:?}", &haystack1);

    bubble_sort::sort(&mut haystack1);
    println!("{:?}", &haystack1);
}
