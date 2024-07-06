pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
    let fr = time / (n - 1);
    let rem = time % (n - 1);
    if fr % 2 == 0 {
        rem + 1
    } else {
        n - rem
    }
}
