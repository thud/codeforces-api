use std::fmt;
use serde::{Deserialize, Serialize};
use serde_yaml;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CFResponseStatus {
    Ok,
    Failed,
}

impl fmt::Display for CFResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error)
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CFResponse {
    pub status: CFResponseStatus,
    pub result: Option<CFResult>,
    pub comment: Option<String>,
}

impl fmt::Display for CFResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error)
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum CFResult {
    CFCommentVec(Vec<CFComment>),
    CFBlogEntry(CFBlogEntry),
    CFHackVec(Vec<CFHack>),
    CFContestVec(Vec<CFContest>),
    CFContestRatingChangeVec(Vec<CFRatingChange>),
    CFContestStandings(CFContestStandings),
    CFSubmissionVec(Vec<CFSubmission>),
    CFProblemset(CFProblemset),
    CFRecentActionVec(Vec<CFRecentAction>),
    CFBlogEntryVec(Vec<CFBlogEntry>),
    CFFriends(Vec<String>),
    CFUserVec(Vec<CFUser>),
    CFRatingChangeVec(Vec<CFRatingChange>),
}

impl fmt::Display for CFResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error)
        }
    }
}

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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error)
        }
    }
}

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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error)
        }
    }
}

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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error)
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CFRecentAction {
    pub time_seconds: i64,
    pub blog_entry: Option<CFBlogEntry>,
    pub comment: Option<CFComment>,
}

impl fmt::Display for CFRecentAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error)
        }
    }
}

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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error)
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum CFContestType {
    #[serde(rename = "CF")]
    Codeforces,
    IOI,
    ICPC,
}

impl fmt::Display for CFContestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error)
        }
    }
}

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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error)
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CFContestStandings {
    pub contest: CFContest,
    pub problems: Vec<CFProblem>,
    pub rows: Vec<CFRanklistRow>,
}

impl fmt::Display for CFContestStandings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error)
        }
    }
}

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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error)
        }
    }
}

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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error)
        }
    }
}

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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error)
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CFMember {
    pub handle: String,
}

impl fmt::Display for CFMember {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error)
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CFProblemType {
    Programming,
    Question,
}

impl fmt::Display for CFProblemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error)
        }
    }
}

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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error)
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CFProblemStatistics {
    pub contest_id: Option<i64>,
    pub index: Option<String>,
    pub solved_count: i64,
}

impl fmt::Display for CFProblemStatistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error)
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CFProblemset {
    pub problems: Vec<CFProblem>,
    pub problem_statistics: Vec<CFProblemStatistics>,
}

impl fmt::Display for CFProblemset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error)
        }
    }
}

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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error)
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CFTestset {
    Samples,
    Pretests,
    Tests,
    Challenges,
    #[serde(rename = "TESTS1")]
    TestS1,
    #[serde(rename = "TESTS2")]
    TestS2,
    #[serde(rename = "TESTS3")]
    TestS3,
    #[serde(rename = "TESTS4")]
    TestS4,
    #[serde(rename = "TESTS5")]
    TestS5,
    #[serde(rename = "TESTS6")]
    TestS6,
    #[serde(rename = "TESTS7")]
    TestS7,
    #[serde(rename = "TESTS8")]
    TestS8,
    #[serde(rename = "TESTS9")]
    TestS9,
    #[serde(rename = "TESTS10")]
    TestS10,
}

impl fmt::Display for CFTestset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error)
        }
    }
}

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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error)
        }
    }
}

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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error)
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CFJudgeProtocol {
    pub manual: String,
    pub protocol: String,
    pub verdict: String,
}

impl fmt::Display for CFJudgeProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error)
        }
    }
}

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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error)
        }
    }
}

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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error)
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CFProblemResultType {
    Preliminary,
    Final,
}

impl fmt::Display for CFProblemResultType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error)
        }
    }
}

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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => Err(fmt::Error)
        }
    }
}

