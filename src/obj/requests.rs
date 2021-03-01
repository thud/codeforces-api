//! Contains the structs etc. required to interface with the Codeforces API
//! and the testcases scraper.

use lazy_static::lazy_static;
use rand::{self, Rng};
use regex::Regex;
use select::document::Document;
use select::predicate::{Class, Descendant, Name};
use sha2::{Digest, Sha512};
use std::time::SystemTime;

use super::error::*;
use super::responses;

const API_STUB: &str = "https://codeforces.com/api/";

/// Wrapper enum for all API methods of form `blogEntry.<method>`.
///
/// More details for the blogEntry command can be found
/// [here](https://codeforces.com/apiHelp/methods#blogEntry.comments).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CFBlogEntryCommand {
    /// Struct for sending `blogEntry.comments` requests to the Codeforces API.
    ///
    /// Returns a list of comments on a specified blog entry.
    ///
    /// If correctly parsed, the response object will be of type
    /// [`responses::CFResult::CFCommentVec`].
    ///
    /// More details for the `blogEntry.comments` command can be found
    /// [here](https://codeforces.com/apiHelp/methods#blogEntry.comments).
    ///
    /// # Examples
    ///
    /// ```
    /// # use codeforces_api::requests::*;
    /// # use codeforces_api::responses::*;
    /// # let api_key = codeforces_api::TEST_API_KEY;
    /// # let api_secret = codeforces_api::TEST_API_SECRET;
    /// let x = CFBlogEntryCommand::Comments {
    ///     blog_entry_id: 82347,
    /// };
    ///
    /// match x.get(api_key, api_secret) {
    ///     Ok(CFResult::CFCommentVec(v)) => {
    ///         // your code here
    ///     },
    ///     _ => {
    ///         panic!("API request failed");
    ///     }
    /// }
    /// ```
    Comments {
        /// blogEntryId of a blog (can be seen in the url of a blog, eg.
        /// [`/blog/entry/82347`](https://codeforces.com/blog/entry/82347)).
        blog_entry_id: i64,
    },
    /// Struct for sending `blogEntry.view` requests to the Codeforces API.
    ///
    /// Returns a specified blog entry.
    ///
    /// If correctly parsed, the response object will be of type
    /// [`responses::CFResult::CFBlogEntry`].
    ///
    /// More details for the `blogEntry.view` command can be found
    /// [here](https://codeforces.com/apiHelp/methods#blogEntry.view).
    ///
    /// # Examples
    ///
    /// ```
    /// # use codeforces_api::requests::*;
    /// # use codeforces_api::responses::*;
    /// # let api_key = codeforces_api::TEST_API_KEY;
    /// # let api_secret = codeforces_api::TEST_API_SECRET;
    /// let x = CFBlogEntryCommand::View {
    ///     blog_entry_id: 82347,
    /// };
    ///
    /// match x.get(api_key, api_secret) {
    ///     Ok(CFResult::CFBlogEntry(e)) => {
    ///         // your code here
    ///     },
    ///     _ => {
    ///         panic!("API request failed");
    ///     }
    /// }
    /// ```
    View {
        /// blogEntryId of a blog (can be seen in the url of a blog, eg.
        /// [`/blog/entry/82347`](https://codeforces.com/blog/entry/82347)).
        blog_entry_id: i64,
    },
}

