use cvlr_log::cvlr_log;

fn main() {
    cvlr_log!(1u8 => "u8");
    cvlr_log!(2u16 => "u16");
    cvlr_log!(3u32 => "u32");
    cvlr_log!(4u64 => "u64");
    cvlr_log!(5usize => "usize");
    cvlr_log!(6u128 => "u128");
    
    cvlr_log!(-1i8 => "i8");
    cvlr_log!(-2i16 => "i16");
    cvlr_log!(-3i32 => "i32");
    cvlr_log!(-4i64 => "i64");
    cvlr_log!(-5i128 => "i128");
}
