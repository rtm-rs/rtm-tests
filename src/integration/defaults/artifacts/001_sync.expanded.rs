struct Test;
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for Test {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            Test => ::core::fmt::Formatter::write_str(f, "Test"),
        }
    }
}
fn f(a: u32) -> u32 {
    a
}
fn main() {
    f(2 * 3);
}