/// Wrapper enum for all API methods of form `contest.<method>`.
///
/// More details for the contest command can be found
/// [here](https://codeforces.com/apiHelp/methods#contest.hacks).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CFContestCommand {
    /// Struct for sending `contest.hacks` requests to the Codeforces API.
    ///
    /// Returns list of hacks in the specified contest. Full information about
    /// hacks is available only after some time after the contest end. During
    /// the contest, a user can see only their own hacks.
    ///
    /// If correctly parsed, the response object will be of type
    /// [`responses::CFResult::CFHackVec`].
    ///
    /// More details for the `contest.hacks` command can be found
    /// [here](https://codeforces.com/apiHelp/methods#contest.hacks).
    ///
    /// # Examples
    ///
    /// ```
    /// # use codeforces_api::requests::*;
    /// # use codeforces_api::responses::*;
    /// # let api_key = codeforces_api::TEST_API_KEY;
    /// # let api_secret = codeforces_api::TEST_API_SECRET;
    /// let x = CFContestCommand::Hacks {
    ///     contest_id: 1485,
    /// };
    ///
    /// match x.get(api_key, api_secret) {
    ///     Ok(CFResult::CFHackVec(v)) => {
    ///         // your code here
    ///     },
    ///     _ => {
    ///         panic!("API request failed");
    ///     }
    /// }
    /// ```
    Hacks {
        /// contestId of a contest (can be seen in the url of a contest, eg.
        /// [`/contest/1485`](https://codeforces.com/contest/1485)).
        contest_id: i64,
    },
    /// Struct for sending `contest.list` requests to the Codeforces API.
    ///
    /// Returns information about all available contests.
    ///
    /// If correctly parsed, the response object will be of type
    /// [`responses::CFResult::CFContestVec`].
    ///
    /// More details for the `contest.list` command can be found
    /// [here](https://codeforces.com/apiHelp/methods#contest.list).
    ///
    /// # Examples
    ///
    /// ```
    /// # use codeforces_api::requests::*;
    /// # use codeforces_api::responses::*;
    /// # let api_key = codeforces_api::TEST_API_KEY;
    /// # let api_secret = codeforces_api::TEST_API_SECRET;
    /// let x = CFContestCommand::List {
    ///     gym: Some(false),
    /// };
    ///
    /// match x.get(api_key, api_secret) {
    ///     Ok(CFResult::CFContestVec(v)) => {
    ///         // your code here
    ///     },
    ///     _ => {
    ///         panic!("API request failed");
    ///     }
    /// }
    /// ```
    List {
        /// If `Some(true)`, then gym contests are returned. Otherwise, regular
        /// contests are returned.
        gym: Option<bool>,
    },
    /// Struct for sending `contest.ratingChanges` requests to the Codeforces
    /// API.
    ///
    /// Returns rating changes after a specified contest.
    ///
    /// If correctly parsed, the response object will be of type
    /// [`responses::CFResult::CFRatingChangeVec`].
    ///
    /// More details for the `contest.ratingChanges` command can be found
    /// [here](https://codeforces.com/apiHelp/methods#contest.ratingChanges).
    ///
    /// # Examples
    ///
    /// ```
    /// # use codeforces_api::requests::*;
    /// # use codeforces_api::responses::*;
    /// # let api_key = codeforces_api::TEST_API_KEY;
    /// # let api_secret = codeforces_api::TEST_API_SECRET;
    /// let x = CFContestCommand::RatingChanges {
    ///     contest_id: 1485,
    /// };
    ///
    /// match x.get(api_key, api_secret) {
    ///     Ok(CFResult::CFRatingChangeVec(v)) => {
    ///         // your code here
    ///     },
    ///     _ => {
    ///         panic!("API request failed");
    ///     }
    /// }
    /// ```
    RatingChanges {
        /// contestId of a contest (can be seen in the url of a contest, eg.
        /// [`/contest/1485`](https://codeforces.com/contest/1485)).
        contest_id: i64,
    },
    /// Struct for sending `contest.standings` requests to the Codeforces API.
    ///
    /// Returns a description of a specified contest as well as the requested
    /// part of the standings.
    ///
    /// If correctly parsed, the response object will be of type
    /// [`responses::CFResult::CFContestStandings`].
    ///
    /// More details for the `contest.standings` command can be found
    /// [here](https://codeforces.com/apiHelp/methods#contest.standings).
    ///
    /// # Examples
    ///
    /// ```
    /// # use codeforces_api::requests::*;
    /// # use codeforces_api::responses::*;
    /// # let api_key = codeforces_api::TEST_API_KEY;
    /// # let api_secret = codeforces_api::TEST_API_SECRET;
    /// let x = CFContestCommand::Standings {
    ///     contest_id: 1485,
    ///     from: Some(1),
    ///     count: Some(3),
    ///     handles: Some(vec!["thud".to_string()]),
    ///     room: None,
    ///     show_unofficial: Some(false),
    /// };
    ///
    /// match x.get(api_key, api_secret) {
    ///     Ok(CFResult::CFContestStandings(s)) => {
    ///         // your code here
    ///     },
    ///     _ => {
    ///         panic!("API request failed");
    ///     }
    /// }
    /// ```
    Standings {
        /// contestId of a contest (can be seen in the url of a contest, eg.
        /// [`/contest/1485`](https://codeforces.com/contest/1485)).
        contest_id: i64,
        /// 1-based index of the standings row to start the ranklist (most
        /// recent first).
        from: Option<i64>,
        /// Number of standing rows to return.
        count: Option<i64>,
        /// Vec of handles. No more than 10000 handles is allowed by Codeforces.
        handles: Option<Vec<String>>,
        /// If specified, then only participants from this room will be shown
        /// in the result. If not, all the participants will be shown.
        room: Option<i64>,
        /// If true, then all participants (virtual, out of competition) are
        /// shown. Otherwise, only official contestants are shown.
        show_unofficial: Option<bool>,
    },
    /// Struct for sending `contest.status` requests to the Codeforces API.
    ///
    /// Returns submissions for specified contest. Optionally can return
    /// submissions of specified user.
    ///
    /// If correctly parsed, the response object will be of type
    /// [`responses::CFResult::CFSubmissionVec`].
    ///
    /// More details for the `contest.status` command can be found
    /// [here](https://codeforces.com/apiHelp/methods#contest.status).
    ///
    /// # Examples
    ///
    /// ```
    /// # use codeforces_api::requests::*;
    /// # use codeforces_api::responses::*;
    /// # let api_key = codeforces_api::TEST_API_KEY;
    /// # let api_secret = codeforces_api::TEST_API_SECRET;
    /// let x = CFContestCommand::Status {
    ///     contest_id: 1485,
    ///     handle: None,
    ///     from: Some(1),
    ///     count: Some(3),
    /// };
    ///
    /// match x.get(api_key, api_secret) {
    ///     Ok(CFResult::CFSubmissionVec(v)) => {
    ///         // your code here
    ///     },
    ///     _ => {
    ///         panic!("API request failed");
    ///     }
    /// }
    /// ```
    Status {
        /// contestId of a contest (can be seen in the url of a contest, eg.
        /// [`/contest/1485`](https://codeforces.com/contest/1485)).
        contest_id: i64,
        /// If specified, then only this user's submissions are returned.
        handle: Option<String>,
        /// 1-based index of the standings row to start the ranklist (most
        /// recent first).
        from: Option<i64>,
        /// Number of submissions to return.
        count: Option<i64>,
    },
}

