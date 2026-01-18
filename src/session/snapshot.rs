use crate::layout::bindings::TileBindings;
use crate::layout::LayoutTree;
use crate::renderer::pipeline::RenderOp;
use crate::tabs::TabManager;

#[derive(Debug, Default)]
pub struct SessionSnapshot {
    pub frame: u64,
    pub active_tab_title: String,
    pub layout: String,
    pub active_tile: u64,
    pub tiles: Vec<u64>,
    pub bindings: String,
    pub binding_pairs: Vec<(u64, u64)>,
    pub last_render_ops: Vec<String>,
}

impl SessionSnapshot {
    pub fn from_state(
        tabs: &TabManager,
        frame: u64,
        layout: &LayoutTree,
        bindings: &TileBindings,
        ops: &[RenderOp],
    ) -> Self {
        Self {
            frame,
            active_tab_title: tabs.active_title(),
            layout: layout.describe(),
            active_tile: layout.active_id(),
            tiles: layout.leaf_ids(),
            bindings: bindings.describe(),
            binding_pairs: bindings.pairs(),
            last_render_ops: ops.iter().map(|op| format!("{:?}", op)).collect(),
        }
    }

    pub fn to_json(&self) -> String {
        let ops = self
            .last_render_ops
            .iter()
            .map(|op| format!("\"{}\"", escape_json(op)))
            .collect::<Vec<String>>()
            .join(",");

        let tiles = self
            .tiles
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let bindings = self
            .binding_pairs
            .iter()
            .map(|(tile, tab)| format!("[{},{}]", tile, tab))
            .collect::<Vec<String>>()
            .join(",");

        format!(
            "{{\"frame\":{},\"active_tab\":\"{}\",\"layout\":\"{}\",\"active_tile\":{},\"tiles\":[{}],\"bindings\":\"{}\",\"binding_pairs\":[{}],\"render_ops\":[{}]}}",
            self.frame,
            escape_json(&self.active_tab_title),
            escape_json(&self.layout),
            self.active_tile,
            tiles,
            escape_json(&self.bindings),
            bindings,
            ops
        )
    }
}

fn escape_json(input: &str) -> String {
    input
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}
