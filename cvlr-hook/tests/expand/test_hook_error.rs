use cvlr_hook::cvlr_hook_on_entry;

fn hook() {
    ();
}

// adding this hook to a struct should result
// in a compile error
#[cvlr_hook_on_entry(hook())]
struct S1 {
    a: i32
}

fn main() {}