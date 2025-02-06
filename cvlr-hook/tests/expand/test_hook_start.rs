use cvlr_hook::cvlr_hook_on_entry;

fn hook() {
    ();
}

#[cvlr_hook_on_entry(hook())]
fn t1() {
    // hook inserted here
    println!("t1");
}