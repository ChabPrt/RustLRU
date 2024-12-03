use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::fmt;

/// Structure représentant un cache avec une capacité limitée.
/// Ce cache maintient un ensemble de paires clé-valeur, en conservant l'ordre d'insertion
/// et en éjectant les éléments les moins utilisés lorsque la capacité est atteinte.
pub struct Cache<K, V> {
    capacity: usize,
    data: HashMap<K, V>,
    keys_order: VecDeque<K>,
}

/// Implémentation de la structure `Cache`.
impl<K, V> Cache<K, V>
where K: Hash + Eq + Clone {
    /// Crée un nouveau cache avec une capacité donnée.
    ///
    /// Cette fonction permet de créer une nouvelle instance de `Cache` en spécifiant la capacité maximale.
    /// Le cache sera initialement vide.
    ///
    /// # Paramètres
    ///
    /// - `capacity`: La capacité maximale du cache. Une fois cette capacité atteinte, les éléments les moins utilisés
    ///   seront éjectés pour laisser place aux nouveaux éléments.
    ///
    /// # Retour
    ///
    /// - Retourne une nouvelle instance de `Cache` avec la capacité spécifiée.
    ///
    /// # Exemple
    ///
    /// ```
    /// use rust_lru::cache::cache::Cache;
    /// let cache = Cache::new(3);
    /// assert_eq!(cache.len(), 0);
    /// ```
    pub fn new(capacity: usize) -> Cache<K, V> {
        Cache {
            capacity,
            data: HashMap::new(),
            keys_order: VecDeque::new(),
        }
    }

    /// Ajoute une paire clé-valeur dans le cache.
    ///
    /// Cette fonction met à jour ou ajoute un élément dans le cache. Si la clé existe déjà,
    /// la valeur est mise à jour et sa position dans l'ordre des clés est modifiée.
    /// Si la capacité maximale du cache est atteinte, l'élément le moins utilisé est supprimé.
    ///
    /// # Paramètres
    ///
    /// - `key`: La clé de l'élément à ajouter.
    /// - `value`: La valeur à associer à la clé dans le cache.
    ///
    /// # Exemple
    ///
    /// ```
    /// use rust_lru::cache::cache::Cache;
    /// let mut cache = Cache::new(1);
    /// cache.put("key_a", "value_a");
    /// assert_eq!(cache.get(&"key_a"), Some(&"value_a"));
    /// ```
    pub fn put(&mut self, key: K, value: V) {
        if self.data.contains_key(&key) {
            self.keys_order.retain(|k| k != &key);
        } else if self.data.len() == self.capacity {
            if let Some(oldest) = self.keys_order.pop_front() {
                self.data.remove(&oldest);
            }
        }

        self.data.insert(key.clone(), value);
        self.keys_order.push_back(key);
    }

    /// Récupère la valeur associée à la clé dans le cache.
    ///
    /// # Paramètres
    ///
    /// - `key`: La clé de l'élément à récupérer.
    ///
    /// # Retour
    ///
    /// - Retourne `Some(&V)` si la clé existe dans le cache, sinon `None`.
    ///
    /// # Exemple
    ///
    /// ```
    /// use rust_lru::cache::cache::Cache;
    /// let mut cache = Cache::new(1);
    /// cache.put("key_a", "value_a");
    /// assert_eq!(cache.get(&"key_a"), Some(&"value_a"));
    /// ```
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.data.contains_key(key) {
            self.keys_order.retain(|k| k != key);
            self.keys_order.push_front(key.clone());
            return self.data.get(key);
        }
        None
    }

    /// Supprime une paire clé-valeur du cache.
    ///
    /// # Paramètres
    ///
    /// - `key`: La clé de l'élément à supprimer.
    ///
    /// # Retour
    ///
    /// - Si l'élément a été trouvé et supprimé, renvoie la valeur associée à cette clé.
    ///   Sinon, renvoie `None`.
    ///
    /// # Exemple
    ///
    /// ```
    /// use rust_lru::cache::cache::Cache;
    /// let mut cache = Cache::new(1);
    /// cache.put("key_a", "value_a");
    /// cache.remove(&"key_a");
    /// assert_eq!(cache.get(&"key_a"), None);
    /// ```
    pub fn remove(&mut self, key: &K) -> Option<V> {
        if self.data.contains_key(key) {
            self.keys_order.retain(|k| k != key);
            self.data.remove(key)
        } else {
            None
        }
    }

    /// Vide le cache.
    ///
    /// # Exemple
    ///
    /// ```
    /// use rust_lru::cache::cache::Cache;
    /// let mut cache = Cache::new(1);
    /// cache.put("key_a", "value_a");
    /// cache.clear();
    /// assert_eq!(cache.is_empty(), true);
    /// ```
    pub fn clear(&mut self) {
        self.data.clear();
        self.keys_order.clear();
    }

    /// Retourne le nombre d'éléments dans le cache.
    ///
    /// # Retour
    ///
    /// - Retourne un entier représentant le nombre d'éléments dans le cache.
    ///
    /// # Exemple
    ///
    /// ```
    /// use rust_lru::cache::cache::Cache;
    /// let mut cache = Cache::new(1);
    /// cache.put("key_a", "value_a");
    /// assert_eq!(cache.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Vérifie si le cache est vide.
    ///
    /// # Retour
    ///
    /// - Retourne `true` si le cache est vide, sinon `false`.
    ///
    /// # Exemple
    ///
    /// ```
    /// use rust_lru::cache::cache::Cache;
    /// let mut cache = Cache::new(1);
    /// assert_eq!(cache.is_empty(), true);
    /// cache.put("key_a", "value_a");
    /// assert_eq!(cache.is_empty(), false);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

/// Implémentation du trait `fmt::Display` pour afficher le cache.
impl<K: fmt::Display + Hash + Eq + Clone, V> fmt::Display for Cache<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cache [Capacity: {}, Keys: [", self.capacity)?;
        for key in &self.keys_order {
            write!(f, "{}, ", key)?;
        }
        write!(f, "]]")
    }
}
