fn find_smallest(v: &Vec<i32>) -> usize{
    let mut smallest_index = 0 as usize;
    let mut smallest = v[smallest_index];
    for i in 1..v.len() {
        if v[i] < smallest {
            smallest_index = i;
            smallest = v[i];
        }
    }
    smallest_index
}

fn selection_sort(mut v: Vec<i32>) -> Vec<i32> {
    let mut sorted = Vec::<i32>::new();
    for _ in 0..v.len() {
        let smallest_index = find_smallest(&v);
        sorted.push(v.remove(smallest_index));
    }
    sorted
}

fn quick_sort(v: Vec<i32>) -> Vec<i32> {
    if v.len() < 2 {
        return v;
    }

    let pivot = v[0];
    let mut less = Vec::<i32>::new();
    let mut greater = Vec::<i32>::new();
    for i in &v[1..] {
        if i <= &pivot {
            less.push(*i);
        }
        else {
            greater.push(*i);
        }
    }
    
    let mut result = quick_sort(less); 
    result.push(pivot);
    result.extend(quick_sort(greater));

    result
}

fn main() {
    println!("===Selection Sort===");
    let vec = vec![5, 2, 3, 4, 9];
    let sorted = selection_sort(vec);

    for x in &sorted {
        println!("{x}");
    }
    
    println!("===Quick Sort===");
    let vec = vec![5, 2, 3, 4, 9];
    let sorted = quick_sort(vec);

    for x in &sorted {
        println!("{x}");
    }
}