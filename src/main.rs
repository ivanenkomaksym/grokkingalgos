fn find_smallest<T>(v: &Vec<T>) -> usize where T: PartialOrd {
    let mut smallest_index = 0 as usize;
    let mut smallest = &v[smallest_index];
    for i in 1..v.len() {
        if v[i] < *smallest {
            smallest_index = i;
            smallest = &v[i];
        }
    }
    smallest_index
}

fn selection_sort<T>(mut v: Vec<T>) -> Vec<T> where T: Copy + PartialOrd {
    let mut sorted = Vec::<T>::new();
    for _ in 0..v.len() {
        let smallest_index = find_smallest(&v);
        sorted.push(v.remove(smallest_index));
    }
    sorted
}

fn quick_sort<T>(v: Vec<T>) -> Vec<T> where T: Copy + PartialOrd {
    if v.len() < 2 {
        return v;
    }

    let pivot = &v[0];
    let mut less = Vec::<T>::new();
    let mut greater = Vec::<T>::new();
    for i in &v[1..] {
        if i <= pivot {
            less.push(*i);
        }
        else {
            greater.push(*i);
        }
    }
    
    let mut result = quick_sort(less); 
    result.push(pivot.clone());
    result.extend(quick_sort(greater));

    result
}

fn main() {
    println!("===Selection Sort===");
    let vec = vec![5.4, 5.2, 5.3, 5.4, 9.0];
    let sorted = selection_sort(vec);

    for x in &sorted {
        println!("{x}");
    }
    
    println!("===Quick Sort===");
    let vec = vec![5.4, 5.2, 5.3, 5.4, 9.0];
    let sorted = quick_sort(vec);

    for x in &sorted {
        println!("{x}");
    }
}