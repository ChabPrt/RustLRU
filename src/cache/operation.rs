use std::hash::Hash;
use crate::cache::cache::Cache;

/// Trait qui définit les opérations de base pour un cache.
pub trait CacheOperations<K, V> {
    /// Ajoute une paire clé-valeur dans le cache.
    ///
    /// Cette fonction ajoute un nouvel élément dans le cache. Si la clé existe déjà, elle est mise à jour.
    /// Si la capacité maximale du cache est atteinte, l'élément le moins utilisé est éjecté.
    ///
    /// # Paramètres
    ///
    /// - `key`: La clé de l'élément à ajouter dans le cache.
    /// - `value`: La valeur associée à la clé dans le cache.
    ///
    /// # Exemple
    ///
    /// ```
    /// use rust_lru::cache::cache::Cache;
    /// let mut cache = Cache::new(1);
    /// cache.put("key_a", "value_a");
    /// assert_eq!(cache.get(&"key_a"), Some(&"value_a"));
    /// ```
    fn put(&mut self, key: K, value: V);

    /// Récupère une valeur à partir de la clé.
    ///
    /// Cette fonction renvoie une référence à la valeur associée à la clé donnée,
    /// si elle existe dans le cache. Si la clé n'est pas présente, la fonction renvoie `None`.
    ///
    /// # Paramètres
    ///
    /// - `key`: La clé de l'élément à récupérer.
    ///
    /// # Retour
    ///
    /// - Si la clé est présente dans le cache, renvoie `Some(&V)`, où `V` est le type de la valeur associée.
    /// - Si la clé n'est pas présente, renvoie `None`.
    ///
    /// # Exemple
    ///
    /// ```
    /// use rust_lru::cache::cache::Cache;
    /// let mut cache = Cache::new(1);
    /// cache.put("key_a", "value_a");
    /// assert_eq!(cache.get(&"key_a"), Some(&"value_a"));
    /// ```
    fn get(&mut self, key: &K) -> Option<&V>;

    /// Supprime une paire clé-valeur du cache.
    ///
    /// Cette fonction supprime un élément du cache en fonction de la clé donnée.
    /// Si la clé est trouvée et supprimée, la fonction renvoie la valeur supprimée,
    /// sinon elle renvoie `None`.
    ///
    /// # Paramètres
    ///
    /// - `key`: La clé de l'élément à supprimer.
    ///
    /// # Retour
    ///
    /// - Si l'élément a été trouvé et supprimé, renvoie `Some(V)`, où `V` est la valeur associée à la clé.
    /// - Si l'élément n'existe pas, renvoie `None`.
    ///
    /// # Exemple
    ///
    /// ```
    /// use rust_lru::cache::cache::Cache;
    /// let mut cache = Cache::new(3);
    /// cache.put("key_c", "value_c");
    /// assert_eq!(cache.remove(&"key_c"), Some("value_c"));
    /// assert_eq!(cache.get(&"key_c"), None);
    /// ```
    fn remove(&mut self, key: &K) -> Option<V>;

    /// Vide le cache.
    ///
    /// Cette fonction supprime tous les éléments du cache, réinitialisant ainsi son état.
    ///
    /// # Exemple
    ///
    /// ```
    /// use rust_lru::cache::cache::Cache;
    /// let mut cache = Cache::new(3);
    /// cache.put("key_a", "value_a");
    /// cache.clear();
    /// assert_eq!(cache.is_empty(), true);
    /// ```
    fn clear(&mut self);
}

/// Implémentation du trait `CacheOperations` pour la structure `Cache`.
impl<K, V> CacheOperations<K, V> for Cache<K, V>
where K: Hash + Eq + Clone {
    fn put(&mut self, key: K, value: V) {
        self.put(key, value);
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        self.get(key)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.remove(key)
    }

    fn clear(&mut self) {
        self.clear();
    }
}
