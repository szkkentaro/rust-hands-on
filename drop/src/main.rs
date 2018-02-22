struct HasDrop;
impl Drop for HasDrop {
    fn drop(&mut self) {
        println!("Dropping!");
    }
}
struct Firework {
    strength: i32
}
impl Drop for Firework {
    fn drop(&mut self) {
       println!("BOOM times {}!!!", self.strength);
    }
}
fn main() {
    let _x = HasDrop;

    let _firecracker = Firework{strength: 100};
    let _tnt = Firework{strength:1};
} // x is out of scope here now, then drop() is called
