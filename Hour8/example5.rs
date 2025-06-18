fn main(){
let my_closure = | num: i32 | { num + 200 }; // create a closure
let num = 100;
println!("{}", my_closure(num)); // call the closure
}
Output:
300
Explanation:
Closure is an anonymous function in Rust Language.
“my_closure” is a closure name.
| num: i32 | is an argument using | | rather than ( ).
{ num + 200 }; is the closure body.