A function in struct, enum, trait is called as a method.
impl Struct/Enum { // implement Struct or Enum
fn method_name(&self) -> type { // define a method
self.member // access the member variable
}
}
The &self indicates that the caller's reference is taken as a parameter.
The first parameter of the method is &self.