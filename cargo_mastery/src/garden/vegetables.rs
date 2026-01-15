// 这里的路径是 crate::garden::vegetables

#[derive(Debug)]
pub struct Asparagus;

pub fn harvest() {
    println!("Garden: Harvesting {:?}!", Asparagus);
}