/// Wrapper enum for all API methods of form `problemset.<method>`.
///
/// More details for the problemset command can be found
/// [here](https://codeforces.com/apiHelp/methods#problemset.problems).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CFProblemsetCommand {
    /// Struct for sending `problemset.problems` requests to the Codeforces API.
    ///
    /// Returns all problems from problemset. Problems can be filtered by tags.
    ///
    /// If correctly parsed, the response object will be of type
    /// [`responses::CFResult::CFProblemset`].
    ///
    /// More details for the `problemset.problems` command can be found
    /// [here](https://codeforces.com/apiHelp/methods#problemset.problems).
    ///
    /// # Examples
    ///
    /// ```
    /// # use codeforces_api::requests::*;
    /// # use codeforces_api::responses::*;
    /// # let api_key = codeforces_api::TEST_API_KEY;
    /// # let api_secret = codeforces_api::TEST_API_SECRET;
    /// let x = CFProblemsetCommand::Problems {
    ///     tags: Some(vec!["dp".to_string(), "greedy".to_string()]),
    ///     problemset_name: None,
    /// };
    ///
    /// match x.get(api_key, api_secret) {
    ///     Ok(CFResult::CFProblemset(p)) => {
    ///         // your code here
    ///     },
    ///     _ => {
    ///         panic!("API request failed");
    ///     }
    /// }
    /// ```
    Problems {
        /// Optional Vec of tags to search for (eg. "dp").
        tags: Option<Vec<String>>,
        /// Optional custom problemset's short name, like `acmsguru`.
        problemset_name: Option<String>,
    },
    /// Struct for sending `problemset.recentStatus` requests to the Codeforces
    /// API.
    ///
    /// Returns recent submissions.
    ///
    /// If correctly parsed, the response object will be of type
    /// [`responses::CFResult::CFSubmissionVec`].
    ///
    /// More details for the `problemset.recentStatus` command can be found
    /// [here](https://codeforces.com/apiHelp/methods#problemset.recentStatus).
    ///
    /// # Examples
    ///
    /// ```
    /// # use codeforces_api::requests::*;
    /// # use codeforces_api::responses::*;
    /// # let api_key = codeforces_api::TEST_API_KEY;
    /// # let api_secret = codeforces_api::TEST_API_SECRET;
    /// let x = CFProblemsetCommand::RecentStatus {
    ///     count: 10,
    ///     problemset_name: None,
    /// };
    ///
    /// match x.get(api_key, api_secret) {
    ///     Ok(CFResult::CFSubmissionVec(v)) => {
    ///         // your code here
    ///     },
    ///     _ => {
    ///         panic!("API request failed");
    ///     }
    /// }
    /// ```
    RecentStatus {
        /// Number of submissions to return. Can be up to 1000.
        count: i64,
        /// Optional custom problemset's short name, like `acmsguru`.
        problemset_name: Option<String>,
    },
}

