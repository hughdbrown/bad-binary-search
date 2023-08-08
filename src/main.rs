use std::panic::{
    catch_unwind,
};

fn binary_search(data: &[i32], value: i32) -> i32 {
    let mut lo = 0;
    let mut hi = data.len() - 1;
    while lo <= hi {
        let mid = (lo + hi) / 2;
        println!("{lo} {mid} {hi}");
        if data[mid] == value {
            return mid as i32;
        }
        else if data[mid] > value {
            hi = mid - 1;
        }
        else {
            lo = mid + 1;
        }
    }
    -1
}

fn bad_1() {
    println!("-----\nbad_1: try to overflow calculation of mid");
    let mut data = vec![0; 1 << 31];
    data.push(1);
    let i = binary_search(&data, 1);
    println!("{}", i);
}

fn bad_2() {
    println!("-----\nbad_2: overflow calculation of hi");
    let data = vec![1; 1];
    let panic = catch_unwind(|| {
        println!("{}", binary_search(&data, 0));
    }).is_err();
    assert!(panic);
}

fn bad_3() {
    println!("-----\nbad_3: empty vector to search");
    let data = vec![];
    let panic = catch_unwind(|| {
        println!("{}", binary_search(&data, 0));
    }).is_err();
    assert!(panic);
}

fn main() {
    bad_1();
    bad_2();
    bad_3();
}
