# unwrap_goto  

unwrap Option an Result data. When data is wrap with Some or Ok, it will unwrap, otherwise can goto `return`/`cotinue`/`break` as you want.  

When none will return.

```rust
#[macro_use]
extern crate unwrap_goto;
fn do_none_is_return(m: &Vec<Option<i32>>) {
    let mut k = 0;
    for v in m {
        println!("imcome:{:?}", v);
        k = some_or_return!(v);
        println!("wrap value:{:?}", k);
    }
    assert_eq!(k, 1);
    0
}

fn none_is_return() {
    let m = vec![Some(1), None, Some(1)];
    let n = do_none_is_return(&m);
    println!("return value:{:?}", n);
} 

```
 

```rust
fn do_none_is_return(m: &Vec<Option<i32>>) -> i64 {
    let mut k = 0;
    for v in m {
        println!("imcome:{:?}", v);
        k = some_or_return!(v, 8);
        println!("wrap value:{:?}", k);
    }
    assert_eq!(k, 1);
    0
}

fn none_is_return() {
    let m = vec![Some(1), None, Some(1)];
    let n = do_none_is_return(&m);
    println!("return value:{:?}", n);
} 

```
 
