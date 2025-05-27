use crate::config::EditorConfig;
use crate::syntax::SyntaxHighlighter;
use egui::{Context, TextEdit, Ui};
use std::fs;
use std::path::Path;
use xi_rope::{Rope, RopeDelta, Transformer};

pub struct TextEditor {
    rope: Rope,

    highlighter: SyntaxHighlighter,

    undo_stack: Vec<RopeDelta>,

    redo_stack: Vec<RopeDelta>,

    ctx: Context,

    config: EditorConfig,

    cursor_pos: usize,
}

impl TextEditor {
    pub fn new(ctx: Context, config: &EditorConfig) -> Self {
        Self {
            rope: Rope::from(""),
            highlighter: SyntaxHighlighter::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            ctx,
            config: config.clone(),
            cursor_pos: 0,
        }
    }

    pub fn clear(&mut self) {
        let old_rope = self.rope.clone();
        self.rope = Rope::from("");

        let delta = RopeDelta::delete_forward(&old_rope, 0, old_rope.len());
        self.undo_stack.push(delta);
        self.redo_stack.clear();

        self.cursor_pos = 0;
    }

    pub fn open_file(&mut self, path: &Path) -> bool {
        match fs::read_to_string(path) {
            Ok(content) => {
                self.rope = Rope::from(content);
                self.undo_stack.clear();
                self.redo_stack.clear();
                self.cursor_pos = 0;
                true
            }
            Err(_) => false,
        }
    }

    pub fn save_file(&self, path: &Path) -> bool {
        let content = self.rope.to_string();
        fs::write(path, content).is_ok()
    }

    pub fn undo(&mut self) {
        if let Some(delta) = self.undo_stack.pop() {
            let inverse = delta.invert(&self.rope);
            self.rope = inverse.apply(&self.rope);

            self.redo_stack.push(inverse.invert(&self.rope));
        }
    }

    pub fn redo(&mut self) {
        if let Some(delta) = self.redo_stack.pop() {
            self.rope = delta.apply(&self.rope);

            self.undo_stack.push(delta);
        }
    }

    pub fn cut(&mut self) {}

    pub fn copy(&mut self) {}

    pub fn paste(&mut self) {}

    pub fn format_code(&mut self) {}

    pub fn ui(&mut self, ui: &mut Ui) -> bool {
        let mut changed = false;

        let text = self.rope.to_string();

        let mut editor_text = text.clone();

        let response = TextEdit::multiline(&mut editor_text)
            .font(egui::TextStyle::Monospace)
            .code_editor()
            .desired_width(f32::INFINITY)
            .desired_rows(30)
            .lock_focus(true)
            .show(ui);

        if editor_text != text {
            let new_rope = Rope::from(editor_text);

            let delta = RopeDelta::delete_forward(&self.rope, 0, self.rope.len())
                .compose(RopeDelta::insert(&Rope::from(""), 0, new_rope));

            self.rope = new_rope;

            self.undo_stack.push(delta);
            self.redo_stack.clear();

            changed = true;
        }

        if let Some(cursor) = response.response.cursor_range {
            if let Some(pos) = cursor.primary.ccursor.index {
                self.cursor_pos = pos;
            }
        }

        changed
    }
}