/// Wrapper enum for all API methods of form `user.<method>`.
///
/// More details for the user command can be found
/// [here](https://codeforces.com/apiHelp/methods#user.blogEntries).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CFUserCommand {
    /// Struct for sending `user.blogEntries` requests to the Codeforces
    /// API.
    ///
    /// Returns a list with all of a specified user's blog entries.
    ///
    /// If correctly parsed, the response object will be of type
    /// [`responses::CFResult::CFBlogEntryVec`].
    ///
    /// More details for the `user.blogEntries` command can be found
    /// [here](https://codeforces.com/apiHelp/methods#user.blogEntries).
    ///
    /// # Examples
    ///
    /// ```
    /// # use codeforces_api::requests::*;
    /// # use codeforces_api::responses::*;
    /// # let api_key = codeforces_api::TEST_API_KEY;
    /// # let api_secret = codeforces_api::TEST_API_SECRET;
    /// let x = CFUserCommand::BlogEntries {
    ///     handle: "thud".to_string(),
    /// };
    ///
    /// match x.get(api_key, api_secret) {
    ///     Ok(CFResult::CFBlogEntryVec(v)) => {
    ///         // your code here
    ///     },
    ///     _ => {
    ///         panic!("API request failed");
    ///     }
    /// }
    /// ```
    BlogEntries {
        /// Codeforces handle of the user for which to fetch blog entries.
        handle: String,
    },
    /// Struct for sending `user.friends` requests to the Codeforces API.
    ///
    /// Returns authorized user's friends (ie. the friends of the user who owns
    /// the API keys in use).
    ///
    /// If correctly parsed, the response object will be of type
    /// [`responses::CFResult::CFFriends`].
    ///
    /// More details for the `user.friends` command can be found
    /// [here](https://codeforces.com/apiHelp/methods#user.friends).
    ///
    /// # Examples
    ///
    /// ```
    /// # use codeforces_api::requests::*;
    /// # use codeforces_api::responses::*;
    /// # let api_key = codeforces_api::TEST_API_KEY;
    /// # let api_secret = codeforces_api::TEST_API_SECRET;
    /// let x = CFUserCommand::Friends {
    ///     only_online: None,
    /// };
    ///
    /// match x.get(api_key, api_secret) {
    ///     Ok(CFResult::CFFriends(v)) => {
    ///         // your code here
    ///     },
    ///     _ => {
    ///         panic!("API request failed");
    ///     }
    /// }
    /// ```
    Friends {
        /// If `Some(true)`, then only online friends are returned. Otherwise
        /// all friends are returned.
        only_online: Option<bool>,
    },
    /// Struct for sending `user.info` requests to the Codeforces API.
    ///
    /// Returns information about one or several users.
    ///
    /// If correctly parsed, the response object will be of type
    /// [`responses::CFResult::CFUserVec`].
    ///
    /// More details for the `user.info` command can be found
    /// [here](https://codeforces.com/apiHelp/methods#user.info).
    ///
    /// # Examples
    ///
    /// ```
    /// # use codeforces_api::requests::*;
    /// # use codeforces_api::responses::*;
    /// # let api_key = codeforces_api::TEST_API_KEY;
    /// # let api_secret = codeforces_api::TEST_API_SECRET;
    /// let x = CFUserCommand::Info {
    ///     handles: vec!["thud".to_string()],
    /// };
    ///
    /// match x.get(api_key, api_secret) {
    ///     Ok(CFResult::CFUserVec(v)) => {
    ///         // your code here
    ///     },
    ///     _ => {
    ///         panic!("API request failed");
    ///     }
    /// }
    /// ```
    Info {
        /// Vec of handles for which to get info for. Codeforces will return an
        /// error if this is empty.
        handles: Vec<String>,
    },
    /// Struct for sending `user.ratedList` requests to the Codeforces API.
    ///
    /// Returns the list of users who have participated in at least one rated
    /// contest.
    ///
    /// If correctly parsed, the response object will be of type
    /// [`responses::CFResult::CFUserVec`].
    ///
    /// More details for the `user.ratedList` command can be found
    /// [here](https://codeforces.com/apiHelp/methods#user.ratedList).
    ///
    /// # Examples
    ///
    /// ```
    /// # use codeforces_api::requests::*;
    /// # use codeforces_api::responses::*;
    /// # let api_key = codeforces_api::TEST_API_KEY;
    /// # let api_secret = codeforces_api::TEST_API_SECRET;
    /// let x = CFUserCommand::RatedList {
    ///     active_only: Some(true),
    /// };
    ///
    /// match x.get(api_key, api_secret) {
    ///     Ok(CFResult::CFUserVec(v)) => {
    ///         // your code here
    ///     },
    ///     _ => {
    ///         panic!("API request failed");
    ///     }
    /// }
    /// ```
    RatedList {
        /// If `Some(true)`, then only users who have participated in a rated
        /// contest during the last month are returned.
        active_only: Option<bool>,
    },
    /// Struct for sending `user.rating` requests to the Codeforces API.
    ///
    /// Returns the rating history of a specified user.
    ///
    /// If correctly parsed, the response object will be of type
    /// [`responses::CFResult::CFRatingChangeVec`].
    ///
    /// More details for the `user.rating` command can be found
    /// [here](https://codeforces.com/apiHelp/methods#user.rating).
    ///
    /// # Examples
    ///
    /// ```
    /// # use codeforces_api::requests::*;
    /// # use codeforces_api::responses::*;
    /// # let api_key = codeforces_api::TEST_API_KEY;
    /// # let api_secret = codeforces_api::TEST_API_SECRET;
    /// let x = CFUserCommand::Rating {
    ///     handle: "thud".to_string(),
    /// };
    ///
    /// match x.get(api_key, api_secret) {
    ///     Ok(CFResult::CFRatingChangeVec(v)) => {
    ///         // your code here
    ///     },
    ///     _ => {
    ///         panic!("API request failed");
    ///     }
    /// }
    /// ```
    Rating {
        /// Codeforces handle of user for which to fetch rating changes for.
        handle: String,
    },
    /// Struct for sending `user.status` requests to the Codeforces API.
    ///
    /// Returns the submissions of a specified user.
    ///
    /// If correctly parsed, the response object will be of type
    /// [`responses::CFResult::CFSubmissionVec`].
    ///
    /// More details for the `user.status` command can be found
    /// [here](https://codeforces.com/apiHelp/methods#user.status).
    ///
    /// # Examples
    ///
    /// ```
    /// # use codeforces_api::requests::*;
    /// # use codeforces_api::responses::*;
    /// # let api_key = codeforces_api::TEST_API_KEY;
    /// # let api_secret = codeforces_api::TEST_API_SECRET;
    /// let x = CFUserCommand::Status {
    ///     handle: "thud".to_string(),
    ///     from: Some(1),
    ///     count: Some(3),
    /// };
    ///
    /// match x.get(api_key, api_secret) {
    ///     Ok(CFResult::CFSubmissionVec(v)) => {
    ///         // your code here
    ///     },
    ///     _ => {
    ///         panic!("API request failed");
    ///     }
    /// }
    /// ```
    Status {
        /// Codeforces handle of user for which to fetch submissions.
        handle: String,
        /// Optional 1-based index of the first submission to return (most recent first).
        from: Option<i64>,
        /// Optional number of submissions to return.
        count: Option<i64>,
    },
}

