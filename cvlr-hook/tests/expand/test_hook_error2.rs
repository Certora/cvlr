use cvlr_hook::cvlr_hook_on_entry;

fn hook() {
    ();
}

fn hook2() {
    ();
}

#[cvlr_hook_on_entry(hook(), hook2())]
fn t1() {
    // hook inserted here
    println!("t1");
}

fn main() {}