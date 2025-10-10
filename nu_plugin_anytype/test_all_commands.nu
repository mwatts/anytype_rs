#!/usr/bin/env nu
# Comprehensive integration test script for Anytype Nushell plugin
#
# Prerequisites:
# - Anytype app running locally on localhost:31009
# - Authentication already completed (anytype auth create)
# - Space named 'dev-test' exists
#
# Usage: nu test_all_commands.nu

# Configuration
const TEST_SPACE = "dev-test"
const RESULTS_FILE = "test_results.txt"

# Test result tracking
mut test_results = []
mut tests_passed = 0
mut tests_failed = 0
mut tests_skipped = 0

# Helper function to run a test and record results
def run_test [
    name: string          # Test name
    command: closure      # Command to execute
    --skip                # Skip this test
    --expect_error        # Expect the command to fail
] {
    print $"Running: ($name)"

    if $skip {
        $env.tests_skipped = ($env.tests_skipped + 1)
        $env.test_results = ($env.test_results | append {
            test: $name
            status: "SKIPPED"
            message: "Test skipped"
            timestamp: (date now | format date "%Y-%m-%d %H:%M:%S")
        })
        print $"  → SKIPPED"
        return
    }

    let start_time = (date now)

    try {
        let result = (do $command)
        let end_time = (date now)
        let duration = ($end_time - $start_time)

        if $expect_error {
            $env.tests_failed = ($env.tests_failed + 1)
            $env.test_results = ($env.test_results | append {
                test: $name
                status: "FAILED"
                message: "Expected error but command succeeded"
                duration: $duration
                timestamp: (date now | format date "%Y-%m-%d %H:%M:%S")
            })
            print $"  → FAILED (expected error but succeeded)"
        } else {
            $env.tests_passed = ($env.tests_passed + 1)
            $env.test_results = ($env.test_results | append {
                test: $name
                status: "PASSED"
                result: ($result | to json --raw)
                duration: $duration
                timestamp: (date now | format date "%Y-%m-%d %H:%M:%S")
            })
            print $"  → PASSED"
        }
    } catch { |err|
        let end_time = (date now)
        let duration = ($end_time - $start_time)

        if $expect_error {
            $env.tests_passed = ($env.tests_passed + 1)
            $env.test_results = ($env.test_results | append {
                test: $name
                status: "PASSED"
                message: "Expected error occurred"
                error: ($err | to json --raw)
                duration: $duration
                timestamp: (date now | format date "%Y-%m-%d %H:%M:%S")
            })
            print $"  → PASSED (expected error)"
        } else {
            $env.tests_failed = ($env.tests_failed + 1)
            $env.test_results = ($env.test_results | append {
                test: $name
                status: "FAILED"
                error: ($err | to json --raw)
                duration: $duration
                timestamp: (date now | format date "%Y-%m-%d %H:%M:%S")
            })
            print $"  → FAILED"
            print $"     Error: ($err)"
        }
    }
}

# Start test execution
print "============================================================================"
print "Anytype Nushell Plugin - Comprehensive Integration Tests"
print "============================================================================"
print $"Test Space: ($TEST_SPACE)"
print $"Results File: ($RESULTS_FILE)"
print $"Start Time: (date now | format date '%Y-%m-%d %H:%M:%S')"
print ""

# Initialize test counters
$env.test_results = []
$env.tests_passed = 0
$env.tests_failed = 0
$env.tests_skipped = 0

# ============================================================================
# Authentication Tests
# ============================================================================
print "## Authentication Tests"
print ""

run_test "auth status - check authentication" {
    let status = (anytype auth status)
    if ($status.status != "authenticated") {
        error make {msg: "Not authenticated"}
    }
    if ($status.connected != true) {
        error make {msg: "Not connected to Anytype"}
    }
    $status
}

run_test "auth status - verify spaces accessible" {
    let status = (anytype auth status)
    if ($status.spaces_count? == null) {
        error make {msg: "No spaces_count in status"}
    }
    if ($status.spaces_count < 1) {
        error make {msg: "No spaces available"}
    }
    $status
}

# ============================================================================
# Space Tests
# ============================================================================
print ""
print "## Space Tests"
print ""

run_test "space list - get all spaces" {
    let spaces = (anytype space list)
    if ($spaces | is-empty) {
        error make {msg: "No spaces found"}
    }
    $spaces
}