/// Struct for sending `recentActions` requests to the Codeforces API.
///
/// Returns recent actions.
///
/// If correctly parsed, the response object will be of type
/// [`responses::CFResult::CFRecentActionVec`].
///
/// More details for the `recentActions` command can be found
/// [here](https://codeforces.com/apiHelp/methods#recentActions).
///
/// # Examples
///
/// ```
/// # use codeforces_api::requests::*;
/// # use codeforces_api::responses::*;
/// # let api_key = codeforces_api::TEST_API_KEY;
/// # let api_secret = codeforces_api::TEST_API_SECRET;
/// let x = CFRecentActionsCommand {
///     max_count: 3,
/// };
///
/// match x.get(api_key, api_secret) {
///     Ok(CFResult::CFRecentActionVec(v)) => {
///         // your code here
///     },
///     _ => {
///         panic!("API request failed");
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CFRecentActionsCommand {
    /// Number of recent actions to return. Can be up to 100.
    pub max_count: i64,
}

/// Converts CFAPIRequestable object into a Codeforces API url. Currently, only
/// authenticated interaction is implemented, though in the future, this could
/// be extended to not require it (ie. no API keys required).
fn as_codeforces_api_url<T: CFAPIRequestable + std::fmt::Debug>(
    command: &T,
    api_key: &str,
    api_secret: &str,
) -> String {
    // generate random number to be used as nonce in url.
    let mut rng = rand::thread_rng();
    let rand: String = (0..6)
        .map(|_| rng.gen_range::<u8, _>(0..=9).to_string())
        .collect();
    // get current UNIX time to be used in url.
    let ctime = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    // get command specific query params from method.
    let mut params = command.query_params();
    // add non-specific query params.
    params.push(("apiKey", api_key.to_string()));
    params.push(("time", ctime.to_string()));
    // Codeforces requires that the query params be sorted in lexicographical
    // order.
    params.sort();
    // construct url by concatenating query params to API_STUB.
    let mut url = String::from(API_STUB);
    url += command.method_name();
    url += "?";
    // construct secondary String which will be hashed for checksum.
    let mut to_hash = String::new();
    to_hash += &rand;
    to_hash += "/";
    to_hash += command.method_name();
    to_hash += "?";
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
    // hash to_hash then add to end of the url.
    let mut hasher = Sha512::new();
    hasher.update(&to_hash);
    let api_sig = hasher.finalize();
    url += "apiSig=";
    url += &rand;
    url += &hex::encode(&api_sig);
    url
}

