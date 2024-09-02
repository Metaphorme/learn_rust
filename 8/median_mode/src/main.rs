fn median(v: &Vec<i32>) -> Option<f64> {
    if v.is_empty() {
        return None;
    }

    let vec_len = v.len();

    let mut sorted_v = v.clone();
    sorted_v.sort(); // 排序

    if vec_len % 2 == 0 {  // 偶数个
        let m= sorted_v[vec_len / 2] as f64;
        let n= sorted_v[vec_len / 2 - 1] as f64;
        Some((m + n) / 2.0)
    } else {  // 奇数个
        let mid = vec_len / 2;  // 向下取整
        Some(sorted_v[mid] as f64)
    }
}

fn mode(v: &Vec<i32>) -> Option<Vec<i32>> {
    if v.is_empty() {
        return None;
    }

    use std::collections::HashMap;

    let mut sum = HashMap::new();

    for num in v {
        let count = sum.entry(num).or_insert(0);
        *count += 1;
    }

    let mut mode_vec = Vec::new();

    let mut max_freq: i32 = 0;

    for (&num, freq) in sum {
        if freq > max_freq {
            max_freq = freq; // 更新最大频率值
            mode_vec.clear();
            mode_vec.push(num);
        } else if freq == max_freq {
            mode_vec.push(num);
        }
    }

    Some(mode_vec)
}

fn main() {
    let list1 = vec![4, 6, 2, 4, 7, 8, 1, 5, 8, 5];

    let med = median(&list1);
    match med {
        Some(med) => println!("The median of list1 is {med}"),
        None => println!("list1 is empty!"),
    }

    let modes_vec = mode(&list1);
    match modes_vec {
        Some(modes_vec)  => println!("The modes of list1 is {:?}", modes_vec),
        None => println!("list1 is empty!"),
    }

}
