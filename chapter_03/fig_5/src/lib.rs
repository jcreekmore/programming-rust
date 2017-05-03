#[allow(dead_code)]

pub enum Cowboy {
    Good,
    Bad,
    Ugly
}

pub enum Rating {
    Awful = -2,
    Bad   = -1,
    Ok    =  0,
    Good  =  1,
    Great =  2
}

pub fn cowboy_to_int(cowboy: Cowboy) -> i32 {
    cowboy as i32
}

#[cfg(compile_error)]
fn int_to_cowboy(val: i32) -> Cowboy {
    // error: non-scalar cast `i32` as `Cowboy`
    val as Cowboy
}
