use crate::layout::LayoutTree;
use crate::renderer::pipeline::RenderOp;
use crate::tabs::TabManager;

#[derive(Debug, Default)]
pub struct SessionSnapshot {
    pub frame: u64,
    pub active_tab_title: String,
    pub layout: String,
    pub last_render_ops: Vec<String>,
}

impl SessionSnapshot {
    pub fn from_state(
        tabs: &TabManager,
        frame: u64,
        layout: &LayoutTree,
        ops: &[RenderOp],
    ) -> Self {
        Self {
            frame,
            active_tab_title: tabs.active_title(),
            layout: layout.describe(),
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

        format!(
            "{{\"frame\":{},\"active_tab\":\"{}\",\"layout\":\"{}\",\"render_ops\":[{}]}}",
            self.frame,
            escape_json(&self.active_tab_title),
            escape_json(&self.layout),
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
