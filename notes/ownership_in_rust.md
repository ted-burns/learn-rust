# Ownership

### Why?

- Guarantess memory safety
- Removes need for garbage collection?

### Rules

Each value in rust has a variable, which is it's owner. There can only be one owner at a time, and when the owner goes out of scope, the value is dropped.

### Moving pointers

For non-literals, when you assign one to the other, you move the reference (or invalidating the old refernce), essentially moving the first variable out of scope.

```rust
let s1 = String::from("hello");
let s2 = s2;

println!("{}, world!", s2); // prints "hello, world!"
println!("{}, world!", s1); // compile time error
```

You can use `clone()`, which creates a copy if you need.

### Copy & Drop

Anything that implements the `Copy` trait (like integers) can be reassigned without invalidating the prior reference because the value is copied. 

Things that implement `Copy` cannot also implement `Drop`



### Calling functions

When you use something like a String as a parameter, you actually take away ownership from the calling context (thus invalidating the previous reference).



## References

References are written as `&type`, which allows you to pass a reference without passing the value itself.

You can dereference with `*`

When you pass a reference somewhere, you still retain ownership of it. The one given the reference is borrowing it.

You can only have one mutable reference, declared by `&mut <var>`. Also, if there is a mutable reference somewhere, you cannot have immutable references and vice versa.

### Dangling References

You have dangling references when a pointer allocated to something on the stack is used after the stack frame is popped. This is prevented at compile time in Rust.

## Slices

Basically list slices (and are refrences). They can be used on strings and arrays.