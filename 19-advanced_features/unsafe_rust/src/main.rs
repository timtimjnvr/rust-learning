use std::slice;

// Accessing an immutable static variable is safe.
// values in a static variable have a fixed address in memory.
// static variables can be mutable but accessing and modifying static variable is unsafe
// because in mutli threaded environment is difficult to ensure there are no data races
static mut COUNTER: u32 = 0;

// exter is a FFI
// defines functions and enable a different programming language to call those functions.
// C defines which application binary interface (ABI) the external function uses:
extern "C" {
    // unsafe function because other programming languages don't enforce Rust rules and guarantees.
    fn abs(input: i32) -> i32;
}

fn main() {
    let mut num = 5;

    // we can create raw pointers in non unsafe code
    // using as to cast an immutable/mutable reference into a raw pointer
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // this could be dangerous but works
    let address = 0x012345usize;
    let _r = address as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
    // this won't compile
    // dangerous()

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];
    // split_at_mut is safe (however it wraps unsafe code but is not labbeled as 'unsafe')
    // indeed Rust’s borrow checker can’t understand that we’re borrowing different parts of the slice
    // it only knows that we’re borrowing from the same slice twice.
    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // reimplementation as a function
    let (a, b) = split_at_mut(r, 3);

    // interact with code written in another language
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    // access the raw pointer of a slice (*mut i32)
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            // from_raw_parts_mut is unsafe because it must trust that this pointer is valid.
            slice::from_raw_parts_mut(ptr, mid),
            // add is unsafe because it must trust that the offset location is also a valid pointer
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
// make a function calleable by other language
#[no_mangle] // to tell the Rust compiler not to change the name of this function.
             // it does not require unsafe.
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// a trait is unsafe when at least one of it's methods has some invariants the compiler can't verify.
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}
