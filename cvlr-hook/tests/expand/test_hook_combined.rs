use cvlr_hook::{cvlr_hook_on_entry, cvlr_hook_on_exit};

fn hook_start() {
    ();
}

fn hook_end() {
    ();
}


#[cvlr_hook_on_entry(hook_start())]
#[cvlr_hook_on_exit(hook_end())]
fn tmp() -> Result<()> {
    // hook start inserted here
    // hook end inserted here
    Ok(())
}

fn t3() {
    assert_eq!(tmp(), Ok(()));
}

#[cvlr_hook_on_entry(hook_start())]
#[cvlr_hook_on_exit(hook_end())]
fn t4() {
    // hook start inserted here
    assert_eq!(1, 1);
    // hook end inserted here
}

#[cvlr_hook_on_entry(hook_start())]
fn abs(x : i32) -> i32 {
    // hook start inserted here
    if x >= 0 { 
        println!("x is positive");
        x 
    } else { 
        println!("x is negative");
        -x 
    }
}

#[cvlr_hook_on_exit(hook_end())]
fn abs2(x : i32) -> i32 {
    // hook end inserted here
    if x >= 0 { 
        println!("x is positive");
        x 
    } else { 
        println!("x is negative");
        -x 
    }
}