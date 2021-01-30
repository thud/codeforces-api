use hex;
use select::document::Document;
use select::predicate::{Class, Descendant, Name};
use sha2::{Digest, Sha512};
use std::time::SystemTime;

use super::error::*;
use super::responses;

const API_STUB: &str = "https://codeforces.com/api/";

#[derive(Debug)]
pub enum CFBlogEntryCommand {
    Comments { blog_entry_id: i64 },
    View { blog_entry_id: i64 },
}

#[derive(Debug)]
pub enum CFContestCommand {
    Hacks {
        contest_id: i64,
    },
    List {
        gym: Option<bool>,
    },
    RatingChanges {
        contest_id: i64,
    },
    Standings {
        contest_id: i64,
        from: Option<i64>,
        count: Option<i64>,
        handles: Option<Vec<String>>,
        room: Option<i64>,
        show_unofficial: Option<bool>,
    },
    Status {
        contest_id: i64,
        handle: Option<String>,
        from: Option<i64>,
        count: Option<i64>,
    },
}

#[derive(Debug)]
pub enum CFProblemsetCommand {
    Problems {
        tags: Option<Vec<String>>,
        problemset_name: Option<String>,
    },
    RecentStatus {
        count: i64,
        problemset_name: Option<String>,
    },
    RecentActions {
        max_count: i64,
    },
}

#[derive(Debug)]
pub enum CFUserCommand {
    BlogEntries {
        handle: String,
    },
    Friends {
        only_online: Option<bool>,
    },
    Info {
        handles: Vec<String>,
    },
    RatedList {
        active_only: Option<bool>,
    },
    Rating {
        handle: String,
    },
    Status {
        handle: String,
        from: Option<i64>,
        count: Option<i64>,
    },
}

/*impl From<&CFBlogEntryCommand> for BTreeMap {
    fn from(command: &CFBlogEntryCommand) -> Self {
        let mut map = BTreeMap::new();
        match &self {
            CFBlogEntryCommand::Comments{ blog_entry_id } => {
                map.insert("blogEntryId", blog_entry_id);
            },
            CFBlogEntryCommand::View{ blog_entry_id } => {
                map.insert("blogEntryId", blog_entry_id);
            },
        }
        map
    }
}*/

pub fn as_codeforces_api_url<T: CFAPIRequestable + std::fmt::Debug>(
    command: &T,
    api_key: &String,
    api_secret: &String,
) -> String {
    let rand = String::from("123456");
    let ctime = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let mut params = command.query_params();
    params.push(("apiKey", api_key.to_string()));
    params.push(("time", ctime.to_string()));
    params.sort();
    let mut url = String::from(API_STUB);
    url += command.method_name(); // "blogEntry.comments";
    url += "?";
    let mut to_hash = String::new();
    to_hash += &rand;
    to_hash += "/";
    to_hash += command.method_name();
    to_hash += "?";
    //to_hash += &command.query_params();
    for (key, val) in params {
        url += &key;
        to_hash += &key;
        url += "=";
        to_hash += "=";
        url += &val;
        to_hash += &val;
        url += "&";
        to_hash += "&";
    }
    to_hash.pop();
    to_hash += "#";
    to_hash += &api_secret.to_string();
    let mut hasher = Sha512::new();
    hasher.update(&to_hash);
    let api_sig = hasher.finalize();
    url += "apiSig=";
    url += &rand;
    url += &hex::encode(&api_sig);
    url
}

pub fn send_codeforces_api_req<T: CFAPIRequestable + std::fmt::Debug>(
    req: &T,
    api_key: &String,
    api_secret: &String,
) -> Result<responses::CFResult, Error> {
    let url = as_codeforces_api_url(req, api_key, api_secret);
    match get_url(&url) {
        Ok(res) => match res.json::<responses::CFResponse>() {
            Ok(json) => match json.status {
                responses::CFResponseStatus::Ok => Ok(json.result.unwrap()),
                responses::CFResponseStatus::Failed => {
                    Err(Error::CodeforcesApi(json.comment.unwrap()))
                }
            },
            Err(e) => Err(Error::Parse(e)),
        },
        Err(e) => Err(Error::Http(e)),
    }
}

pub fn send_codeforces_api_req_raw<T: CFAPIRequestable + std::fmt::Debug>(
    req: &T,
    api_key: &String,
    api_secret: &String,
) -> Result<String, Error> {
    let url = as_codeforces_api_url(req, api_key, api_secret);
    get_url_raw(&url)
}

fn get_url(
    url: &String,
) -> Result<reqwest::blocking::Response, reqwest::Error> {
    let body = reqwest::blocking::get(url);
    body
}

