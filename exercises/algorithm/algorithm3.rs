/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
// I AM  DONE
use std::ptr;
fn sort<T>(array: &mut [T]) where T: Ord {
    do_sort(array, 0, array.len());
}

fn do_sort<T> (arr: &mut [T], left:usize, right:usize) where T: Ord {
    if left >= right {
        return;
    }
    let pivot_index = left;
    let mut i = left;
    let mut j = i + 1;
    while i < right && j < right {
        if arr[j] <= arr[pivot_index] {
            i += 1;
            swap(arr, i, j);
        }
        j += 1;
    }
    swap(arr, pivot_index, i);
    do_sort(arr, left, i);
    do_sort(arr, i + 1, right);
}

fn swap<T>(arr: &mut [T], i: usize, j: usize) {
    unsafe {
        let pa: *mut T = &mut arr[i];
        let pb: *mut T = &mut arr[j];
        ptr::swap(pa, pb)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}