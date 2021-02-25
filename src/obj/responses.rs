//! Contains the structs etc. which are returned by the Codeforces API
//! following a request.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Response code returned by Codeforces API (Ok, Failed).
///
/// This is extracted from JSON API responses (the `status` field).
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CFResponseStatus {
    Ok,
    Failed,
}

impl fmt::Display for CFResponseStatus {
    /// Display type as yaml using `serde_yaml`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error),
        }
    }
}

/// Response type used internally which directly represents network responses
/// sent back from the Codeforces API.
///
/// Note that using the `.get()` function on a type like
/// [`CFUserCommand::Info`](super::requests::CFUserCommand::Info), will return
/// a [`CFResult`] (in a result) and not this type. Internally, [`CFResponse`]
/// is used as a wrapper to handle errors and serialization more easily.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CFResponse {
    pub status: CFResponseStatus,
    pub result: Option<CFResult>,
    pub comment: Option<String>,
}

impl fmt::Display for CFResponse {
    /// Display type as yaml using `serde_yaml`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error),
        }
    }
}

/// Wrapper for all forms of result returned by the Codeforces API.
///
/// # Examples
///
/// You probably want to match on it depending on the kind of request you are
/// making.
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
/// // x.get(..) will return a CFResult type. You should match on it to make
/// // sure that it returned the type you expected.
/// // To check which type a request should return, you can check its docs.
/// // If the type returned is of the form `Ok(<unexpected_type>)`, then an
/// // internal parsing issue has occurred somewhere (this should be rare).
/// if let Ok(CFResult::CFSubmissionVec(v)) = x.get(api_key, api_secret) {
///     // your code here
/// }
/// ```
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum CFResult {
    CFCommentVec(Vec<CFComment>),
    CFBlogEntry(CFBlogEntry),
    CFHackVec(Vec<CFHack>),
    CFContestVec(Vec<CFContest>),
    CFRatingChangeVec(Vec<CFRatingChange>),
    CFContestStandings(CFContestStandings),
    CFSubmissionVec(Vec<CFSubmission>),
    CFProblemset(CFProblemset),
    CFRecentActionVec(Vec<CFRecentAction>),
    CFBlogEntryVec(Vec<CFBlogEntry>),
    CFFriends(Vec<String>),
    CFUserVec(Vec<CFUser>),
}

impl fmt::Display for CFResult {
    /// Display type as yaml using `serde_yaml`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error),
        }
    }
}

/// Struct representing a Codeforces
/// [user](https://codeforces.com/apiHelp/objects#User).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CFUser {
    pub handle: String,
    pub email: Option<String>,
    pub vk_id: Option<String>,
    pub open_id: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub organization: Option<String>,
    pub contribution: i64,
    pub rank: Option<String>,
    pub rating: Option<i64>,
    pub max_rank: Option<String>,
    pub max_rating: Option<i64>,
    pub last_online_time_seconds: i64,
    pub registration_time_seconds: i64,
    pub friend_of_count: i64,
    pub avatar: String,
    pub title_photo: String,
}

impl fmt::Display for CFUser {
    /// Display type as yaml using `serde_yaml`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error),
        }
    }
}

/// Struct representing a Codeforces
/// [blog entry](https://codeforces.com/apiHelp/objects#BlogEntry).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CFBlogEntry {
    pub id: i64,
    pub original_locale: String,
    pub creation_time_seconds: i64,
    pub author_handle: String,
    pub title: String,
    pub content: Option<String>,
    pub locale: String,
    pub modification_time_seconds: i64,
    pub allow_view_history: bool,
    pub tags: Vec<String>,
    pub rating: i64,
}

impl fmt::Display for CFBlogEntry {
    /// Display type as yaml using `serde_yaml`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error),
        }
    }
}

/// Struct representing a Codeforces blog entry
/// [comment](https://codeforces.com/apiHelp/objects#Comment).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CFComment {
    pub id: i64,
    pub creation_time_seconds: i64,
    pub commentator_handle: String,
    pub locale: String,
    pub text: String,
    pub parent_comment_id: Option<i64>,
    pub rating: i64,
}

impl fmt::Display for CFComment {
    /// Display type as yaml using `serde_yaml`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error),
        }
    }
}

/// Struct representing a Codeforces
/// [recent action](https://codeforces.com/apiHelp/objects#RecentAction).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CFRecentAction {
    pub time_seconds: i64,
    pub blog_entry: Option<CFBlogEntry>,
    pub comment: Option<CFComment>,
}

impl fmt::Display for CFRecentAction {
    /// Display type as yaml using `serde_yaml`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error),
        }
    }
}

/// Struct representing a Codeforces
/// [rating change](https://codeforces.com/apiHelp/objects#RatingChange).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CFRatingChange {
    pub contest_id: i64,
    pub contest_name: String,
    pub handle: String,
    pub rank: i64,
    pub rating_update_time_seconds: i64,
    pub old_rating: i64,
    pub new_rating: i64,
}

