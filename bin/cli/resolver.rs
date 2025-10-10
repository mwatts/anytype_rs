use super::cache::ResolveCache;
use anyhow::{Result, anyhow};
use anytype_rs::AnytypeClient;

/// Resolver that wraps API client and caching layer
pub struct Resolver<'a> {
    client: &'a AnytypeClient,
    cache: ResolveCache,
}

impl<'a> Resolver<'a> {
    pub fn new(client: &'a AnytypeClient, cache_ttl: u64) -> Self {
        Self {
            client,
            cache: ResolveCache::new(cache_ttl),
        }
    }

    /// Resolve space name to ID, with auto-detection for UUID format
    pub async fn resolve_space(&self, name_or_id: &str) -> Result<String> {
        // If it looks like a UUID, use it directly as ID
        if is_uuid_like(name_or_id) {
            return Ok(name_or_id.to_string());
        }

        // Check cache first
        if let Some(id) = self.cache.get_space(name_or_id) {
            return Ok(id);
        }

        // Cache miss - fetch from API
        let spaces = self.client.list_spaces().await?;

        // Find space by name
        let space = spaces
            .iter()
            .find(|s| s.name == name_or_id)
            .ok_or_else(|| anyhow!("No space found with name '{}'", name_or_id))?;

        // Cache the result
        self.cache
            .insert_space(name_or_id.to_string(), space.id.clone());

        Ok(space.id.clone())
    }

    /// Resolve type name to ID within a space, with auto-detection for UUID format
    pub async fn resolve_type(&self, space_id: &str, name_or_id: &str) -> Result<String> {
        // If it looks like a UUID, use it directly as ID
        if is_uuid_like(name_or_id) {
            return Ok(name_or_id.to_string());
        }

        // Check cache first
        if let Some(id) = self.cache.get_type(space_id, name_or_id) {
            return Ok(id);
        }

        // Cache miss - fetch from API
        let types = self.client.list_types(space_id).await?;

        // Find type by name
        let type_obj = types.iter().find(|t| t.name == name_or_id).ok_or_else(|| {
            anyhow!(
                "No type found with name '{}' in space '{}'",
                name_or_id,
                space_id
            )
        })?;

        // Cache the result
        self.cache.insert_type(
            space_id.to_string(),
            name_or_id.to_string(),
            type_obj.id.clone(),
        );

        Ok(type_obj.id.clone())
    }
}

/// Check if a string looks like a UUID (basic heuristic)
fn is_uuid_like(s: &str) -> bool {
    // UUID format: 8-4-4-4-12 hex characters
    // Example: 550e8400-e29b-41d4-a716-446655440000
    if s.len() != 36 {
        return false;
    }

    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 5 {
        return false;
    }

    // Check length of each part: 8-4-4-4-12
    parts[0].len() == 8
        && parts[1].len() == 4
        && parts[2].len() == 4
        && parts[3].len() == 4
        && parts[4].len() == 12
        && parts
            .iter()
            .all(|p| p.chars().all(|c| c.is_ascii_hexdigit()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid_detection() {
        assert!(is_uuid_like("550e8400-e29b-41d4-a716-446655440000"));
        assert!(is_uuid_like("00000000-0000-0000-0000-000000000000"));
        assert!(!is_uuid_like("not-a-uuid"));
        assert!(!is_uuid_like("my-space"));
        assert!(!is_uuid_like(""));
        assert!(!is_uuid_like(
            "550e8400-e29b-41d4-a716-44665544000" // Too short
        ));
        assert!(!is_uuid_like(
            "550e8400-e29b-41d4-a716-4466554400000" // Too long
        ));
    }
}
