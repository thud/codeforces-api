mod obj;
pub use obj::{requests, responses};
pub use obj::error::Error;

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