run_test "space list - verify dev-test exists" {
    let spaces = (anytype space list)
    let dev_test = ($spaces | where name == $TEST_SPACE)
    if ($dev_test | is-empty) {
        error make {msg: $"Space '($TEST_SPACE)' not found"}
    }
    $dev_test
}

run_test "space get - get dev-test by name" {
    let space = (anytype space get $TEST_SPACE)
    if ($space.name != $TEST_SPACE) {
        error make {msg: "Wrong space returned"}
    }
    $space
}

run_test "space get - verify has ID" {
    let space = (anytype space get $TEST_SPACE)
    if ($space.id? == null) {
        error make {msg: "Space has no ID"}
    }
    if ($space.id | str length) < 1 {
        error make {msg: "Space ID is empty"}
    }
    $space
}

run_test "space get - nonexistent space" --expect_error {
    anytype space get "nonexistent-space-12345"
}

# ============================================================================
# Type Tests
# ============================================================================
print ""
print "## Type Tests"
print ""

run_test "type list - with space flag" {
    let types = (anytype type list --space $TEST_SPACE)
    if ($types | is-empty) {
        error make {msg: "No types found in space"}
    }
    $types
}

run_test "type list - via pipeline" {
    let types = (anytype space get $TEST_SPACE | anytype type list)
    if ($types | is-empty) {
        error make {msg: "No types found via pipeline"}
    }
    $types
}

run_test "type list - verify has type_key" {
    let types = (anytype type list --space $TEST_SPACE)
    let first_type = ($types | first)
    if ($first_type.key? == null) {
        error make {msg: "Type has no key"}
    }
    $first_type
}

run_test "type get - get first type by name" {
    let types = (anytype type list --space $TEST_SPACE)
    let first_type = ($types | first)
    let type_by_name = (anytype type get $first_type.name --space $TEST_SPACE)
    if ($type_by_name.name != $first_type.name) {
        error make {msg: "Type names don't match"}
    }
    $type_by_name
}

run_test "type get - via pipeline" {
    let types = (anytype type list --space $TEST_SPACE)
    let first_type = ($types | first)
    let type_by_name = (anytype space get $TEST_SPACE | anytype type get $first_type.name)
    if ($type_by_name.name != $first_type.name) {
        error make {msg: "Type names don't match"}
    }
    $type_by_name
}

run_test "type get - nonexistent type" --expect_error {
    anytype type get "nonexistent-type-12345" --space $TEST_SPACE
}

# ============================================================================
# Object Tests
# ============================================================================
print ""
print "## Object Tests"
print ""

run_test "object list - with space flag" {
    let objects = (anytype object list --space $TEST_SPACE)
    # Objects list can be empty, that's OK
    $objects
}

run_test "object list - via pipeline" {
    let objects = (anytype space get $TEST_SPACE | anytype object list)
    $objects
}

run_test "object list - verify structure" {
    let objects = (anytype object list --space $TEST_SPACE)
    if not ($objects | is-empty) {
        let first_obj = ($objects | first)
        if ($first_obj.id? == null) {
            error make {msg: "Object has no ID"}
        }
        if ($first_obj.type_key? == null) {
            error make {msg: "Object has no type_key"}
        }
        $first_obj
    } else {
        {message: "No objects in space (OK)"}
    }
}

run_test "object get - get first object if exists" {
    let objects = (anytype object list --space $TEST_SPACE)
    if not ($objects | is-empty) {
        let first_obj = ($objects | first)
        if ($first_obj.name? != null) and ($first_obj.name | str length) > 0 {
            let obj = (anytype object get $first_obj.name --space $TEST_SPACE)
            if ($obj.id != $first_obj.id) {
                error make {msg: "Object IDs don't match"}
            }
            $obj
        } else {
            {message: "First object has no name (skipped get test)"}
        }
    } else {
        {message: "No objects available"}
    }
}

run_test "object get - nonexistent object" --expect_error {
    anytype object get "nonexistent-object-12345" --space $TEST_SPACE
}

# ============================================================================
# Search Tests
# ============================================================================
print ""
print "## Search Tests"
print ""

run_test "search - basic search in space" {
    let results = (anytype search "test" --space $TEST_SPACE)
    # Results can be empty
    $results
}

run_test "search - with limit" {
    let results = (anytype search "test" --space $TEST_SPACE --limit 5)
    if ($results | length) > 5 {
        error make {msg: "Returned more results than limit"}
    }
    $results
}