/// Takes any CFAPIRequestable object and sends it as an API request to the
/// Codeforces servers. Made possible by `as_codeforces_url()` function.
fn send_codeforces_api_req<T: CFAPIRequestable + std::fmt::Debug>(
    req: &T,
    api_key: &str,
    api_secret: &str,
) -> Result<responses::CFResult, Error> {
    // convert request object into a url String.
    let url = as_codeforces_api_url(req, api_key, api_secret);
    match get_url(&url) {
        // if fetch was successful, then parse the JSON into a `CFResponse`.
        Ok(res) => match res.json::<responses::CFResponse>() {
            // if parse was successful, then check Codeforces response code.
            Ok(json) => match json.status {
                // if response is `Ok`, then return `CFResult` object.
                responses::CFResponseStatus::Ok => Ok(json.result.unwrap()),
                // if response is `Failed`, then return `Error::CodeforcesApi`,
                // with the returned comment as its String param.
                responses::CFResponseStatus::Failed => {
                    Err(Error::CodeforcesApi(json.comment.unwrap()))
                }
            },
            // if parse failed, then wrap reqwest parsing error with custom.
            Err(e) => Err(Error::Parse(e)),
        },
        // if fetch failed, then wrap reqwest error with custom Http.
        Err(e) => Err(Error::Http(e)),
    }
}

