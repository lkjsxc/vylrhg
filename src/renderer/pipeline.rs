use crate::core::event_bus::Event;
use crate::assembly::parser::parse_program;
use crate::core::commands::help_text;
use crate::assembly::vm::Vm;
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
    vm: Vm,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            frame: 0,
            vm: Vm::new(),
        }
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
                if text == "sys:help" {
                    vec![RenderOp::Text(help_text().to_string())]
                } else if let Some(markup) = text.strip_prefix("markup:") {
                    let doc = parse_markup(markup);
                    vec![RenderOp::Text(format!(
                        "render markup roots={}",
                        doc.roots.len()
                    ))]
                } else if let Some(program) = text.strip_prefix("asm:") {
                    self.vm.reset();
                    let program = parse_program(program);
                    let result = self.vm.run(&program);
                    vec![RenderOp::Text(format!(
                        "exec asm halted={} stack={:?}",
                        result.halted, result.stack
                    ))]
                } else {
                    vec![RenderOp::Text(format!("render input={}", text))]
                }
            }
            Event::Shutdown => vec![RenderOp::Text("render shutdown".to_string())],
        }
    }
}
