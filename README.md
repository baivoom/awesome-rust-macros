# awesome-rust-macros

All credits to the authors

List some useful and productive MACRO crates (Ordered randomly and means nothing)

## Paste

(https://github.com/dtolnay/paste)[https://github.com/dtolnay/paste]    

```rust
use paste::paste;

macro_rules! make_a_struct_and_getters {
    ($name:ident { $($field:ident),* }) => {
        // Define a struct. This expands to:
        //
        //     pub struct S {
        //         a: String,
        //         b: String,
        //         c: String,
        //     }
        pub struct $name {
            $(
                $field: String,
            )*
        }

        // Build an impl block with getters. This expands to:
        //
        //     impl S {
        //         pub fn get_a(&self) -> &str { &self.a }
        //         pub fn get_b(&self) -> &str { &self.b }
        //         pub fn get_c(&self) -> &str { &self.c }
        //     }
        paste! {
            impl $name {
                $(
                    pub fn [<get_ $field>](&self) -> &str {
                        &self.$field
                    }
                )*
            }
        }
    }
}

make_a_struct_and_getters!(S { a, b, c });

fn call_some_getters(s: &S) -> bool {
    s.get_a() == s.get_b() && s.get_c().is_empty()
}
```

## Nestify

(https://github.com/snowfoxsh/nestify)[https://github.com/snowfoxsh/nestify]

```rust
nest! {
    struct UserProfile {
        name: String,
        address: struct Address {
            street: String,
            city: String,
        },
        preferences: struct Preferences {
            newsletter: bool,
        },
    }
}
```



