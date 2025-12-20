mod circular;
mod deref;
mod drop;
mod rc;
mod refcell;

use circular::*;
use deref::*;
use drop::*;
use rc::*;
use refcell::*;

fn main() {
    deref_tutorial();
    println!("==========");
    drop_tutorial();
    println!("==========");
    rc_tutorial();
    println!("==========");
    refcell_tutorial();
    println!("==========");
    circular_tutorial();
}
