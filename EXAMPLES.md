# API Examples

This document provides examples of how to use the anytype-core library to interact with your local Anytype application.

## Basic Client Setup

```rust
use anytype_core::{AnytypeClient, ClientConfig, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Create client with default configuration (localhost:31009)
    let client = AnytypeClient::new()?;
    
    // Or create with custom configuration
    let config = ClientConfig {
        base_url: "http://localhost:31009".to_string(),
        timeout_seconds: 60,
        app_name: "my-custom-app".to_string(),
    };
    let client = AnytypeClient::with_config(config)?;
    
    Ok(())
}
```

## Authentication Flow

```rust
use anytype_core::{AnytypeClient, Result};

async fn authenticate() -> Result<String> {
    let client = AnytypeClient::new()?;
    
    // Step 1: Create challenge with your local Anytype app
    let challenge = client.create_challenge().await?;
    println!("Challenge ID: {}", challenge.challenge_id);
    
    // Step 2: User receives 4-digit code via local Anytype app notification
    // In a real application, you'd prompt the user for this
    let code = "1234"; // Replace with actual code from user
    
    // Step 3: Create API key
    let api_key_response = client.create_api_key(challenge.challenge_id, code.to_string()).await?;
    
    Ok(api_key_response.api_key)
}
```

## Working with Spaces

```rust
use anytype_core::{AnytypeClient, Result};

async fn work_with_spaces() -> Result<()> {
    let mut client = AnytypeClient::new()?;
    
    // Set API key (from authentication flow)
    client.set_api_key("your-jwt-token".to_string());
    
    // List all spaces
    let spaces = client.list_spaces().await?;
    println!("Found {} spaces", spaces.len());
    
    if let Some(space) = spaces.first() {
        // Get space details
        let space_details = client.get_space(&space.id).await?;
        println!("Space: {} - {}", space_details.id, space_details.name);
        
        // List objects in space
        let objects = client.list_objects(&space.id).await?;
        println!("Found {} objects in space", objects.len());
    }
    
    Ok(())
}
```

## Searching Objects

```rust
use anytype_core::{AnytypeClient, SearchRequest, Result};

async fn search_objects() -> Result<()> {
    let mut client = AnytypeClient::new()?;
    client.set_api_key("your-jwt-token".to_string());
    
    // Basic search
    let request = SearchRequest {
        query: Some("important notes".to_string()),
        limit: Some(10),
        offset: Some(0),
        space_id: None, // Search all spaces
    };
    
    let results = client.search(request).await?;
    println!("Found {} objects", results.objects.len());
    
    for object in results.objects {
        println!("Object: {} in space {}", object.id, object.space_id);
        
        // Access object properties
        if let Some(properties) = object.properties.as_object() {
            for (key, value) in properties {
                println!("  {}: {}", key, value);
            }
        }
    }
    
    Ok(())
}
```

## Working with Templates

```rust
use anytype_core::{AnytypeClient, Result};

async fn get_template_details() -> Result<()> {
    let mut client = AnytypeClient::new()?;
    client.set_api_key("your-jwt-token".to_string());
    
    // List templates for a specific type
    let templates = client.list_templates("space_id", "type_id").await?;
    
    println!("Found {} templates", templates.len());
    for template in templates {
        println!("- {} ({})", template.name.as_deref().unwrap_or("Unnamed"), template.id);
        if let Some(snippet) = template.snippet {
            println!("  Snippet: {}", snippet);
        }
    }
    
    // Get template details
    let template = client.get_template("space_id", "type_id", "template_id").await?;
    
    println!("Template: {}", template.name.as_deref().unwrap_or("Unnamed"));
    println!("ID: {}", template.id);
    println!("Space: {}", template.space_id);
    
    if let Some(snippet) = template.snippet {
        println!("Snippet: {}", snippet);
    }
    
    if let Some(markdown) = template.markdown {
        println!("Markdown content: {}", markdown);
    }
    
    if let Some(object_type) = template.object_type {
        println!("Type: {} ({})", object_type.name, object_type.key);
    }
    
    Ok(())
}
```

