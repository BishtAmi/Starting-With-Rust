fn main()
{
    let a = 2;
    let res = stack_only(a);
    dbg !(res);
}

fn stack_only(b : i32)->i32
{
    let c = 3;
    return b + c + stack_and_heap();
}
fn stack_and_heap()->i32
{

    // Declaration of SmartPointer(Deallocates Heap Automatically)  in Rust
    let e = Box::new (7);
    return *e;
}