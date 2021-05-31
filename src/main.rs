use abc_derive::Abc;

#[derive(Abc)]
#[derive(Debug, PartialEq, Eq)]
struct Xyz;


fn main() {
    print_all_attributes();
}
