use crate::obj::requests::*;

// The following api keys are purely for testing, they refer to the
// user "MikeWazowski". Not to be abused.
const TEST_API_KEY: &str = "7dd1c6a92bf0a6cb22b0e9fa9c08d1dac4948023";
const TEST_SECRET: &str = "acc9a26087164935d62610ed693c063463e123c2";

fn get_api_keys() -> (String, String) {
    (TEST_API_KEY.to_string(), TEST_SECRET.to_string())
}

#[test]
fn test_api_blogentry() {
    let (k, s) = get_api_keys();
    let x = CFBlogEntryCommand::Comments { blog_entry_id: -1 };
    println!("{:?}", x.get(&k, &s));
}

#[test]
fn test_api_user() {
    let (k, s) = get_api_keys();
    let x = CFUserCommand::Friends { only_online: None };
    println!("{:?}", x.get(&k, &s));
}

#[test]
fn test_api_user_status() {
    let (k, s) = get_api_keys();
    let x = CFUserCommand::Status {
        handle: "thud".to_string(),
        from: None,
        count: Some(3),
    };
    println!("{:?}", x.get(&k, &s));
}