impl fmt::Display for CFRatingChange {
    /// Display type as yaml using `serde_yaml`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error),
        }
    }
}

/// Contest type returned by Codeforces API (eg. IOI, ICPC).
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum CFContestType {
    #[serde(rename = "CF")]
    Codeforces,
    IOI,
    ICPC,
}

impl fmt::Display for CFContestType {
    /// Display type as yaml using `serde_yaml`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error),
        }
    }
}

/// Contest phase returned by Codeforces API (eg. PendingSystemTest).
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CFContestPhase {
    Before,
    Coding,
    PendingSystemTest,
    SystemTest,
    Finished,
}

impl fmt::Display for CFContestPhase {
    /// Display type as yaml using `serde_yaml`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error),
        }
    }
}

/// Struct representing the object returned by a
/// [`contest.standings`](super::requests::CFContestCommand::Standings)
/// request.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CFContestStandings {
    pub contest: CFContest,
    pub problems: Vec<CFProblem>,
    pub rows: Vec<CFRanklistRow>,
}

impl fmt::Display for CFContestStandings {
    /// Display type as yaml using `serde_yaml`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error),
        }
    }
}

/// Struct representing a Codeforces
/// [contest](https://codeforces.com/apiHelp/objects#Contest).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CFContest {
    pub id: i64,
    pub name: String,
    #[serde(rename = "type")]
    pub contest_type: CFContestType,
    pub phase: CFContestPhase,
    pub duration_seconds: i64,
    pub start_time_seconds: Option<i64>,
    pub relative_time_seconds: Option<i64>,
    pub prepared_by: Option<String>,
    pub website_url: Option<String>,
    pub description: Option<String>,
    pub difficulty: Option<i64>,
    pub kind: Option<String>,
    pub icpc_region: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub season: Option<String>,
}

impl fmt::Display for CFContest {
    /// Display type as yaml using `serde_yaml`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error),
        }
    }
}

/// Participant type returned by Codeforces API (eg. Contestant, Virtual).
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CFParticipantType {
    Contestant,
    Practice,
    Virtual,
    Manager,
    OutOfCompetition,
}

impl fmt::Display for CFParticipantType {
    /// Display type as yaml using `serde_yaml`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error),
        }
    }
}

/// Struct representing a Codeforces
/// [party](https://codeforces.com/apiHelp/objects#Party).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CFParty {
    pub contest_id: Option<i64>,
    pub members: Vec<CFMember>,
    pub participant_type: CFParticipantType,
    pub team_id: Option<i64>,
    pub team_name: Option<String>,
    pub ghost: bool,
    pub room: Option<i64>,
    pub start_time_seconds: Option<i64>,
}

impl fmt::Display for CFParty {
    /// Display type as yaml using `serde_yaml`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error),
        }
    }
}

/// Struct representing a Codeforces
/// [member](https://codeforces.com/apiHelp/objects#Member).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CFMember {
    pub handle: String,
}

impl fmt::Display for CFMember {
    /// Display type as yaml using `serde_yaml`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error),
        }
    }
}

/// Problem type returned by Codeforces API (Programming, Question).
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CFProblemType {
    Programming,
    Question,
}

impl fmt::Display for CFProblemType {
    /// Display type as yaml using `serde_yaml`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error),
        }
    }
}

/// Struct representing a Codeforces
/// [problem](https://codeforces.com/apiHelp/objects#Problem).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CFProblem {
    pub contest_id: Option<i64>,
    pub problemset_name: Option<String>,
    pub index: Option<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub problem_type: CFProblemType,
    pub points: Option<f64>,
    pub rating: Option<i64>,
    pub tags: Vec<String>,
    #[serde(skip_deserializing)]
    pub input_testcases: Option<Vec<String>>,
}

impl fmt::Display for CFProblem {
    /// Display type as yaml using `serde_yaml`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error),
        }
    }
}

/// Struct representing a Codeforces problem
/// [statistics](https://codeforces.com/apiHelp/objects#ProblemStatistics).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CFProblemStatistics {
    pub contest_id: Option<i64>,
    pub index: Option<String>,
    pub solved_count: i64,
}

impl fmt::Display for CFProblemStatistics {
    /// Display type as yaml using `serde_yaml`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error),
        }
    }
}

/// Struct representing the object returned by a
/// [`problemset.problems`](super::requests::CFProblemsetCommand::Problems)
/// request.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CFProblemset {
    pub problems: Vec<CFProblem>,
    pub problem_statistics: Vec<CFProblemStatistics>,
}

impl fmt::Display for CFProblemset {
    /// Display type as yaml using `serde_yaml`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error),
        }
    }
}