fn get_url_raw(url: &String) -> Result<String, Error> {
    let body = get_url(url);
    match body {
        Ok(res) => match res.text() {
            Ok(text) => Ok(text),
            Err(e) => Err(Error::Http(e)),
        },
        Err(e) => Err(Error::Http(e)),
    }
}

pub trait CFAPIRequestable {
    fn query_params(&self) -> Vec<(&'static str, String)>;
    fn method_name(&self) -> &'static str;
    fn get(
        &self,
        api_key: &String,
        api_secret: &String,
    ) -> Result<responses::CFResult, Error>;
    fn get_raw(
        &self,
        api_key: &String,
        api_secret: &String,
    ) -> Result<String, Error>;
}

impl CFAPIRequestable for CFBlogEntryCommand {
    fn query_params(&self) -> Vec<(&'static str, String)> {
        let mut res = vec![];
        match self {
            CFBlogEntryCommand::Comments { blog_entry_id } => {
                res.push(("blogEntryId", blog_entry_id.to_string()));
            }
            CFBlogEntryCommand::View { blog_entry_id } => {
                res.push(("blogEntryId", blog_entry_id.to_string()));
            }
        }
        res
    }

    fn method_name(&self) -> &'static str {
        match self {
            CFBlogEntryCommand::Comments { blog_entry_id: _ } => {
                "blogEntry.comments"
            }
            CFBlogEntryCommand::View { blog_entry_id: _ } => "blogEntry.view",
        }
    }

    fn get(
        &self,
        api_key: &String,
        api_secret: &String,
    ) -> Result<responses::CFResult, Error> {
        send_codeforces_api_req(self, api_key, api_secret)
    }

    fn get_raw(
        &self,
        api_key: &String,
        api_secret: &String,
    ) -> Result<String, Error> {
        send_codeforces_api_req_raw(self, api_key, api_secret)
    }
}

impl CFAPIRequestable for CFContestCommand {
    fn query_params(&self) -> Vec<(&'static str, String)> {
        let mut res = vec![];
        match self {
            CFContestCommand::Hacks { contest_id } => {
                res.push(("contestId", contest_id.to_string()));
            }
            CFContestCommand::List { gym } => {
                if let Some(b) = gym {
                    res.push((
                        "gym",
                        if *b {
                            "true".to_string()
                        } else {
                            "false".to_string()
                        },
                    ));
                }
            }
            CFContestCommand::RatingChanges { contest_id } => {
                res.push(("contestId", contest_id.to_string()));
            }
            CFContestCommand::Standings {
                contest_id,
                from,
                count,
                handles,
                room,
                show_unofficial,
            } => {
                res.push(("contestId", contest_id.to_string()));
                if let Some(i) = from {
                    res.push(("from", i.to_string()));
                }
                if let Some(i) = count {
                    res.push(("count", i.to_string()));
                }
                if let Some(v) = handles {
                    res.push(("handles", v.join(";")));
                }
                if let Some(i) = room {
                    res.push(("room", i.to_string()));
                }
                if let Some(b) = show_unofficial {
                    res.push((
                        "showUnofficial",
                        if *b {
                            "true".to_string()
                        } else {
                            "false".to_string()
                        },
                    ));
                }
            }
            CFContestCommand::Status {
                contest_id,
                handle,
                from,
                count,
            } => {
                res.push(("contestId", contest_id.to_string()));
                if let Some(s) = handle {
                    res.push(("handle", s.to_string()));
                }
                if let Some(i) = from {
                    res.push(("from", i.to_string()));
                }
                if let Some(i) = count {
                    res.push(("count", i.to_string()));
                }
            }
        }
        res
    }

    fn method_name(&self) -> &'static str {
        match self {
            CFContestCommand::Hacks { .. } => "contest.hacks",
            CFContestCommand::List { .. } => "contest.list",
            CFContestCommand::RatingChanges { .. } => "contest.ratingChanges",
            CFContestCommand::Standings { .. } => "contest.standings",
            CFContestCommand::Status { .. } => "contest.status",
        }
    }

    fn get(
        &self,
        api_key: &String,
        api_secret: &String,
    ) -> Result<responses::CFResult, Error> {
        send_codeforces_api_req(self, api_key, api_secret)
    }

    fn get_raw(
        &self,
        api_key: &String,
        api_secret: &String,
    ) -> Result<String, Error> {
        send_codeforces_api_req_raw(self, api_key, api_secret)
    }
}

