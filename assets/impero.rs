// demo/src/api.rs

// Those imports are used by the parser to solve TS imports.
// Known limitation: We can't use wildcard imports for API types
use chrono::{DateTime, Utc};
use impero_common::skipable::Skippable;
use impero_control::api::{ControlProgramOverview, EditActions, EditViewActions};
use impero_database::api::{
    ControlProgramId, ControlResultExternalId, ControlResultId, ControlResultUserPoolId,
    ReviewResultExternalId, ReviewResultId, UserId,
};
use impero_user::api::UserOverview;
use std::collections::{HashMap, HashSet};

#[derive(Debug, TypeName, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ControlResultsOverview {
    pub control_results: Vec<ControlResultOverview>,
    pub resources: ControlResultsOverviewResources,
    pub actions: ControlResultsOverviewActions,
    pub resource_actions: ControlResultsOverviewResourceActions,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ControlResultOverview {
    pub id: ControlResultId,
    pub title: String,
    pub completed: bool,
    pub control_program_id: ControlProgramId,
    pub assignee: ControlResultAssignee,
    pub reviewer: Option<ControlResultReviewer>,
    pub due_date: DateTime<Utc>,
    pub tags: HashMap<String, Vec<String>>,
    pub workflow_status: WorkflowStatus,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ControlResultAssignee {
    pub kind: ControlResultAssigneeKind,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum ControlResultAssigneeKind {
    #[serde(rename_all = "camelCase")]
    User { user_id: UserId },
    #[serde(rename_all = "camelCase")]
    Pool {
        user_pool_id: ControlResultUserPoolId,
    },
}

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ControlResultReviewer {
    pub review_result_id: ReviewResultId,
    pub kind: ControlResultReviewerKind,
    pub completion_due_date: DateTime<Utc>,
    pub final_review_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum ControlResultReviewerKind {
    #[serde(rename_all = "camelCase")]
    User { user_id: UserId },
    #[serde(rename_all = "camelCase")]
    Pool {
        user_pool_id: ControlResultUserPoolId,
    },
}

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ControlResultsOverviewResources {
    pub control_programs: Vec<ControlProgramOverview>,
    pub users: Vec<UserOverview>,
    pub control_result_user_pools: Vec<ControlResultUserPoolOverview>,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ControlResultUserPoolOverview {
    pub id: ControlResultUserPoolId,
    pub name: String,
    pub description: Option<String>,
    pub claimant_user_id: Option<UserId>,
    pub user_ids: Vec<UserId>,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ControlResultsOverviewActions {
    pub edit: HashSet<ControlResultId>,
    pub view: HashSet<ControlResultId>,
    pub collect: HashSet<ControlResultId>,
    pub remind: HashSet<ControlResultId>,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ControlResultsOverviewResourceActions {
    pub control_programs: ControlProgramsActions,
    pub users: UsersActions,
}

#[derive(PartialEq, Eq, Serialize, Debug)]
#[serde(tag = "type")]
pub enum WorkflowStatus {
    #[serde(rename_all = "camelCase")]
    PendingUserCompletion {
        external_id: Protected<ControlResultExternalId>,
    },
    #[serde(rename_all = "camelCase")]
    PendingReview {
        external_id: Protected<ReviewResultExternalId>,
    },
    #[serde(rename_all = "camelCase")]
    Completed,
}

pub type ControlProgramsActions = EditViewActions<ControlProgramId>;

pub type UsersActions = EditActions<UserId>;

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Debug)]
#[serde(untagged)]
pub enum Protected<T> {
    Confidential,
    Visible(T),
}

#[derive(PartialEq, Eq, Clone, Copy, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Foo {
    #[serde(default)]
    foo: Skippable<i32>,
}
