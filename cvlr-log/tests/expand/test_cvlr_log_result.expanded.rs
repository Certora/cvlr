use cvlr_log::cvlr_log;
fn main() {
    let ok_value: Result<u64, &str> = Ok(42);
    let err_value: Result<u64, &str> = Err("error message");
    ::cvlr_log::cvlr_log("ok_result", &(ok_value));
    ::cvlr_log::cvlr_log("err_result", &(err_value));
    ::cvlr_log::cvlr_log("Ok::<u64, &str>(100)", &(Ok::<u64, &str>(100)));
    ::cvlr_log::cvlr_log(
        "Err::<u64, &str>(\"failure\")",
        &(Err::<u64, &str>("failure")),
    );
}