run_test "search - with offset" {
    let results = (anytype search "test" --space $TEST_SPACE --limit 10 --offset 0)
    $results
}

run_test "search - sort by created_date" {
    let results = (anytype search "test" --space $TEST_SPACE --sort created_date)
    $results
}

run_test "search - sort by last_modified_date desc" {
    let results = (anytype search "test" --space $TEST_SPACE --sort last_modified_date --direction desc)
    $results
}

run_test "search - sort by name asc" {
    let results = (anytype search "test" --space $TEST_SPACE --sort name --direction asc)
    $results
}

run_test "search - global search (no space)" {
    let results = (anytype search "test" --limit 10)
    # May return results from multiple spaces
    $results
}

run_test "search - via pipeline" {
    let results = (anytype space get $TEST_SPACE | anytype search "test")
    $results
}

# ============================================================================
# Member Tests
# ============================================================================
print ""
print "## Member Tests"
print ""

run_test "member list - with space flag" {
    let members = (anytype member list --space $TEST_SPACE)
    if ($members | is-empty) {
        error make {msg: "No members found (should have at least owner)"}
    }
    $members
}

run_test "member list - via pipeline" {
    let members = (anytype space get $TEST_SPACE | anytype member list)
    if ($members | is-empty) {
        error make {msg: "No members found"}
    }
    $members
}

run_test "member list - verify structure" {
    let members = (anytype member list --space $TEST_SPACE)
    let first_member = ($members | first)
    if ($first_member.id? == null) {
        error make {msg: "Member has no ID"}
    }
    if ($first_member.role? == null) {
        error make {msg: "Member has no role"}
    }
    $first_member
}

# ============================================================================
# Template Tests
# ============================================================================
print ""
print "## Template Tests"
print ""

run_test "template list - with space flag" {
    let templates = (anytype template list --space $TEST_SPACE)
    # Templates can be empty
    $templates
}

run_test "template list - via pipeline" {
    let templates = (anytype space get $TEST_SPACE | anytype template list)
    $templates
}

# ============================================================================
# Resolve Tests
# ============================================================================
print ""
print "## Resolve Tests"
print ""

run_test "resolve space - by name" {
    let resolved = (anytype resolve space $TEST_SPACE)
    if ($resolved.name != $TEST_SPACE) {
        error make {msg: "Resolved wrong space"}
    }
    if ($resolved.id? == null) {
        error make {msg: "No ID in resolved space"}
    }
    $resolved
}

run_test "resolve space - nonexistent" --expect_error {
    anytype resolve space "nonexistent-space-12345"
}

run_test "resolve type - first type in space" {
    let types = (anytype type list --space $TEST_SPACE)
    if not ($types | is-empty) {
        let first_type = ($types | first)
        let resolved = (anytype resolve type $first_type.name --space $TEST_SPACE)
        if ($resolved.name != $first_type.name) {
            error make {msg: "Resolved wrong type"}
        }
        if ($resolved.key? == null) {
            error make {msg: "No key in resolved type"}
        }
        $resolved
    } else {
        {message: "No types available"}
    }
}

run_test "resolve object - first object if exists" {
    let objects = (anytype object list --space $TEST_SPACE)
    if not ($objects | is-empty) {
        let first_obj = ($objects | first)
        if ($first_obj.name? != null) and ($first_obj.name | str length) > 0 {
            let resolved = (anytype resolve object $first_obj.name --space $TEST_SPACE)
            if ($resolved.name != $first_obj.name) {
                error make {msg: "Resolved wrong object"}
            }
            $resolved
        } else {
            {message: "First object has no name"}
        }
    } else {
        {message: "No objects available"}
    }
}

# ============================================================================
# Cache Tests
# ============================================================================
print ""
print "## Cache Tests"
print ""

run_test "cache stats - get statistics" {
    let stats = (anytype cache stats)
    if ($stats.ttl_seconds? == null) {
        error make {msg: "No ttl_seconds in stats"}
    }
    $stats
}

run_test "cache clear - clear all caches" {
    let result = (anytype cache clear)
    if ($result != "Cache cleared") {
        error make {msg: $"Unexpected result: ($result)"}
    }
    $result
}

run_test "cache stats - after clear" {
    let stats = (anytype cache stats)
    $stats
}

# ============================================================================
# Pipeline Integration Tests
# ============================================================================
print ""
print "## Pipeline Integration Tests"
print ""

