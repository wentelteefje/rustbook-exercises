# rustbook exercises
## 1. General Considerations
- To model the exercises to the text, we should carefully note which concepts are presented and strive to incorporate as many of these concepts into the exercises as possible (ideally touching on all concepts in the exercises—obviously).
- There are generally different categories of exercises. Some examples include:
    - Exercises asking the reader to extend an example already given in the chapter by adding some functionality (typically not very interesting on their own, but they reinforce the point). An example is the following:
    > IPv4 addresses are classified into classes (A, B, C, D, E). The class for a given IPv4 address can be derived from its first byte (also known as the first octet). If it's between 0-127, it's A; from 128-191, it's B; from 192-223, it's C; from 224-239, it's D; and for all other values, it's classified as E. Given the IpAddr enum from the text, implement a method `ip_class` to return the correct class as an `Option<char>`. For IPv6 addresses return `None`.
    
    This would be a correct solution to this exercise:
    ```rust
    impl IpAddr {
        fn ip_class(&self) -> Option<char> {
            match self {
                IpAddr::V4(a, _, _, _) => Some(match a {
                    0..=127 => 'A',
                    128..=191 => 'B',
                    192..=223 => 'C',
                    224..=239 => 'D',
                    _ => 'E',
                }),
                IpAddr::V6(_) => None,
            }
        }
    }
    ```
    - Standalone exercises that ask the user to implement something close to being useful or interesting using concepts learned over the course of the current chapter (e.g., the TuringMachine example). These types of exercises could potentially become more involved in later chapters.
- All exercises should come with a solution to prevent readers from getting stuck or frustrated and also to provide good code patterns/discourage anti-patterns.
- I think the approach taken in the Rust exam, where reading tests might give the reader a few hints and also gives them a feeling of progressively approaching the final solution, should be taken.

## 2. Chapter 6 of the Rustbook
### 2.1 Presented Concepts
These lists can be used to keep track of which material to include in exercises.
#### In "6.1 Defining an Enum":
- Enum definition, i.e. `enum Name { ... }`.
- Instance creation and double colon syntax, e.g. `let four = IpAddrKind::V4;`.
- Enum type as function parameter, e.g. `rustfn route(ip_kind: IpAddrKind) {}`.
- Storing data inside of enums, e.g. 
```rust
enum IpAddr { 
	V4(String),
	V6(String), 
}
let home = IpAddr::V4(String::from("127.0.0.1"));
```
- Using different types for each variant in Enum definition
```rust
enum IpAddr {
	V4(u8, u8, u8, u8),
	V6(String),
}
```
- Defining methods on Enums using `impl`
- Special enum `Option<T>`
#### In "6.2 The match Control Flow Construct":
...you get the point.