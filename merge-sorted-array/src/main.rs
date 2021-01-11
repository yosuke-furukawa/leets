fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let num1len = nums1.len();
    let m = m as usize;
    let n = n as usize;
    for i in (0..m).rev() {
        nums1.swap(i, num1len - m + i);
    }
    let mut i = num1len - m;
    let mut j = 0;
    let mut k = 0;
        
    while i < num1len || j < n {
        if j == n || (i < num1len && nums1[i] < nums2[j]) {
            nums1.swap(k, i);
            i += 1;
        } else {
            nums1[k] = nums2[j];
            j += 1;
        }
        k += 1;
    }
}

fn main() {
    let mut num1 = vec![1,2,3,0,0,0];
    let mut num2 = vec![2,5,6];
    merge(&mut num1, 3, &mut num2, 2);
    println!("{:?}", num1);
}
