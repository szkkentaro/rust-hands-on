struct HasDrop;
impl Drop for HasDrop {
    fn drop(&mut self) {
        println!("Dropping!");
    }
}
fn main() {
    let _x = HasDrop;
} // x is out of scope here now, then drop() is called