/// Analogous to `send_codeforces_api_req()`, only don't bother parsing.
/// Returns a JSON String or an `Error::Http`.
fn send_codeforces_api_req_raw<T: CFAPIRequestable + std::fmt::Debug>(
    req: &T,
    api_key: &str,
    api_secret: &str,
) -> Result<String, Error> {
    let url = as_codeforces_api_url(req, api_key, api_secret);
    get_url_raw(&url)
}

/// Simple blocking request to url using [`reqwest::blocking::get`]. This is
/// not very efficient for rapidly making lots of requests since the function
/// creates and destroys a new [`reqwest::Client`] with every request.
fn get_url(url: &str) -> Result<reqwest::blocking::Response, reqwest::Error> {
    reqwest::blocking::get(url)
}

/// Analogous to `get_url()`, but immediately returns just the text content of
/// the request.
fn get_url_raw(url: &str) -> Result<String, Error> {
    match get_url(url) {
        Ok(res) => match res.text() {
            Ok(text) => Ok(text),
            Err(e) => Err(Error::Http(e)),
        },
        Err(e) => Err(Error::Http(e)),
    }
}

/// Trait implemented by any type which can be sent as a request to the
/// Codeforces API.
///
/// Exposes functions which allow a type to be converted into a URL and fetched
/// from the server.
pub trait CFAPIRequestable {
    /// Method which returns a Vec of pairs (key, val) which will be mapped
    /// onto URL query parameters. Used internally and not much use for most
    /// people.
    fn query_params(&self) -> Vec<(&'static str, String)>;
    /// Method which returns a str slice of the method name (eg. "user.info").
    /// Used internally and not much use for most
    /// people.
    fn method_name(&self) -> &'static str;
    /// Fetch response from Codeforces servers.
    ///
    /// # Examples
    ///
    /// ```
    /// # use codeforces_api::requests::*;
    /// # use codeforces_api::responses::*;
    /// # let api_key = codeforces_api::TEST_API_KEY;
    /// # let api_secret = codeforces_api::TEST_API_SECRET;
    /// let x = CFUserCommand::Status {
    ///     handle: "thud".to_string(),
    ///     from: Some(1),
    ///     count: Some(3),
    /// };
    ///
    /// match x.get(api_key, api_secret) {
    ///     Ok(CFResult::CFSubmissionVec(v)) => {
    ///         // your code here
    ///     },
    ///     _ => {
    ///         panic!("API request failed");
    ///     }
    /// }
    /// ```
    fn get(
        &self,
        api_key: &str,
        api_secret: &str,
    ) -> Result<responses::CFResult, Error>;
    /// Fetch raw JSON response from Codeforces servers.
    ///
    /// # Examples
    ///
    /// ```
    /// # use codeforces_api::requests::*;
    /// # use codeforces_api::responses::*;
    /// # let api_key = codeforces_api::TEST_API_KEY;
    /// # let api_secret = codeforces_api::TEST_API_SECRET;
    /// let x = CFUserCommand::Status {
    ///     handle: "thud".to_string(),
    ///     from: Some(1),
    ///     count: Some(3),
    /// };
    ///
    /// match x.get_raw(api_key, api_secret) {
    ///     Ok(s) => {
    ///         assert!(s.starts_with("{\"status\":\"OK\""));
    ///         // your code here
    ///     },
    ///     _ => {
    ///         panic!("raw API request failed");
    ///     }
    /// }
    /// ```
    fn get_raw(&self, api_key: &str, api_secret: &str)
        -> Result<String, Error>;
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
        api_key: &str,
        api_secret: &str,
    ) -> Result<responses::CFResult, Error> {
        send_codeforces_api_req(self, api_key, api_secret)
    }

    fn get_raw(
        &self,
        api_key: &str,
        api_secret: &str,
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
        api_key: &str,
        api_secret: &str,
    ) -> Result<responses::CFResult, Error> {
        send_codeforces_api_req(self, api_key, api_secret)
    }

