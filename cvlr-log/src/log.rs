use crate::CvlrLogger;

pub trait CvlrLog {
    fn log(&self, tag: &str, logger: &mut CvlrLogger);
}

#[inline(always)]
pub fn cvlr_log_with<T: CvlrLog>(tag: &str, val: &T, logger: &mut CvlrLogger) {
    val.log(tag, logger);
}

#[inline(always)]
pub fn cvlr_log<T: CvlrLog>(tag: &str, val: &T) {
    let mut logger = CvlrLogger::new();
    val.log(tag, &mut logger);
}

#[macro_export]
macro_rules! cvlr_log {
    ($v:expr => $t:expr) => {
        $crate::cvlr_log($t, &($v));
    };

    ($v:expr => $t:expr ; $l:ident) => {
        $crate::cvlr_log_with($t, &($v), $l)
    };

    ($v:expr) => {
        $crate::cvlr_log! { $v => stringify!($v) }
    };

    ($v:expr, $( $vs:expr ),+) => {
        $crate::cvlr_log! { $v }
        $crate::cvlr_log! { $( $vs ),+ }
    };
}

pub use cvlr_log as clog;

macro_rules! impl_cvlr_log_for_uint {
    ($t:ty) => {
        impl CvlrLog for $t {
            fn log(&self, tag: &str, logger: &mut CvlrLogger) {
                logger.log_u64(tag, *self as u64);
            }
        }
    };
}


impl_cvlr_log_for_uint!(bool);
impl_cvlr_log_for_uint!(u8);
impl_cvlr_log_for_uint!(u16);
impl_cvlr_log_for_uint!(u32);
impl_cvlr_log_for_uint!(u64);
impl_cvlr_log_for_uint!(usize);

macro_rules! impl_cvlr_log_for_int {
    ($t:ty) => {
        impl CvlrLog for $t {
            fn log(&self, tag: &str, logger: &mut CvlrLogger) {
                logger.log_i64(tag, *self as i64);
            }
        }
    };
}

impl_cvlr_log_for_int!(i8);
impl_cvlr_log_for_int!(i16);
impl_cvlr_log_for_int!(i32);
impl_cvlr_log_for_int!(i64);

impl<T: CvlrLog> CvlrLog for &T {
    fn log(&self, tag: &str, logger: &mut CvlrLogger) {
        (**self).log(tag, logger);
    }
}

impl CvlrLog for &str {
    fn log(&self, _tag: &str, logger: &mut CvlrLogger) {
        logger.log(*self);
    }
}

impl CvlrLog for () {
    fn log(&self, tag: &str, logger: &mut CvlrLogger) {
        logger.log_str(tag, "()");
    }
}

impl<T: CvlrLog> CvlrLog for Option<T> {
    fn log(&self, tag: &str, logger: &mut CvlrLogger) {
        if let Some(v) = self {
            v.log(tag, logger);
        } else {
            logger.log_str(tag, "None");
        }
    }
}

impl<T: CvlrLog, E: CvlrLog> CvlrLog for Result<T, E> {
    fn log(&self, tag: &str, logger: &mut CvlrLogger) {
        match self {
            Ok(v) => {
                logger.log("Ok");
                v.log(tag, logger)
            }
            Err(v) => {
                logger.log("Err");
                v.log(tag, logger)
            }
        }
    }
}

#[cfg(feature = "mathint")]
impl CvlrLog for cvlr_mathint::NativeInt {
    fn log(&self, tag: &str, logger: &mut CvlrLogger) {
        logger.log_u64(tag, self.as_internal());
    }
}
