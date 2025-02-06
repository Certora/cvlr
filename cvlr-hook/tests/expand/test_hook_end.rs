use cvlr_hook::cvlr_hook_on_exit;

fn hook() {
    ();
}

#[cvlr_hook_on_exit(hook())]
fn t1() {
    assert_eq!(1, 1);
    // hook inserted here
    assert_eq!(2, 2);
}

#[cvlr_hook_on_exit(hook())]
fn t2() {
    // hook inserted here
    assert_eq!(1, 1);
}

#[cvlr_hook_on_exit(hook())]
fn tmp() -> Result<()> {
    // hook inserted here
    Ok(())
}

fn t3() {
    assert_eq!(tmp(), Ok(()));
}