run_test "pipeline - space -> types -> filter" {
    let result = (
        anytype space get $TEST_SPACE
        | anytype type list
        | where name != null
    )
    $result
}

run_test "pipeline - space -> objects -> select fields" {
    let result = (
        anytype space get $TEST_SPACE
        | anytype object list
        | select id name type_key
    )
    $result
}

run_test "pipeline - space -> search -> filter by type" {
    let result = (
        anytype space get $TEST_SPACE
        | anytype search "test"
        | where type_key != null
    )
    $result
}

run_test "pipeline - search -> count results" {
    let count = (
        anytype search "test" --space $TEST_SPACE
        | length
    )
    {result_count: $count}
}

# ============================================================================
# Context Resolution Tests
# ============================================================================
print ""
print "## Context Resolution Tests"
print ""

run_test "context - flag takes priority over pipeline" {
    # Get a different space (if exists) or use same space
    let spaces = (anytype space list)
    if ($spaces | length) > 1 {
        let other_space = ($spaces | where name != $TEST_SPACE | first)
        let result = (
            anytype space get $TEST_SPACE
            | anytype object list --space $other_space.name
        )
        # Should use other_space from flag, not TEST_SPACE from pipeline
        $result
    } else {
        {message: "Only one space available, cannot test priority"}
    }
}

run_test "context - pipeline provides space" {
    let result = (
        anytype space get $TEST_SPACE
        | anytype type list
    )
    if ($result | is-empty) {
        error make {msg: "No types from pipeline context"}
    }
    $result
}

# ============================================================================
# Error Handling Tests
# ============================================================================
print ""
print "## Error Handling Tests"
print ""

run_test "error - invalid space name" --expect_error {
    anytype object list --space "invalid-space-name-12345"
}

run_test "error - missing required argument" --expect_error {
    anytype space get
}

run_test "error - invalid flag value" --expect_error {
    anytype search "test" --limit "not-a-number"
}

run_test "error - unknown sort property" --expect_error {
    anytype search "test" --sort invalid_property
}

run_test "error - invalid sort direction" --expect_error {
    anytype search "test" --direction invalid
}

# ============================================================================
# Generate Test Report
# ============================================================================
print ""
print "============================================================================"
print "Test Execution Complete"
print "============================================================================"
print ""

let end_time = (date now)
let total_tests = ($env.tests_passed + $env.tests_failed + $env.tests_skipped)

print $"Total Tests:  ($total_tests)"
print $"Passed:       ($env.tests_passed) ✓"
print $"Failed:       ($env.tests_failed) ✗"
print $"Skipped:      ($env.tests_skipped) ○"
print $"Success Rate: (($env.tests_passed * 100 / $total_tests)  | math round)%"
print ""
print $"End Time: ($end_time | format date '%Y-%m-%d %H:%M:%S')"
print ""

# Write detailed results to file
print $"Writing detailed results to ($RESULTS_FILE)..."

let report_header = $"
Anytype Nushell Plugin - Test Results
================================================================================
Generated: (date now | format date '%Y-%m-%d %H:%M:%S')
Test Space: ($TEST_SPACE)
Total Tests: ($total_tests)
Passed: ($env.tests_passed)
Failed: ($env.tests_failed)
Skipped: ($env.tests_skipped)
Success Rate: (($env.tests_passed * 100 / $total_tests) | math round)%
================================================================================

"

# Write header
$report_header | save --force $RESULTS_FILE

# Write summary table
"## Test Summary\n" | save --append $RESULTS_FILE
($env.test_results
    | select test status timestamp
    | to md
) | save --append $RESULTS_FILE

# Write detailed results
"\n## Detailed Results\n" | save --append $RESULTS_FILE
for test in $env.test_results {
    let detail = $"
### ($test.test)
- **Status**: ($test.status)
- **Time**: ($test.timestamp)
(if ($test.duration? != null) { $"- **Duration**: ($test.duration)\n" } else { "" })
(if ($test.message? != null) { $"- **Message**: ($test.message)\n" } else { "" })
(if ($test.error? != null) { $"- **Error**: ```\n($test.error)\n```\n" } else { "" })
"
    $detail | save --append $RESULTS_FILE
}

print $"Results saved to ($RESULTS_FILE)"
print ""

# Exit with appropriate code
if $env.tests_failed > 0 {
    print "⚠️  Some tests failed!"
    exit 1
} else {
    print "✅ All tests passed!"
    exit 0
}