impl CFAPIRequestable for CFProblemsetCommand {
    fn query_params(&self) -> Vec<(&'static str, String)> {
        let mut res = vec![];
        match self {
            CFProblemsetCommand::Problems {
                tags,
                problemset_name,
            } => {
                if let Some(v) = tags {
                    res.push(("tags", v.join(";")));
                }
                if let Some(s) = problemset_name {
                    res.push(("problemsetName", s.to_string()));
                }
            }
            CFProblemsetCommand::RecentStatus {
                count,
                problemset_name,
            } => {
                res.push(("count", count.to_string()));
                if let Some(s) = problemset_name {
                    res.push(("problemsetName", s.to_string()));
                }
            }
            CFProblemsetCommand::RecentActions { max_count } => {
                res.push(("maxCount", max_count.to_string()));
            }
        }
        res
    }

    fn method_name(&self) -> &'static str {
        match self {
            CFProblemsetCommand::Problems { .. } => "problemset.problems",
            CFProblemsetCommand::RecentStatus { .. } => {
                "problemset.recentStatus"
            }
            CFProblemsetCommand::RecentActions { .. } => {
                "problemset.recentActions"
            }
        }
    }

    fn get(
        &self,
        api_key: &String,
        api_secret: &String,
    ) -> Result<responses::CFResult, Error> {
        send_codeforces_api_req(self, api_key, api_secret)
    }

    fn get_raw(
        &self,
        api_key: &String,
        api_secret: &String,
    ) -> Result<String, Error> {
        send_codeforces_api_req_raw(self, api_key, api_secret)
    }
}

impl CFAPIRequestable for CFUserCommand {
    fn query_params(&self) -> Vec<(&'static str, String)> {
        let mut res = vec![];
        match self {
            CFUserCommand::BlogEntries { handle } => {
                res.push(("handle", handle.to_string()));
            }
            CFUserCommand::Friends { only_online } => {
                if let Some(b) = only_online {
                    res.push((
                        "onlyOnline",
                        if *b {
                            "true".to_string()
                        } else {
                            "false".to_string()
                        },
                    ));
                }
            }
            CFUserCommand::Info { handles } => {
                res.push(("handles", handles.join(";")));
            }
            CFUserCommand::RatedList { active_only } => {
                if let Some(b) = active_only {
                    res.push((
                        "activeOnly",
                        if *b {
                            "true".to_string()
                        } else {
                            "false".to_string()
                        },
                    ));
                }
            }
            CFUserCommand::Rating { handle } => {
                res.push(("handle", handle.to_string()));
            }
            CFUserCommand::Status {
                handle,
                from,
                count,
            } => {
                res.push(("handle", handle.to_string()));
                if let Some(i) = from {
                    res.push(("from", i.to_string()));
                }
                if let Some(i) = count {
                    res.push(("count", i.to_string()));
                }
            }
        }
        res
    }

    fn method_name(&self) -> &'static str {
        match self {
            CFUserCommand::BlogEntries { .. } => "user.blogEntries",
            CFUserCommand::Friends { .. } => "user.friends",
            CFUserCommand::Info { .. } => "user.info",
            CFUserCommand::RatedList { .. } => "user.ratedList",
            CFUserCommand::Rating { .. } => "user.rating",
            CFUserCommand::Status { .. } => "user.status",
        }
    }

    fn get(
        &self,
        api_key: &String,
        api_secret: &String,
    ) -> Result<responses::CFResult, Error> {
        send_codeforces_api_req(self, api_key, api_secret)
    }

    fn get_raw(
        &self,
        api_key: &String,
        api_secret: &String,
    ) -> Result<String, Error> {
        send_codeforces_api_req_raw(self, api_key, api_secret)
    }
}

pub fn fetch_testcases_for_problem(
    contest_id: &i64,
    problem_index: &String,
) -> Result<Vec<String>, Error> {
    let url = "https://codeforces.com/problemset/problem/".to_string()
        + &contest_id.to_string()
        + "/"
        + &problem_index.to_string();
    match get_url(&url) {
        Ok(res) => {
            let document = Document::from_read(res).unwrap();
            let testcases: Vec<String> = document
                .find(Descendant(Class("input"), Name("pre")))
                .map(|e| e.inner_html()) // TODO replace `<br>` with `\n`
                .collect();
            if testcases.is_empty() {
                Err(Error::User("No testcase input found for this problem."))
            } else {
                Ok(testcases)
            }
        }
        Err(e) => Err(Error::Http(e)),
    }
}

impl responses::CFProblem {
    pub fn fetch_testcases(&mut self) -> Result<Vec<String>, Error> {
        if self.contest_id.is_none() {
            return Err(Error::User("problem.contest_id field is required."));
        }
        if self.index.is_none() {
            return Err(Error::User("problem.index field is required."));
        }
        let testcases = fetch_testcases_for_problem(
            &self.contest_id.unwrap(),
            &self.index.as_ref().unwrap(),
        );
        if let Ok(ref v) = testcases {
            self.input_testcases = Some(v.to_vec());
        }
        testcases
    }
}
