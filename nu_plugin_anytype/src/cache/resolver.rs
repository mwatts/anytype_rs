use super::ResolveCache;
use anytype_rs::{AnytypeClient, AnytypeError, Result};
use std::sync::Arc;

/// Resolver that wraps API client and caching layer
pub struct Resolver {
    client: Arc<AnytypeClient>,
    cache: ResolveCache,
}

impl Resolver {
    pub fn new(client: Arc<AnytypeClient>, cache_ttl: u64) -> Self {
        Self {
            client,
            cache: ResolveCache::new(cache_ttl),
        }
    }

    /// Resolve space name to ID
    pub async fn resolve_space(&self, name: &str) -> Result<String> {
        // Check cache first
        if let Some(id) = self.cache.get_space(name) {
            return Ok(id);
        }

        // Cache miss - fetch from API
        let spaces = self.client.list_spaces().await?;

        // TODO: Implement case-insensitive matching based on config
        let space = spaces
            .iter()
            .find(|s| s.name == name)
            .ok_or_else(|| AnytypeError::Api {
                message: format!("No Space found with name '{}'", name),
            })?;

        // Cache the result
        self.cache
            .insert_space(name.to_string(), space.id.clone());

        Ok(space.id.clone())
    }

    /// Resolve type name to ID within a space
    pub async fn resolve_type(&self, space_id: &str, name: &str) -> Result<String> {
        // Check cache first
        if let Some(id) = self.cache.get_type(space_id, name) {
            return Ok(id);
        }

        // Cache miss - fetch from API
        let types = self.client.list_types(space_id).await?;

        let type_data = types
            .iter()
            .find(|t| t.name == name)
            .ok_or_else(|| AnytypeError::Api {
                message: format!("No Type found with name '{}' in space '{}'", name, space_id),
            })?;

        // Cache the result
        self.cache
            .insert_type(space_id.to_string(), name.to_string(), type_data.id.clone());

        Ok(type_data.id.clone())
    }

    /// Resolve type_key (global) to type_id (space-specific)
    pub async fn resolve_type_by_key(&self, space_id: &str, type_key: &str) -> Result<String> {
        // For now, do a simple lookup by key
        // In the future, this could use a separate cache
        let types = self.client.list_types(space_id).await?;

        let type_data = types
            .iter()
            .find(|t| t.key == type_key)
            .ok_or_else(|| AnytypeError::Api {
                message: format!("No Type found with key '{}' in space '{}'", type_key, space_id),
            })?;

        Ok(type_data.id.clone())
    }

    /// Resolve object name to ID within a space
    pub async fn resolve_object(&self, space_id: &str, name: &str) -> Result<String> {
        // Check cache first
        if let Some(id) = self.cache.get_object(space_id, name) {
            return Ok(id);
        }

        // Cache miss - fetch from API
        let objects = self.client.list_objects(space_id).await?;

        // Find first object matching the name
        // TODO: Handle name conflicts with warnings
        let object = objects
            .iter()
            .find(|o| o.name.as_deref() == Some(name))
            .ok_or_else(|| AnytypeError::Api {
                message: format!("No Object found with name '{}' in space '{}'", name, space_id),
            })?;

        // Cache the result
        self.cache
            .insert_object(space_id.to_string(), name.to_string(), object.id.clone());

        Ok(object.id.clone())
    }

    /// Resolve list name to ID within a space
    pub async fn resolve_list(&self, space_id: &str, name: &str) -> Result<String> {
        // Check cache first
        if let Some(id) = self.cache.get_list(space_id, name) {
            return Ok(id);
        }

        // Cache miss - fetch from API
        // TODO: Implement list_lists API call when available
        // For now, return an error
        Err(AnytypeError::Api {
            message: format!("List resolution not yet implemented. Cannot find '{}'", name),
        })
    }

    /// Resolve property name to ID within a type
    pub async fn resolve_property(&self, type_id: &str, name: &str) -> Result<String> {
        // Check cache first
        if let Some(id) = self.cache.get_property(type_id, name) {
            return Ok(id);
        }

        // Cache miss - fetch from API
        // TODO: Implement list_properties API call when available
        // For now, return an error
        Err(AnytypeError::Api {
            message: format!("Property resolution not yet implemented. Cannot find '{}'", name),
        })
    }

    /// Resolve tag name to ID within a property
    pub async fn resolve_tag(&self, space_id: &str, property_id: &str, name: &str) -> Result<String> {
        // Check cache first
        if let Some(id) = self.cache.get_tag(property_id, name) {
            return Ok(id);
        }

        // Cache miss - fetch from API
        let tags = self.client.list_tags(space_id, property_id).await?;

        let tag = tags
            .iter()
            .find(|t| t.name == name)
            .ok_or_else(|| AnytypeError::Api {
                message: format!("No Tag found with name '{}' in property '{}'", name, property_id),
            })?;

        // Cache the result
        self.cache
            .insert_tag(property_id.to_string(), name.to_string(), tag.id.clone());

        Ok(tag.id.clone())
    }

    /// Invalidate tag cache
    pub fn invalidate_tag(&self, property_id: &str, name: &str) {
        self.cache.invalidate_tag(property_id, name);
    }

    /// Invalidate property cache (with cascade)
    pub fn invalidate_property(&self, type_id: &str, property_id: &str) {
        self.cache.invalidate_property(type_id, property_id);
    }

    /// Clear all caches
    pub fn clear_cache(&self) {
        self.cache.clear_all();
    }

    /// Invalidate space cache (with cascade)
    pub fn invalidate_space(&self, space_id: &str) {
        self.cache.invalidate_space(space_id);
    }

    /// Invalidate type cache (with cascade)
    pub fn invalidate_type(&self, space_id: &str, type_id: &str) {
        self.cache.invalidate_type(space_id, type_id);
    }

    /// Invalidate object cache
    pub fn invalidate_object(&self, space_id: &str, name: &str) {
        self.cache.invalidate_object(space_id, name);
    }

    /// Get underlying client
    pub fn client(&self) -> &Arc<AnytypeClient> {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    // TODO: Add tests with mock client
}