    fn get_raw(
        &self,
        api_key: &str,
        api_secret: &str,
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
        }
        res
    }

    fn method_name(&self) -> &'static str {
        match self {
            CFProblemsetCommand::Problems { .. } => "problemset.problems",
            CFProblemsetCommand::RecentStatus { .. } => {
                "problemset.recentStatus"
            }
        }
    }

    fn get(
        &self,
        api_key: &str,
        api_secret: &str,
    ) -> Result<responses::CFResult, Error> {
        send_codeforces_api_req(self, api_key, api_secret)
    }

    fn get_raw(
        &self,
        api_key: &str,
        api_secret: &str,
    ) -> Result<String, Error> {
        send_codeforces_api_req_raw(self, api_key, api_secret)
    }
}

impl CFAPIRequestable for CFRecentActionsCommand {
    fn query_params(&self) -> Vec<(&'static str, String)> {
        let mut res = vec![];
        res.push(("maxCount", self.max_count.to_string()));
        res
    }

    fn method_name(&self) -> &'static str {
        "recentActions"
    }

    fn get(
        &self,
        api_key: &str,
        api_secret: &str,
    ) -> Result<responses::CFResult, Error> {
        send_codeforces_api_req(self, api_key, api_secret)
    }

    fn get_raw(
        &self,
        api_key: &str,
        api_secret: &str,
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
        api_key: &str,
        api_secret: &str,
    ) -> Result<responses::CFResult, Error> {
        send_codeforces_api_req(self, api_key, api_secret)
    }

    fn get_raw(
        &self,
        api_key: &str,
        api_secret: &str,
    ) -> Result<String, Error> {
        send_codeforces_api_req_raw(self, api_key, api_secret)
    }
}

/// Extra utility function which webscrapes problem pages to get input testcases
/// to a given problem.
///
/// Used internally to provide
/// [`problem.fetch_testcases()`](responses::CFProblem::fetch_testcases).
pub fn fetch_testcases_for_problem(
    contest_id: &i64,
    problem_index: &str,
) -> Result<Vec<String>, Error> {
    // construct problem url.
    let url = "https://codeforces.com/contest/".to_string()
        + &contest_id.to_string()
        + "/problem/"
        + &problem_index.to_string();
    match get_url(&url) {
        // if fetch was successful, then read response.
        Ok(res) => {
            let document = Document::from_read(res).unwrap();
            // older problems use <br> instead of text \n chars in the
            // testcases. These are replaced by a regex for consistency.
            lazy_static! {
                static ref RE: Regex = Regex::new(r"(<br>|<br/>)").unwrap();
            }
            let testcases: Vec<String> = document
                .find(Descendant(Class("input"), Name("pre")))
                .map(|e| e.inner_html())
                .map(|e| RE.replace_all(&e, "\n").into())
                .collect();
            if testcases.is_empty() {
                Err(Error::Testcases(
                    "No testcase input found for this \
                        problem.",
                ))
            } else {
                Ok(testcases)
            }
        }
        // if fetch unsuccessful, then wrap `reqwest::Error` in custom Error.
        Err(e) => Err(Error::Http(e)),
    }
}

impl responses::CFProblem {
    /// Extra method which allows a user to fetch testcases directly from a
    /// [`CFProblem`](super::responses::CFProblem).
    ///
    /// Returns Vec of Strings where each String is a separate input testcase
    /// for the problem. Currently, the 'expected output' provided by
    /// Codeforces is not returned. However, in future this could be
    /// implemented relatively easily.
    ///
    /// Uses [`fetch_testcases_for_problem`] under the hood.
    pub fn fetch_testcases(&mut self) -> Result<Vec<String>, Error> {
        if self.contest_id.is_none() {
            return Err(Error::Testcases(
                "problem.contest_id field is \
                    required.",
            ));
        }
        if self.index.is_none() {
            return Err(Error::Testcases("problem.index field is required."));
        }
        let testcases = fetch_testcases_for_problem(
            &self.contest_id.unwrap(),
            &self.index.as_ref().unwrap(),
        );
        // if getting testcases was successful, then set self.input_testcases.
        if let Ok(ref v) = testcases {
            self.input_testcases = Some(v.to_vec());
        }
        testcases
    }
}
