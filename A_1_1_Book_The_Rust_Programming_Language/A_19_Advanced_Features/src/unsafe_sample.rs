use std::slice;

fn raw_pointer_sample() {
    let address = 0x012345usize;
    let r = address as *const i32;

    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        *r2 = 50;
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

fn unsafe_function_sample() {
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
}

fn split_at_mut_sample() {
    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let ptr = values.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            let m1 = slice::from_raw_parts_mut(ptr, mid);
            let len_m2 = ptr.add(mid);
            let m2 = slice::from_raw_parts_mut(len_m2, len - mid);
            (m1, m2)
        }
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

fn creating_arbitrary_memory_location_error_sample() {
    let address = 0x01234usize;
    let r = address as *mut i32;

    let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
    println!("{:?}", values); // process didn't exit successfully: `target\debug\A_19_Advanced_Features.exe` (exit code: 0xc0000005, STATUS_ACCESS_VIOLATION)
}

pub fn sample() {
    raw_pointer_sample();
    unsafe_function_sample();
    split_at_mut_sample();
    creating_arbitrary_memory_location_error_sample();
}
