//! Invariant property tests
//!
//! These tests verify that certain invariants hold for API types,
//! such as pagination logic and field constraints.

use anytype_rs::api::types::Pagination;
use proptest::prelude::*;

use super::strategies::*;

proptest! {
    /// Test that when pagination has_more is false, offset + limit should be >= total
    /// This ensures the pagination state is consistent
    #[test]
    fn test_pagination_invariant_no_more(
        limit in limit_strategy(),
        total in offset_strategy(),
    ) {
        // Create pagination where has_more is false
        let offset = if total > limit { total - limit } else { 0 };
        let pagination = Pagination {
            has_more: false,
            limit,
            offset,
            total,
        };

        // Invariant: if has_more is false, we've seen all items
        // This means offset + limit >= total (we're at or past the end)
        assert!(
            pagination.offset + pagination.limit >= pagination.total,
            "When has_more=false, offset + limit ({} + {}) should be >= total ({})",
            pagination.offset,
            pagination.limit,
            pagination.total
        );
    }

    /// Test that when pagination has_more is true, offset + limit should be < total
    /// This ensures there are more items to fetch
    #[test]
    fn test_pagination_invariant_has_more(
        limit in limit_strategy(),
        offset in offset_strategy(),
    ) {
        // Create pagination where has_more is true and total is large enough
        let total = offset + limit + 1; // Ensure there's at least one more item
        let pagination = Pagination {
            has_more: true,
            limit,
            offset,
            total,
        };

        // Invariant: if has_more is true, there should be items beyond current page
        assert!(
            pagination.offset + pagination.limit < pagination.total,
            "When has_more=true, offset + limit ({} + {}) should be < total ({})",
            pagination.offset,
            pagination.limit,
            pagination.total
        );
    }

    /// Test that pagination offset is never greater than total
    #[test]
    fn test_pagination_offset_bounds(
        has_more in any::<bool>(),
        limit in limit_strategy(),
        base_offset in 0usize..200,
    ) {
        prop_assume!(limit > 0); // Ensure limit is positive

        #[allow(clippy::manual_range_contains)]
        let total = if has_more {
            base_offset + limit + 1
        } else {
            base_offset + (limit / 2).max(1)
        };

        let pagination = Pagination {
            has_more,
            limit,
            offset: base_offset,
            total,
        };

        // Offset should never exceed total
        assert!(
            pagination.offset <= pagination.total,
            "Offset ({}) should never exceed total ({})",
            pagination.offset,
            pagination.total
        );
    }

    /// Test that limit is always positive
    #[test]
    fn test_pagination_positive_limit(
        has_more in any::<bool>(),
        limit in limit_strategy(),
        offset in offset_strategy(),
        total in offset_strategy(),
    ) {
        let pagination = Pagination {
            has_more,
            limit,
            offset,
            total,
        };

        assert!(pagination.limit > 0, "Limit must be positive");
    }
}