## Working with Properties

```rust
use anytype_core::{AnytypeClient, Result};

async fn list_space_properties() -> Result<()> {
    let mut client = AnytypeClient::new()?;
    client.set_api_key("your-jwt-token".to_string());
    
    // List properties in a space
    let properties = client.list_properties("space_id").await?;
    
    println!("Found {} properties", properties.len());
    for property in properties {
        println!("- {} ({})", property.name, property.key);
        println!("  ID: {}", property.id);
        println!("  Format: {}", property.format);
        println!("  Object: {}", property.object);
    }
    
    Ok(())
}
```

## Working with Tags

```rust
use anytype_core::{AnytypeClient, Result};

async fn list_property_tags() -> Result<()> {
    let mut client = AnytypeClient::new()?;
    client.set_api_key("your-jwt-token".to_string());
    
    // List tags for a specific property
    let tags = client.list_tags("space_id", "property_id").await?;
    
    println!("Found {} tags", tags.len());
    for tag in tags {
        println!("- {} ({})", tag.name, tag.key);
        println!("  ID: {}", tag.id);
        if let Some(color) = tag.color {
            println!("  Color: {}", color);
        }
        println!("  Object: {}", tag.object);
    }
    
    Ok(())
}
```

## Error Handling

```rust
use anytype_core::{AnytypeClient, AnytypeError, Result};

async fn handle_errors() -> Result<()> {
    let client = AnytypeClient::new()?;
    
    match client.list_spaces().await {
        Ok(spaces) => {
            println!("Success: {} spaces", spaces.len());
        }
        Err(AnytypeError::Auth { message }) => {
            eprintln!("Authentication error: {}", message);
            // Handle auth error - maybe redirect to login
        }
        Err(AnytypeError::Http { source }) => {
            eprintln!("Network error: {}", source);
            // Handle network issues - maybe retry
        }
        Err(AnytypeError::Api { message }) => {
            eprintln!("API error: {}", message);
            // Handle API-specific errors
        }
        Err(e) => {
            eprintln!("Other error: {}", e);
        }
    }
    
    Ok(())
}
```

## Configuration

```rust
use anytype_core::{AnytypeClient, ClientConfig};

// Default configuration (connects to local Anytype app)
let client = AnytypeClient::new()?;

// Custom configuration example
let config = ClientConfig {
    base_url: "http://localhost:31009".to_string(),
    timeout_seconds: 120, // 2 minutes timeout
    app_name: "my-rust-app".to_string(),
};

let client = AnytypeClient::with_config(config)?;
```

## Complete Example

Here's a complete example that demonstrates the full workflow with your local Anytype app:

```rust
use anytype_core::{AnytypeClient, SearchRequest, Result};
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to local Anytype app
    let mut client = AnytypeClient::new()?;
    
    // Authentication with local app
    println!("Starting authentication with local Anytype app...");
    let challenge = client.create_challenge().await?;
    
    print!("Check your Anytype app for the 4-digit code and enter it here: ");
    io::stdout().flush().unwrap();
    let mut code = String::new();
    io::stdin().read_line(&mut code).unwrap();
    let code = code.trim();
    
    let api_key_response = client.create_api_key(challenge.challenge_id, code.to_string()).await?;
    client.set_api_key(api_key_response.api_key);
    
    println!("Authentication successful!");
    
    // List spaces from your local Anytype
    let spaces = client.list_spaces().await?;
    println!("Available spaces in your local Anytype:");
    for space in &spaces {
        println!("  - {} ({})", space.name, space.id);
    }
    
    // Search for objects in your local data
    if !spaces.is_empty() {
        let search_request = SearchRequest {
            query: Some("test".to_string()),
            limit: Some(5),
            offset: Some(0),
            space_id: Some(spaces[0].id.clone()),
        };
        
        let results = client.search(search_request).await?;
        println!("Search results from local Anytype: {} objects found", results.objects.len());
        
        for object in results.objects {
            println!("  - Object: {}", object.id);
        }
    }
    
    Ok(())
}
```
