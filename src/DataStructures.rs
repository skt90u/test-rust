pub struct S01 {} // Define a struct with named fields.
pub struct S02 { pub x: i32 }	// Define struct with named field x of type T.
struct S03 (i32);	// Define "tupled" struct with numbered field .0 of type T.
struct S04;	// Define zero sized unit struct. Occupies no space, optimized away.
enum E01 {}	// Define an enum, algebraic data types, tagged unions.
enum E02 { A, B(), C {} }	// Define variants of enum; can be unit- A, tuple-B() and struct-like C{}.
enum E03 { A = 1 }	// If variants are only unit-like, allow discriminant values, REF e.g., for FFI.
enum E04 {}	// Enum w/o variants is uninhabited, REF can't be instantiated, c. 'never' ↓ 🝖
union U {}	// Unsafe C-like union REF for FFI compatibility. 🝖
// static X01: T = T();	// Global variable with 'static lifetime, single memory location.
// const X02: T = T();	// Defines constant, BK EX REF copied into a temporary when used.
// let x01: T;	// Allocate T bytes on stack1 bound as x. Assignable once, not mutable.
// let mut x02: T;	// Like let, but allow for mutability BK EX and mutable borrow.2
// x03 = y;	// Moves y to x, invalidating y if T is not Copy, STD and copying y otherwise.