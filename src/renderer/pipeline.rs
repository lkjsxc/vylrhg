use crate::core::event_bus::Event;
use crate::markup::parser::parse_markup;

#[derive(Debug, Clone)]
pub enum RenderOp {
    FrameBegin,
    FrameEnd,
    Text(String),
}

#[derive(Debug, Default)]
pub struct Renderer {
    frame: u64,
}

impl Renderer {
    pub fn new() -> Self {
        Self { frame: 0 }
    }

    pub fn handle_event(&mut self, event: &Event) -> Vec<RenderOp> {
        match event {
            Event::Tick => {
                self.frame += 1;
                vec![
                    RenderOp::FrameBegin,
                    RenderOp::Text(format!("render frame={}", self.frame)),
                    RenderOp::FrameEnd,
                ]
            }
            Event::Input(text) => {
                if let Some(markup) = text.strip_prefix("markup:") {
                    let doc = parse_markup(markup);
                    vec![RenderOp::Text(format!(
                        "render markup roots={}",
                        doc.roots.len()
                    ))]
                } else {
                    vec![RenderOp::Text(format!("render input={}", text))]
                }
            }
            Event::Shutdown => vec![RenderOp::Text("render shutdown".to_string())],
        }
    }
}
