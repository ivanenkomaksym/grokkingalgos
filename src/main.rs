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

fn main() {
    let vec = vec![5, 2, 3, 4, 9];
    let sorted = selection_sort(vec);

    for x in &sorted {
        println!("{x}");
    }
}