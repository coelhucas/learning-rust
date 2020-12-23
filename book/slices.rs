// Yeah, this isn't needed, it's a simple feature. But for now it's nice to have
// an example of its usage.

fn main() {
    let arr = [0, 1, 2, 3, 4, 5, 6];
    println!("{:?}", &arr[1..3]);
    // The later value in the slice (3) gets the index arr[3 - 1], which is 2.
}
