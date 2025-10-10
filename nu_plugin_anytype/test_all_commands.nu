#!/usr/bin/env nu
# Comprehensive integration test script for Anytype Nushell plugin
#
# Prerequisites:
# - Anytype app running locally on localhost:31009
# - Authentication already completed (anytype auth create)
# - Space named 'dev-test' exists
#
# Usage: nu test_all_commands.nu
#
# Note: Custom values (AnytypeValue) don't support cell path access in Nushell,
# so tests focus on command success rather than detailed field validation.
# Field-level validation is done in the integration test suite (tests/plugin_test.rs).

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
    # Auth status returns a regular record, not a custom value
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

run_test "space get - get dev-test by name" {
    anytype space get $TEST_SPACE
}

run_test "space get - verify returns custom value" {
    let space = (anytype space get $TEST_SPACE)
    let type_name = ($space | describe)
    if ($type_name != "AnytypeValue") {
        error make {msg: $"Expected AnytypeValue, got ($type_name)"}
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

run_test "type list - verify returns custom values" {
    let types = (anytype type list --space $TEST_SPACE)
    if ($types | is-empty) {
        error make {msg: "No types to verify"}
    }
    let first_type = ($types | first)
    let type_name = ($first_type | describe)
    if ($type_name != "AnytypeValue") {
        error make {msg: $"Expected AnytypeValue, got ($type_name)"}
    }
    $types
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
    # Objects list can be empty, that's OK
    anytype object list --space $TEST_SPACE
}

run_test "object list - via pipeline" {
    anytype space get $TEST_SPACE | anytype object list
}

run_test "object list - verify structure" {
    let objects = (anytype object list --space $TEST_SPACE)
    if not ($objects | is-empty) {
        let first_obj = ($objects | first)
        let type_name = ($first_obj | describe)
        if ($type_name != "AnytypeValue") {
            error make {msg: $"Expected AnytypeValue, got ($type_name)"}
        }
    }
    {status: "OK"}
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
    anytype search "test" --space $TEST_SPACE
}

run_test "search - with limit" {
    let results = (anytype search "test" --space $TEST_SPACE --limit 5)
    if ($results | length) > 5 {
        error make {msg: "Returned more results than limit"}
    }
    $results
}

run_test "search - with offset" {
    anytype search "test" --space $TEST_SPACE --limit 10 --offset 0
}

run_test "search - sort by created_date" {
    anytype search "test" --space $TEST_SPACE --sort created_date
}

run_test "search - sort by last_modified_date desc" {
    anytype search "test" --space $TEST_SPACE --sort last_modified_date --direction desc
}

run_test "search - sort by name asc" {
    anytype search "test" --space $TEST_SPACE --sort name --direction asc
}

run_test "search - global search (no space)" {
    anytype search "test" --limit 10
}

run_test "search - via pipeline" {
    anytype space get $TEST_SPACE | anytype search "test"
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
    if ($members | is-empty) {
        error make {msg: "No members to verify"}
    }
    let first_member = ($members | first)
    let type_name = ($first_member | describe)
    if ($type_name != "AnytypeValue") {
        error make {msg: $"Expected AnytypeValue, got ($type_name)"}
    }
    $members
}

# ============================================================================
# Template Tests
# ============================================================================
print ""
print "## Template Tests"
print ""

run_test "template list - with space flag" {
    # Templates can be empty
    anytype template list --space $TEST_SPACE
}

run_test "template list - via pipeline" {
    anytype space get $TEST_SPACE | anytype template list
}

# ============================================================================
# Resolve Tests
# ============================================================================
print ""
print "## Resolve Tests"
print ""

run_test "resolve space - by name" {
    anytype resolve space $TEST_SPACE
}

run_test "resolve space - nonexistent" --expect_error {
    anytype resolve space "nonexistent-space-12345"
}

# ============================================================================
# Cache Tests
# ============================================================================
print ""
print "## Cache Tests"
print ""

run_test "cache stats - get statistics" {
    let stats = (anytype cache stats)
    # Cache stats returns a regular record
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
    anytype cache stats
}

# ============================================================================
# Pipeline Integration Tests
# ============================================================================
print ""
print "## Pipeline Integration Tests"
print ""

run_test "pipeline - space to types" {
    anytype space get $TEST_SPACE | anytype type list
}

run_test "pipeline - space to objects" {
    anytype space get $TEST_SPACE | anytype object list
}

run_test "pipeline - space to search" {
    anytype space get $TEST_SPACE | anytype search "test"
}

run_test "pipeline - count results" {
    let count = (anytype search "test" --space $TEST_SPACE | length)
    {result_count: $count}
}

# ============================================================================
# Context Resolution Tests
# ============================================================================
print ""
print "## Context Resolution Tests"
print ""

run_test "context - pipeline provides space" {
    let result = (anytype space get $TEST_SPACE | anytype type list)
    if ($result | is-empty) {
        error make {msg: "No types from pipeline context"}
    }
    $result
}

run_test "context - flag takes priority" {
    # Just verify the flag is accepted
    anytype type list --space $TEST_SPACE
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

run_test "error - unknown sort property" --expect_error {
    anytype search "test" --space $TEST_SPACE --sort invalid_property
}

run_test "error - invalid sort direction" --expect_error {
    anytype search "test" --space $TEST_SPACE --direction invalid
}

# Note: Tests for parse-time errors (missing positional args, invalid flag values)
# cannot be tested within closures as Nushell parses them before execution.
# These are validated by the integration tests in tests/plugin_test.rs instead.

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
    let header = $"\n### ($test.test)\n- **Status**: ($test.status)\n- **Time**: ($test.timestamp)\n"
    $header | save --append $RESULTS_FILE

    if ($test.duration? != null) {
        $"- **Duration**: ($test.duration)\n" | save --append $RESULTS_FILE
    }
    if ($test.message? != null) {
        $"- **Message**: ($test.message)\n" | save --append $RESULTS_FILE
    }
    if ($test.error? != null) {
        $"- **Error**: ```\n($test.error)\n```\n" | save --append $RESULTS_FILE
    }
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
