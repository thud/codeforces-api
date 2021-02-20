# codeforces-api

A rust crate for interfacing with Codeforces resources with authentication.
It provides the full functionality of the
[Codeforces API](https://codeforces.com/apiHelp) as well as the ability to
fetch testcases for a given problem.

# Usage

```rust
use codeforces_api::requests::{CFBlogEntryCommand, CFAPIRequestable};
use codeforces_api::responses::CFResult;

fn main() {
    // This is equivalent to the Codeforces `blogEntry.view` API method.
    let x = CFBlogEntryCommand::View {
        blog_entry_id: 82347,
    };

    // The `.get(..)` method on API commands returns a result with either
    // an error or an `Ok(CFResult)`.
    match x.get("<api_key>", "<api_secret>") {
        Ok(CFResult::CFBlogEntry(blog_entry)) => {
            assert_eq!(blog_entry.id, 82347);
            println!("Your blog entry: {:?}", blog_entry);
        },
        Ok(_) => {
            // In very rare cases, an unexpected type may be returned by
            // `.get()`. If this happens, then you may wish to throw a
            // custom error.
            panic!("`.get()` returned an unexpected type.");
        }
        Err(e) => {
            // Errors returned are of a custom Error type. This could be
            // returned if, for example, an invalid API key/secret was used
            // or if there was no internet connection.
            panic!("something failed {:?}", e);
        }
    }
}
```

[Docs](https://docs.rs/codeforces-api) |
[Crate](https://crates.io/crates/codeforces-api) |
[License](LICENSE)
