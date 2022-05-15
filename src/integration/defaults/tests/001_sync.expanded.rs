struct test;
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for test {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            test => ::core::fmt::Formatter::write_str(f, "test"),
        }
    }
}
fn f(a: u32) -> u32 {
    a
}
fn main() {
    f(2 * 3);
}
