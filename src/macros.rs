/// Colored type (alias, env) expansion for printing to stdout/stderr
#[macro_export]
macro_rules! ctype_exp {
    ($a:expr) => {
        $a.to_string().green().bold()
    };
}
