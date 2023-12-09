fn main() {
    // https://www.youtube.com/watch?v=uCsD3ZGzMgE&lc=UgxvY__LBijHfVFxk4V4AaABAg&t=719s
    let n: u64 = 3005290;
    println!("{}", ((n << 1) | 1) & (u64::MAX >> n.leading_zeros()));
}
