fn main() {
    let mut v = vec![1, 2, 3];
    //Instead of using raw pointers, we use safe methods to modify the vector 
    for i in 0..v.len(){
        v[i] += 1;
    }
    println!("{:?}", v);
} 