use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct TileBindings {
    map: HashMap<u64, u64>,
}

impl TileBindings {
    pub fn new(default_tab: u64) -> Self {
        let mut map = HashMap::new();
        map.insert(1, default_tab);
        Self { map }
    }

    pub fn ensure_tiles(&mut self, tiles: &[u64], default_tab: u64) {
        for tile in tiles {
            self.map.entry(*tile).or_insert(default_tab);
        }
    }

    pub fn bind(&mut self, tile: u64, tab: u64) -> bool {
        if self.map.contains_key(&tile) {
            self.map.insert(tile, tab);
            true
        } else {
            false
        }
    }

    pub fn unbind(&mut self, tile: u64) -> bool {
        self.map.remove(&tile).is_some()
    }

    pub fn describe(&self) -> String {
        let mut pairs = self
            .map
            .iter()
            .map(|(tile, tab)| format!("{}->{}", tile, tab))
            .collect::<Vec<String>>();
        pairs.sort();
        pairs.join(",")
    }

    pub fn pairs(&self) -> Vec<(u64, u64)> {
        let mut pairs = self.map.iter().map(|(t, tab)| (*t, *tab)).collect();
        pairs.sort_by(|a, b| a.0.cmp(&b.0));
        pairs
    }
}
