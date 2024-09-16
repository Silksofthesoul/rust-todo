//pub fn min(vec: Vec<i32>) -> i32 {
//    let mut min = vec[0];
//    for i in 1..vec.len() {
//        if vec[i] < min {
//            min = vec[i];
//        }
//    }
//    min
//}

pub fn max(vec: Vec<i32>) -> i32 {
    let mut max = vec[0];
    for i in 1..vec.len() {
        if vec[i] > max {
            max = vec[i];
        }
    }
    max
}
