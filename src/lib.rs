#![allow(clippy::needless_doctest_main)]

//! codeforces-api crate - interact with the
//! [Codeforces](https://codeforces.com/) API.
//!
//! This crate uses the [official API](https://codeforces.com/apiHelp) of the
//! Codeforces platform.
//!
//! As of this version, an API key and secret is required with every request
//! made. Instructions to generate these can be found
//! [here](https://codeforces.com/apiHelp) (in the Authorization section).
//!
//! This crate solely uses `reqwest`'s blocking network client meaning that all
//! requests made through this crate are also blocking. No client object is
//! provided with this crate and thus no rate-limiting provided. This could
//! also impact performance since a new `reqwest` client is created and
//! destroyed with every network interaction.
//!
//! # Usage
//!
//! ```no_run
//! use codeforces_api::requests::{CFBlogEntryCommand, CFAPIRequestable};
//! use codeforces_api::responses::CFResult;
//!
//! fn main() {
//!     // This is equivalent to the Codeforces `blogEntry.view` API method.
//!     let x = CFBlogEntryCommand::View {
//!         blog_entry_id: 82347,
//!     };
//!
//!     // The `.get(..)` method on API commands returns a result with either
//!     // an error or an `Ok(CFResult)`.
//!     match x.get("<api_key>", "<api_secret>") {
//!         Ok(CFResult::CFBlogEntry(blog_entry)) => {
//!             assert_eq!(blog_entry.id, 82347);
//!             println!("Your blog entry: {:?}", blog_entry);
//!         },
//!         Ok(_) => {
//!             // In very rare cases, an unexpected type may be returned by
//!             // `.get()`. If this happens, then you may wish to throw a
//!             // custom error.
//!             panic!("`.get()` returned an unexpected type.");
//!         }
//!         Err(e) => {
//!             // Errors returned are of a custom Error type. This could be
//!             // returned if, for example, an invalid API key/secret was used
//!             // or if there was no internet connection.
//!             panic!("something failed {:?}", e);
//!         }
//!     }
//! }
//! ```

mod obj;
pub use obj::error::Error;
pub use obj::{requests, responses};

#[cfg(test)]
mod test;

#[doc(hidden)]
/// Testing api key not to be abused, refers to user "MikeWazowski". Not to be
/// abused.
pub const TEST_API_KEY: &str = "7dd1c6a92bf0a6cb22b0e9fa9c08d1dac4948023";
#[doc(hidden)]
/// Testing api secret not to be abused, refers to user "MikeWazowski". Not to
/// be abused.
pub const TEST_API_SECRET: &str = "acc9a26087164935d62610ed693c063463e123c2";
