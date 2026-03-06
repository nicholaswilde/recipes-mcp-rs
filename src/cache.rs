use serde::{Deserialize, Serialize};
use std::time::{SystemTime, Duration};

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheEntry<T> {
    pub data: T,
    pub timestamp: SystemTime,
    pub ttl: Duration,
}

impl<T> CacheEntry<T> {
    pub fn new(data: T, ttl: Duration) -> Self {
        Self {
            data,
            timestamp: SystemTime::now(),
            ttl,
        }
    }

    pub fn is_expired(&self) -> bool {
        match SystemTime::now().duration_since(self.timestamp) {
            Ok(elapsed) => elapsed > self.ttl,
            Err(_) => true, // Clock went backwards, treat as expired
        }
    }
}

use crate::search::{SearchResult, RecipeProvider};
use crate::scraper::Recipe;
use crate::dietary::DietaryFilters;
use async_trait::async_trait;

#[async_trait]
pub trait RecipeCache: Send + Sync {
    async fn get_search_results(&self, query: &str, limit: u32, provider: &RecipeProvider, filters: &DietaryFilters) -> Option<Vec<SearchResult>>;
    async fn set_search_results(&self, query: &str, limit: u32, provider: &RecipeProvider, filters: &DietaryFilters, results: Vec<SearchResult>, ttl: Duration);
    
    async fn get_recipe(&self, url: &str) -> Option<Recipe>;
    async fn set_recipe(&self, url: &str, recipe: Recipe, ttl: Duration);

    async fn cleanup_expired(&self);
}

use std::path::PathBuf;
use std::fs;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct FileRecipeCache {
    cache_dir: PathBuf,
}

impl FileRecipeCache {
    pub fn new(cache_dir: PathBuf) -> Self {
        if !cache_dir.exists() {
            fs::create_dir_all(&cache_dir).expect("Failed to create cache directory");
        }
        Self { cache_dir }
    }

    fn get_search_key(query: &str, limit: u32, provider: &RecipeProvider, filters: &DietaryFilters) -> String {
        let mut hasher = DefaultHasher::new();
        query.hash(&mut hasher);
        limit.hash(&mut hasher);
        format!("{:?}", provider).hash(&mut hasher);
        format!("{:?}", filters).hash(&mut hasher);
        format!("search_{:x}.json", hasher.finish())
    }

    fn get_recipe_key(url: &str) -> String {
        let mut hasher = DefaultHasher::new();
        url.hash(&mut hasher);
        format!("recipe_{:x}.json", hasher.finish())
    }
}

#[async_trait]
impl RecipeCache for FileRecipeCache {
    async fn get_search_results(&self, query: &str, limit: u32, provider: &RecipeProvider, filters: &DietaryFilters) -> Option<Vec<SearchResult>> {
        let key = Self::get_search_key(query, limit, provider, filters);
        let path = self.cache_dir.join(key);
        
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(entry) = serde_json::from_str::<CacheEntry<Vec<SearchResult>>>(&content) {
                if !entry.is_expired() {
                    return Some(entry.data);
                } else {
                    let _ = fs::remove_file(path);
                }
            }
        }
        None
    }

    async fn set_search_results(&self, query: &str, limit: u32, provider: &RecipeProvider, filters: &DietaryFilters, results: Vec<SearchResult>, ttl: Duration) {
        let key = Self::get_search_key(query, limit, provider, filters);
        let path = self.cache_dir.join(key);
        let entry = CacheEntry::new(results, ttl);
        
        if let Ok(content) = serde_json::to_string(&entry) {
            let _ = fs::write(path, content);
        }
    }

    async fn get_recipe(&self, url: &str) -> Option<Recipe> {
        let key = Self::get_recipe_key(url);
        let path = self.cache_dir.join(key);
        
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(entry) = serde_json::from_str::<CacheEntry<Recipe>>(&content) {
                if !entry.is_expired() {
                    return Some(entry.data);
                } else {
                    let _ = fs::remove_file(path);
                }
            }
        }
        None
    }

    async fn set_recipe(&self, url: &str, recipe: Recipe, ttl: Duration) {
        let key = Self::get_recipe_key(url);
        let path = self.cache_dir.join(key);
        let entry = CacheEntry::new(recipe, ttl);
        
        if let Ok(content) = serde_json::to_string(&entry) {
            let _ = fs::write(path, content);
        }
    }

    async fn cleanup_expired(&self) {
        if let Ok(entries) = fs::read_dir(&self.cache_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        // We need to try deserializing as either SearchResult or Recipe entry
                        // Or we can just check the timestamp/ttl if we had a generic header
                        // For simplicity, we'll try to deserialize as CacheEntry<serde_json::Value>
                        if let Ok(entry) = serde_json::from_str::<CacheEntry<serde_json::Value>>(&content) {
                            if entry.is_expired() {
                                let _ = fs::remove_file(path);
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::search::RecipeProvider;
    use crate::dietary::DietaryFilters;
    use std::time::Duration;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_file_cache_search() {
        let dir = tempdir().unwrap();
        let cache = FileRecipeCache::new(dir.path().to_path_buf());
        
        let query = "pizza";
        let limit = 5;
        let provider = RecipeProvider::AllRecipes;
        let filters = DietaryFilters::default();
        let results = vec![SearchResult {
            title: "Pizza".to_string(),
            url: "http://example.com/pizza".to_string(),
        }];
        
        // Should be None initially
        assert!(cache.get_search_results(query, limit, &provider, &filters).await.is_none());
        
        // Set results
        cache.set_search_results(query, limit, &provider, &filters, results.clone(), Duration::from_secs(60)).await;
        
        // Should be Some now
        let cached = cache.get_search_results(query, limit, &provider, &filters).await.unwrap();
        assert_eq!(cached, results);
    }

    #[tokio::test]
    async fn test_file_cache_recipe() {
        let dir = tempdir().unwrap();
        let cache = FileRecipeCache::new(dir.path().to_path_buf());
        
        let url = "http://example.com/recipe";
        let recipe = Recipe {
            name: Some("Test Recipe".to_string()),
            ..Default::default()
        };
        
        // Should be None initially
        assert!(cache.get_recipe(url).await.is_none());
        
        // Set recipe
        cache.set_recipe(url, recipe.clone(), Duration::from_secs(60)).await;
        
        // Should be Some now
        let cached = cache.get_recipe(url).await.unwrap();
        assert_eq!(cached, recipe);
    }

    #[tokio::test]
    async fn test_cache_expiration() {
        let dir = tempdir().unwrap();
        let cache = FileRecipeCache::new(dir.path().to_path_buf());
        
        let url = "http://example.com/expired";
        let recipe = Recipe::default();
        
        // Set with 0 TTL
        cache.set_recipe(url, recipe.clone(), Duration::from_secs(0)).await;
        
        // Should be None due to expiration
        assert!(cache.get_recipe(url).await.is_none());
    }

    #[tokio::test]
    async fn test_cleanup_expired() {
        let dir = tempdir().unwrap();
        let cache = FileRecipeCache::new(dir.path().to_path_buf());
        
        // Set one valid and one expired
        cache.set_recipe("http://example.com/valid", Recipe::default(), Duration::from_secs(60)).await;
        cache.set_recipe("http://example.com/expired", Recipe::default(), Duration::from_secs(0)).await;
        
        // Verify both files exist
        let count = fs::read_dir(dir.path()).unwrap().count();
        assert_eq!(count, 2);
        
        // Cleanup
        cache.cleanup_expired().await;
        
        // Verify only one file remains
        let count = fs::read_dir(dir.path()).unwrap().count();
        assert_eq!(count, 1);
    }
}