/// Submission verdict returned by Codeforces API (eg. Ok, CompilationError).
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CFSubmissionVerdict {
    Failed,
    Ok,
    Partial,
    CompilationError,
    RuntimeError,
    WrongAnswer,
    PresentationError,
    TimeLimitExceeded,
    MemoryLimitExceeded,
    IdlenessLimitExceeded,
    SecurityViolated,
    Crashed,
    InputPreparationCrashed,
    Challenged,
    Skipped,
    Testing,
    Rejected,
}

impl fmt::Display for CFSubmissionVerdict {
    /// Display type as yaml using `serde_yaml`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error),
        }
    }
}

/// Testset returned by Codeforces API (eg. Pretests, TestSet1).
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CFTestset {
    Samples,
    Pretests,
    Tests,
    Challenges,
    #[serde(rename = "TESTS1")]
    TestSet1,
    #[serde(rename = "TESTS2")]
    TestSet2,
    #[serde(rename = "TESTS3")]
    TestSet3,
    #[serde(rename = "TESTS4")]
    TestSet4,
    #[serde(rename = "TESTS5")]
    TestSet5,
    #[serde(rename = "TESTS6")]
    TestSet6,
    #[serde(rename = "TESTS7")]
    TestSet7,
    #[serde(rename = "TESTS8")]
    TestSet8,
    #[serde(rename = "TESTS9")]
    TestSet9,
    #[serde(rename = "TESTS10")]
    TestSet10,
}

impl fmt::Display for CFTestset {
    /// Display type as yaml using `serde_yaml`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error),
        }
    }
}

/// Struct representing a Codeforces
/// [submission](https://codeforces.com/apiHelp/objects#Submission).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CFSubmission {
    pub id: i64,
    pub contest_id: Option<i64>,
    pub creation_time_seconds: i64,
    pub relative_time_seconds: Option<i64>,
    pub problem: CFProblem,
    pub author: CFParty,
    pub programming_language: String,
    pub verdict: Option<CFSubmissionVerdict>,
    pub testset: CFTestset,
    pub passed_test_count: i64,
    pub time_consumed_millis: i64,
    pub memory_consumed_bytes: i64,
    pub points: Option<f64>,
}

impl fmt::Display for CFSubmission {
    /// Display type as yaml using `serde_yaml`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error),
        }
    }
}

/// Hack verdict returned by Codeforces API (eg. HackSuccessful, Testing).
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CFHackVerdict {
    HackSuccessful,
    HackUnsuccessful,
    InvalidInput,
    GeneratorIncompilable,
    GeneratorCrashed,
    Ignored,
    Testing,
    Other,
}

impl fmt::Display for CFHackVerdict {
    /// Display type as yaml using `serde_yaml`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error),
        }
    }
}

/// Struct representing a Codeforces judge protocol for hacks.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CFJudgeProtocol {
    pub manual: String,
    pub protocol: String,
    pub verdict: String,
}

impl fmt::Display for CFJudgeProtocol {
    /// Display type as yaml using `serde_yaml`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error),
        }
    }
}

/// Struct representing a Codeforces
/// [hack](https://codeforces.com/apiHelp/objects#Hack).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CFHack {
    pub id: i64,
    pub creation_time_seconds: i64,
    pub hacker: CFParty,
    pub defender: CFParty,
    pub verdict: Option<CFHackVerdict>,
    pub problem: CFProblem,
    pub test: Option<String>,
    pub judge_protocol: Option<CFJudgeProtocol>,
}

impl fmt::Display for CFHack {
    /// Display type as yaml using `serde_yaml`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error),
        }
    }
}

/// Struct representing a Codeforces
/// [ranklist row](https://codeforces.com/apiHelp/objects#RanklistRow).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CFRanklistRow {
    pub party: CFParty,
    pub rank: i64,
    pub points: f64,
    pub penalty: i64,
    pub successful_hack_count: i64,
    pub unsuccessful_hack_count: i64,
    pub problem_results: Vec<CFProblemResult>,
    pub last_submission_time_seconds: Option<i64>,
}

impl fmt::Display for CFRanklistRow {
    /// Display type as yaml using `serde_yaml`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error),
        }
    }
}

/// Problem result type returned by Codeforces API (Preliminary, Final).
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CFProblemResultType {
    Preliminary,
    Final,
}

impl fmt::Display for CFProblemResultType {
    /// Display type as yaml using `serde_yaml`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error),
        }
    }
}

/// Struct representing a Codeforces
/// [problem result](https://codeforces.com/apiHelp/objects#ProblemResult).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CFProblemResult {
    pub points: f64,
    pub penalty: Option<i64>,
    pub rejected_attempt_count: i64,
    #[serde(rename = "type")]
    pub problem_result_type: CFProblemResultType,
    pub best_submission_time_seconds: Option<i64>,
}

impl fmt::Display for CFProblemResult {
    /// Display type as yaml using `serde_yaml`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error),
        }
    }
}
