//! Snapshot tests for members module types

use anytype_rs::api::{Member, MemberRole, MemberStatus};

#[test]
fn test_member_role_serialization() {
    insta::assert_json_snapshot!("member_role_viewer", MemberRole::Viewer);
    insta::assert_json_snapshot!("member_role_editor", MemberRole::Editor);
    insta::assert_json_snapshot!("member_role_owner", MemberRole::Owner);
    insta::assert_json_snapshot!("member_role_no_permission", MemberRole::NoPermission);
}

#[test]
fn test_member_status_serialization() {
    insta::assert_json_snapshot!("member_status_joining", MemberStatus::Joining);
    insta::assert_json_snapshot!("member_status_active", MemberStatus::Active);
    insta::assert_json_snapshot!("member_status_removed", MemberStatus::Removed);
    insta::assert_json_snapshot!("member_status_declined", MemberStatus::Declined);
    insta::assert_json_snapshot!("member_status_removing", MemberStatus::Removing);
    insta::assert_json_snapshot!("member_status_canceled", MemberStatus::Canceled);
}

#[test]
fn test_member_serialization() {
    let member = Member {
        id: "member123".to_string(),
        name: Some("John Doe".to_string()),
        global_name: Some("john.any".to_string()),
        identity: Some("identity456".to_string()),
        object: Some("member".to_string()),
        role: MemberRole::Editor,
        status: MemberStatus::Active,
        icon: Some(serde_json::json!({"emoji": "👤"})),
    };
    insta::assert_json_snapshot!("member_full", member);

    let member_minimal = Member {
        id: "member789".to_string(),
        name: None,
        global_name: None,
        identity: None,
        object: None,
        role: MemberRole::Viewer,
        status: MemberStatus::Joining,
        icon: None,
    };
    insta::assert_json_snapshot!("member_minimal", member_minimal);
}
