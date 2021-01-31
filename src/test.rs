use crate::obj::error::*;
use crate::obj::requests::*;
use crate::obj::responses::*;

// The following api keys are purely for testing, they refer to the
// user "MikeWazowski". Not to be abused.
const TEST_API_KEY: &str = "7dd1c6a92bf0a6cb22b0e9fa9c08d1dac4948023";
const TEST_SECRET: &str = "acc9a26087164935d62610ed693c063463e123c2";

fn get_api_keys() -> (String, String) {
    (TEST_API_KEY.to_string(), TEST_SECRET.to_string())
}

#[test]
fn test_api_bad_blogentry() {
    let (k, s) = get_api_keys();
    let x = CFBlogEntryCommand::Comments { blog_entry_id: -1 };
    match x.get(&k, &s) {
        Err(Error::CodeforcesApi(e)) => {
            println!("Received expected error: {}", e);
        }
        _ => {
            panic!("Fail, Expected error from Codeforces Api.");
        }
    }
}

#[test]
fn test_api_user() {
    let (k, s) = get_api_keys();
    let x = CFUserCommand::Friends { only_online: None };
    match x.get(&k, &s) {
        Ok(CFResult::CFFriends(v)) => {
            println!("Received friends list successfully: {}", CFResult::CFFriends(v));
        }
        Ok(_) => {
            panic!("Fail, user.friends response not parsed into Vec<String>");
        }
        Err(e) => {
            panic!("Fail, request failed: {}", e);
        }
    }
}

#[test]
fn test_api_user_status() {
    let (k, s) = get_api_keys();
    let x = CFUserCommand::Status {
        handle: "thud".to_string(),
        from: None,
        count: Some(3),
    };
    match x.get(&k, &s) {
        Ok(CFResult::CFSubmissionVec(v)) => {
            println!(
                "Received user submissions (user.status) successfully: {}",
                CFResult::CFSubmissionVec(v)
            );
        }
        Ok(_) => {
            panic!(
                "Fail, user.status response not parsed into CFSubmissionVec"
            );
        }
        Err(e) => {
            panic!("Fail, request failed: {}", e);
        }
    }
}

#[test]
fn test_api_problem() {
    let (k, s) = get_api_keys();
    let x = CFContestCommand::Standings {
        contest_id: 1477,
        from: Some(1),
        count: Some(1),
        handles: Some(vec!["thud".to_owned()]),
        room: None,
        show_unofficial: Some(false),
    };
    match x.get(&k, &s) {
        Ok(CFResult::CFContestStandings(d)) => {
            println!("Received contest standings (contest.standings) successfully: {}", CFResult::CFContestStandings(d));
        }
        Ok(_) => {
            panic!(
                "Fail, contest.standings response not parsed into CFContestStandings"
            );
        }
        Err(e) => {
            panic!("Fail, request failed: {}", e);
        }
    }
}

#[test]
fn test_fetch_testcase() {
    let mut p = CFProblem {
        contest_id: Some(1477),
        problemset_name: None,
        index: Some("B".to_string()),
        name: "Nezzar and Binary String".to_string(),
        problem_type: CFProblemType::Programming,
        points: Some(1000.0),
        rating: Some(1900),
        tags: vec!["data structures".to_string(), "greedy".to_string()],
        input_testcases: None,
    };
    match p.fetch_testcases() {
        Ok(v) => {
            assert!(p.input_testcases.unwrap().len() > 0);
            println!("Received problem testcases successfully: {:?}", v);
        }
        Err(e) => {
            panic!("Fail, testcase request failed: {}", e);
        }
    }
}
