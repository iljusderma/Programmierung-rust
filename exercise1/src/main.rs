fn main() {
    let x: i32 = 5;
    
    println!("x: {:p}", &x);
  
    borrow(&x);
    copy(x.clone());
  }
  
  // This function should borrow "x"
  fn borrow(y: &i32) {
    println!("y: {:p}", y);
  }
  
  // This method should copy "x"
  fn copy(z: i32) {
    println!("z: {:p}", &z);
  }
  