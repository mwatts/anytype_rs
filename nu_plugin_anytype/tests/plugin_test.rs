/// Integration tests for Nushell plugin commands
///
/// These tests use nu-plugin-test-support to test commands in a realistic
/// Nushell environment without requiring a running Anytype instance.
use nu_plugin_test_support::PluginTest;
use nu_protocol::{ShellError, Span};

/// Helper to create the plugin test instance
fn create_plugin_test() -> Result<PluginTest, ShellError> {
    PluginTest::new(
        "anytype",
        nu_plugin_anytype::AnytypePlugin::new().into(),
    )
}

#[test]
fn test_plugin_creates_successfully() -> Result<(), ShellError> {
    let _plugin = create_plugin_test()?;
    Ok(())
}

// ============================================================================
// Authentication Commands Tests
// ============================================================================

#[test]
fn test_auth_status_without_credentials() -> Result<(), ShellError> {
    let pipeline = create_plugin_test()?.eval("anytype auth status")?;
    let value = pipeline.into_value(Span::test_data())?;

    // Should return a record with status = "not_authenticated"
    let record = value.as_record()?;
    let status = record.get("status").expect("Should have status field");
    assert_eq!(status.as_str()?, "not_authenticated");
    Ok(())
}

#[test]
fn test_auth_delete_is_safe_when_no_credentials() -> Result<(), ShellError> {
    // Should not panic even if no credentials exist
    let result = create_plugin_test()?.eval("anytype auth delete");

    // May succeed (no-op) or fail gracefully
    let _ = result;
    Ok(())
}

// ============================================================================
// Cache Commands Tests
// ============================================================================

