fn main() {
    let x = 5;
    let raw = &x as *const i32;
    let points_at = unsafe { *raw };

    let mut y = 10;
    let raw_mut = &mut y as *mut i32;

    println!("{}, {:?}, {}", x, raw, points_at);
    println!("{}, {:?}", y, raw_mut);

    // 明示的
    let i: u32 = 1;
    let p_imm: *const u32 = &i as *const u32;
    // 暗黙的
    let mut m: u32 = 2;
    let p_mut: *mut u32 = &mut m; // `as *const u32` is nothing
    unsafe {
        let ref_imm: &u32 = &*p_imm;
        let ref_mut: &mut u32 = &mut *p_mut;
        println!("{:?}, {:?}", ref_imm, ref_mut);
    }
}
