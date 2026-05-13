use serde::{Deserialize, Serialize};

/// Represents a GQueues queue (folder/list).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Queue {
    /// The unique identifier for the queue.
    pub key: String,
    /// The display name of the queue.
    pub name: String,
    /// Whether this is the primary Inbox queue.
    #[serde(default)]
    pub is_inbox: bool,
    /// Timestamp of the last modification.
    pub last_modified: Option<String>,
    /// The category key this queue belongs to.
    pub category: Option<String>,
    /// The name of the category.
    pub category_name: Option<String>,
    /// The name of the team (if a team queue).
    pub team_name: Option<String>,
    /// The visibility scope of the queue (Internal to the client).
    #[serde(skip)]
    pub scope: Option<String>, // "Personal", "Team", or "Shared"
}

/// Represents a task in GQueues.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    /// The unique identifier for the task.
    pub key: String,
    /// The title of the task.
    pub title: String,
    /// Optional notes or description for the task.
    pub notes: Option<String>,
    /// Whether the task is marked as completed.
    #[serde(default)]
    pub completed: bool,
    /// The key of the queue this task belongs to.
    pub queue_key: Option<String>,
    /// The key of the parent task (if this is a subitem).
    pub parent_key: Option<String>,
    /// Optional list of subtasks.
    pub subitems: Option<Vec<Task>>,
    /// List of tags associated with the task.
    pub tags: Option<Vec<String>>,
    /// List of assignments for this task.
    pub assignments: Option<Vec<Assignment>>,
    /// Information about when the task was created.
    pub creation_date: Option<DateInfo>,
    /// Information about the task's due date.
    pub due_date: Option<DueDateInfo>,
    /// Recurrence information.
    #[serde(default)]
    pub repeats: serde_json::Value, // Can be bool or object
    
    // Expanded fields
    /// The section key this task belongs to.
    pub section_key: Option<String>,
    /// List of attachments.
    pub attachments: Option<Vec<Attachment>>,
    /// Whether the task is crossed off.
    #[serde(default)]
    pub crossed: bool,
    /// Number of comments.
    pub num_comments: Option<u32>,
    /// Whether the task has subitems (shorthand check).
    #[serde(default)]
    pub has_subitems: bool,
    /// The position string for ordering.
    pub position: Option<String>,
    /// Access level.
    pub access: Option<String>,
    /// Whether comments can be added.
    #[serde(default)]
    pub add_comments: bool,
    /// Local-only ordering state.
    pub local_order: Option<f64>,
}

/// Represents an assignment of a task to a user.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Assignment {
    /// The unique key for the assignment.
    pub key: Option<String>,
    /// The email address of the assignee.
    pub email: String,
    /// The name of the assignee.
    pub name: String,
    /// Profile URL.
    pub profile: Option<String>,
    /// Comment for the assignment.
    pub comment: Option<String>,
    /// Date completed.
    pub date_completed: Option<String>,
    /// Whether the assignment is completed.
    #[serde(default)]
    pub completed: bool,
}

/// Information about a date.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DateInfo {
    /// The display text of the date.
    pub text: Option<String>,
    /// The raw date string.
    pub raw: String,
}

/// Information about a due date.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DueDateInfo {
    /// The display title of the due date.
    pub title: Option<String>,
    /// The display text.
    pub text: Option<String>,
    /// The raw due date string.
    pub raw_date: Option<String>,
    /// The raw due time string.
    pub raw_time: Option<String>,
    /// Duration if applicable.
    pub duration: Option<u32>,
    /// Reminder settings.
    pub reminder: Option<serde_json::Value>,
    /// Repeats setting (duplicated from Task level if needed by API).
    pub repeats: Option<serde_json::Value>,
}

/// Represents an attachment.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    pub id: String,
    pub name: String,
    pub url: String,
    pub icon: Option<String>,
    pub short_url: Option<String>,
    pub mime_type: Option<String>,
}