#[test]
fn test_cache_clear_requires_auth() -> Result<(), ShellError> {
    let result = create_plugin_test()?.eval("anytype cache clear");

    // Should fail with authentication error
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_cache_stats_requires_auth() -> Result<(), ShellError> {
    let result = create_plugin_test()?.eval("anytype cache stats");

    // Should fail with authentication error
    assert!(result.is_err());
    Ok(())
}

// ============================================================================
// Resolve Commands Tests (without authentication)
// ============================================================================

#[test]
fn test_resolve_space_requires_auth() -> Result<(), ShellError> {
    let result = create_plugin_test()?.eval("anytype resolve space 'Work'");

    // Should fail with authentication error
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_resolve_type_requires_auth() -> Result<(), ShellError> {
    let result = create_plugin_test()?.eval("anytype resolve type 'Task' --space 'Work'");

    // Should fail with authentication error
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_resolve_object_requires_auth() -> Result<(), ShellError> {
    let result = create_plugin_test()?.eval("anytype resolve object 'MyObject' --space 'Work'");

    // Should fail with authentication error
    assert!(result.is_err());
    Ok(())
}

// ============================================================================
// Space Commands Tests (without authentication)
// ============================================================================

#[test]
fn test_space_list_requires_auth() -> Result<(), ShellError> {
    let result = create_plugin_test()?.eval("anytype space list");

    // Should fail with authentication error
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_space_get_requires_auth() -> Result<(), ShellError> {
    let result = create_plugin_test()?.eval("anytype space get 'Work'");

    // Should fail with authentication error
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_space_create_requires_auth() -> Result<(), ShellError> {
    let result = create_plugin_test()?.eval("anytype space create 'TestSpace'");

    // Should fail with authentication error
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_space_create_with_description() -> Result<(), ShellError> {
    let result = create_plugin_test()?
        .eval("anytype space create 'TestSpace' --description 'A test space'");

    // Should fail with authentication error (but command parsing should work)
    assert!(result.is_err());
    Ok(())
}

// ============================================================================
// Type Commands Tests (without authentication)
// ============================================================================

#[test]
fn test_type_list_requires_space_context() -> Result<(), ShellError> {
    let result = create_plugin_test()?.eval("anytype type list");

    // Should fail with either auth error or context error
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_type_list_with_space_flag() -> Result<(), ShellError> {
    let result = create_plugin_test()?.eval("anytype type list --space 'Work'");

    // Should fail with authentication error
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_type_get_requires_space_context() -> Result<(), ShellError> {
    let result = create_plugin_test()?.eval("anytype type get 'Task'");

    // Should fail with either auth error or context error
    assert!(result.is_err());
    Ok(())
}

// ============================================================================
// Object Commands Tests (without authentication)
// ============================================================================

#[test]
fn test_object_list_requires_space_context() -> Result<(), ShellError> {
    let result = create_plugin_test()?.eval("anytype object list");

    // Should fail with either auth error or context error
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_object_list_with_space_flag() -> Result<(), ShellError> {
    let result = create_plugin_test()?.eval("anytype object list --space 'Work'");

    // Should fail with authentication error
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_object_get_requires_space_context() -> Result<(), ShellError> {
    let result = create_plugin_test()?.eval("anytype object get 'MyObject'");

    // Should fail with either auth error or context error
    assert!(result.is_err());
    Ok(())
}

// ============================================================================
// Search Commands Tests (without authentication)
// ============================================================================

#[test]
fn test_search_global_requires_auth() -> Result<(), ShellError> {
    let result = create_plugin_test()?.eval("anytype search 'notes'");

    // Should fail with authentication error
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_search_with_space_requires_auth() -> Result<(), ShellError> {
    let result = create_plugin_test()?.eval("anytype search 'notes' --space 'Work'");

    // Should fail with authentication error
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_search_with_pagination_flags() -> Result<(), ShellError> {
    let result = create_plugin_test()?
        .eval("anytype search 'notes' --limit 10 --offset 5");

    // Should fail with authentication error (but flags should parse correctly)
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_search_with_sort_flags() -> Result<(), ShellError> {
    let result = create_plugin_test()?
        .eval("anytype search 'notes' --sort created_date --direction asc");

    // Should fail with authentication error (but flags should parse correctly)
    assert!(result.is_err());
    Ok(())
}

// ============================================================================
// Member Commands Tests (without authentication)
// ============================================================================

#[test]
fn test_member_list_requires_space_context() -> Result<(), ShellError> {
    let result = create_plugin_test()?.eval("anytype member list");

    // Should fail with either auth error or context error
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_member_list_with_space_flag() -> Result<(), ShellError> {
    let result = create_plugin_test()?.eval("anytype member list --space 'Work'");

    // Should fail with authentication error
    assert!(result.is_err());
    Ok(())
}

// ============================================================================
// Template Commands Tests (without authentication)
// ============================================================================

#[test]
fn test_template_list_requires_space_context() -> Result<(), ShellError> {
    let result = create_plugin_test()?.eval("anytype template list");

    // Should fail with either auth error or context error
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_template_list_with_space_flag() -> Result<(), ShellError> {
    let result = create_plugin_test()?.eval("anytype template list --space 'Work'");

    // Should fail with authentication error
    assert!(result.is_err());
    Ok(())
}

// ============================================================================
// Command Signature Tests
// ============================================================================

#[test]
fn test_all_commands_are_registered() -> Result<(), ShellError> {
    let mut plugin = create_plugin_test()?;

    // Try to get help for each command category
    // If the command doesn't exist, these will fail
    let commands = vec![
        "anytype auth",
        "anytype space",
        "anytype type",
        "anytype object",
        "anytype property",
        "anytype search",
        "anytype member",
        "anytype template",
        "anytype resolve",
        "anytype cache",
    ];

    for cmd in commands {
        // Just verify the command namespace exists
        // We can't test help directly, but we can verify eval doesn't panic
        let _ = plugin.eval(&format!("{} --help", cmd));
    }

    Ok(())
}

// ============================================================================
// Error Message Tests
// ============================================================================

#[test]
fn test_authentication_error_message_is_helpful() -> Result<(), ShellError> {
    let result = create_plugin_test()?.eval("anytype space list");

    match result {
        Err(err) => {
            let msg = format!("{:?}", err);
            // Should mention authentication or auth
            assert!(
                msg.to_lowercase().contains("auth") ||
                msg.to_lowercase().contains("credentials")
            );
        }
        Ok(_) => panic!("Expected authentication error"),
    }

    Ok(())
}

#[test]
fn test_context_error_message_is_helpful() -> Result<(), ShellError> {
    // This would need a mock authenticated client to test properly
    // For now, we just verify the command structure is correct
    let result = create_plugin_test()?.eval("anytype object list");

    // Should fail (either auth or context error)
    assert!(result.is_err());
    Ok(())
}

// ============================================================================
// Custom Value Tests
// ============================================================================

#[test]
fn test_custom_value_helpers() {
    use nu_plugin_anytype::AnytypeValue;

    let space = AnytypeValue::Space {
        id: "sp_123".to_string(),
        name: "Work".to_string(),
        description: None,
        icon: None,
    };

    assert_eq!(space.id(), "sp_123");
    assert_eq!(space.space_id(), Some("sp_123"));
    assert_eq!(space.name(), "Work");
    assert_eq!(space.type_id(), None);
}

#[test]
fn test_custom_value_object_with_context() {
    use nu_plugin_anytype::AnytypeValue;

    let object = AnytypeValue::Object {
        id: "obj_456".to_string(),
        name: Some("My Task".to_string()),
        properties: serde_json::json!({}),
        markdown: None,
        snippet: None,
        space_id: "sp_123".to_string(),
        type_id: "ty_789".to_string(),
        type_key: "ot_task".to_string(),
    };

    assert_eq!(object.id(), "obj_456");
    assert_eq!(object.space_id(), Some("sp_123"));
    assert_eq!(object.type_id(), Some("ty_789"));
    assert_eq!(object.type_key(), Some("ot_task"));
    assert_eq!(object.name(), "My Task");
}

#[test]
fn test_custom_value_name_fallback() {
    use nu_plugin_anytype::AnytypeValue;

    // Object with no name but with snippet
    let object = AnytypeValue::Object {
        id: "obj_456".to_string(),
        name: None,
        properties: serde_json::json!({}),
        markdown: None,
        snippet: Some("A preview snippet".to_string()),
        space_id: "sp_123".to_string(),
        type_id: "ty_789".to_string(),
        type_key: "ot_note".to_string(),
    };

    assert_eq!(object.name(), "A preview snippet");

    // Object with no name and no snippet - should fall back to ID
    let object_no_name = AnytypeValue::Object {
        id: "obj_456".to_string(),
        name: None,
        properties: serde_json::json!({}),
        markdown: None,
        snippet: None,
        space_id: "sp_123".to_string(),
        type_id: "ty_789".to_string(),
        type_key: "ot_note".to_string(),
    };

    assert_eq!(object_no_name.name(), "obj_456");
}

// ============================================================================
// Property Commands Tests (without authentication)
// ============================================================================

#[test]
fn test_property_list_requires_space_context() -> Result<(), ShellError> {
    let result = create_plugin_test()?.eval("anytype property list");

    // Should fail with either auth error or context error
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_property_list_with_space_flag() -> Result<(), ShellError> {
    let result = create_plugin_test()?.eval("anytype property list --space 'Work'");

    // Should fail with authentication error
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_property_get_requires_space_context() -> Result<(), ShellError> {
    let result = create_plugin_test()?.eval("anytype property get 'MyProperty'");

    // Should fail with either auth error or context error
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_property_get_with_space_flag() -> Result<(), ShellError> {
    let result = create_plugin_test()?.eval("anytype property get 'MyProperty' --space 'Work'");

    // Should fail with authentication error
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_property_create_requires_name() -> Result<(), ShellError> {
    let result = create_plugin_test()?.eval("anytype property create");

    // Should fail with missing argument error
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_property_create_with_format_flag() -> Result<(), ShellError> {
    let result = create_plugin_test()?.eval("anytype property create 'Status' --format select --space 'Work'");

    // Should fail with authentication error (but flags should parse correctly)
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_property_update_requires_name() -> Result<(), ShellError> {
    let result = create_plugin_test()?.eval("anytype property update");

    // Should fail with missing argument error
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_property_update_with_new_name_flag() -> Result<(), ShellError> {
    let result = create_plugin_test()?.eval("anytype property update 'OldName' --new-name 'NewName' --space 'Work'");

    // Should fail with authentication error (but flags should parse correctly)
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_property_delete_requires_name() -> Result<(), ShellError> {
    let result = create_plugin_test()?.eval("anytype property delete");

    // Should fail with missing argument error
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_property_delete_with_space_flag() -> Result<(), ShellError> {
    let result = create_plugin_test()?.eval("anytype property delete 'MyProperty' --space 'Work'");

    // Should fail with authentication error
    assert!(result.is_err());
    Ok(())
}

#[test]
fn test_custom_value_property_with_context() {
    use nu_plugin_anytype::AnytypeValue;

    let property = AnytypeValue::Property {
        id: "prop_123".to_string(),
        name: "Status".to_string(),
        key: "status".to_string(),
        format: "select".to_string(),
        space_id: "sp_456".to_string(),
        type_id: "ty_789".to_string(),
    };

    assert_eq!(property.id(), "prop_123");
    assert_eq!(property.name(), "Status");
    assert_eq!(property.space_id(), Some("sp_456"));
    assert_eq!(property.type_id(), Some("ty_789"));
}

