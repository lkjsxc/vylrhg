use crate::core::event_bus::Event;

#[derive(Debug, Clone)]
pub struct Tab {
    pub id: u64,
    pub title: String,
}

#[derive(Debug)]
pub struct TabManager {
    next_id: u64,
    tabs: Vec<Tab>,
    active: Option<u64>,
}

impl TabManager {
    pub fn new() -> Self {
        let mut manager = Self {
            next_id: 1,
            tabs: Vec::new(),
            active: None,
        };
        manager.create_tab("welcome".to_string());
        manager
    }

    pub fn create_tab(&mut self, title: String) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.tabs.push(Tab { id, title });
        self.active = Some(id);
        id
    }

    pub fn handle_event(&mut self, event: &Event) -> Option<String> {
        match event {
            Event::Tick => {
                let title = self
                    .active
                    .and_then(|id| self.tabs.iter().find(|tab| tab.id == id))
                    .map(|tab| tab.title.clone())
                    .unwrap_or_else(|| "none".to_string());
                Some(format!("tick active_tab={}", title))
            }
            Event::Input(command) => {
                if command.starts_with("tab:new ") {
                    let title = command.trim_start_matches("tab:new ").to_string();
                    let id = self.create_tab(title.clone());
                    Some(format!("tab created id={} title={}", id, title))
                } else {
                    Some(format!("input {}", command))
                }
            }
            Event::Shutdown => Some("shutdown".to_string()),
        }
    }
}
