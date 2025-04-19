use crate::parser::ast::{Program, Statement, Expression, Literal};
use crate::parser::Parser;
use crate::lexer::lex_wfl_with_positions;
use std::path::Path;

pub struct CodeFixer {
    indent_size: usize,
}

pub enum FixerOutputMode {
    Stdout,        // Print fixed code to stdout
    InPlace,       // Overwrite the input file
    Diff,          // Generate a unified diff
}

pub struct FixerSummary {
    pub lines_reformatted: usize,
    pub vars_renamed: usize,
    pub dead_code_removed: usize,
}

impl CodeFixer {
    pub fn new() -> Self {
        Self {
            indent_size: 4,
        }
    }
    
    pub fn set_indent_size(&mut self, size: usize) {
        self.indent_size = size;
    }
    
    pub fn fix(&self, program: &Program, source: &str) -> (String, FixerSummary) {
        let mut output = String::new();
        let mut summary = FixerSummary {
            lines_reformatted: 0,
            vars_renamed: 0,
            dead_code_removed: 0,
        };
        
        self.pretty_print(program, &mut output, 0, &mut summary);
        
        let tokens = lex_wfl_with_positions(&output);
        let mut parser = Parser::new(&tokens);
        match parser.parse() {
            Ok(_new_program) => {
            }
            Err(_) => {
                eprintln!("Warning: Re-parsing the fixed code resulted in errors. This is a bug in the code fixer.");
            }
        }
        
        (output, summary)
    }
    
    fn pretty_print(&self, program: &Program, output: &mut String, indent_level: usize, summary: &mut FixerSummary) {
        for statement in &program.statements {
            self.pretty_print_statement(statement, output, indent_level, summary);
        }
    }
    
    fn pretty_print_statement(&self, statement: &Statement, output: &mut String, indent_level: usize, summary: &mut FixerSummary) {
        let indent = " ".repeat(indent_level * self.indent_size);
        
        match statement {
            Statement::VariableDeclaration { name, value, .. } => {
                let fixed_name = self.fix_identifier_name(name, summary);
                output.push_str(&indent);
                output.push_str("store ");
                output.push_str(&fixed_name);
                output.push_str(" as ");
                self.pretty_print_expression(value, output, indent_level, summary);
                output.push('\n');
                summary.lines_reformatted += 1;
            },
            _ => {
                output.push_str(&indent);
                output.push_str(&format!("{:?}\n", statement));
                summary.lines_reformatted += 1;
            }
        }
    }
    
    fn pretty_print_expression(&self, expression: &Expression, output: &mut String, _indent_level: usize, _summary: &mut FixerSummary) {
        match expression {
            Expression::Literal(literal, ..) => {
                match literal {
                    Literal::String(s) => {
                        output.push('"');
                        output.push_str(s);
                        output.push('"');
                    },
                    Literal::Integer(n) => {
                        output.push_str(&n.to_string());
                    },
                    Literal::Float(f) => {
                        output.push_str(&f.to_string());
                    },
                    Literal::Boolean(b) => {
                        output.push_str(if *b { "yes" } else { "no" });
                    },
                    Literal::Nothing => {
                        output.push_str("nothing");
                    },
                    Literal::Pattern(p) => {
                        output.push('/');
                        output.push_str(p);
                        output.push('/');
                    },
                }
            },
            _ => {
                output.push_str(&format!("{:?}", expression));
            }
        }
    }
    
    fn fix_identifier_name(&self, name: &str, summary: &mut FixerSummary) -> String {
        if !self.is_snake_case(name) {
            summary.vars_renamed += 1;
            self.to_snake_case(name)
        } else {
            name.to_string()
        }
    }
    
    fn is_snake_case(&self, s: &str) -> bool {
        !s.contains(char::is_uppercase) && !s.contains(' ')
    }
    
    fn to_snake_case(&self, s: &str) -> String {
        let mut result = String::new();
        let mut previous_char_is_lowercase = false;
        
        for (i, c) in s.char_indices() {
            if c.is_uppercase() {
                if i > 0 && previous_char_is_lowercase {
                    result.push('_');
                }
                result.push(c.to_lowercase().next().unwrap());
            } else if c == ' ' {
                result.push('_');
            } else {
                result.push(c);
            }
            
            previous_char_is_lowercase = c.is_lowercase();
        }
        
        result
    }
    
    pub fn generate_diff(&self, original: &str, fixed: &str) -> String {
        let mut diff = String::new();
        
        let original_lines: Vec<&str> = original.lines().collect();
        let fixed_lines: Vec<&str> = fixed.lines().collect();
        
        for i in 0..original_lines.len().max(fixed_lines.len()) {
            if i < original_lines.len() && i < fixed_lines.len() {
                if original_lines[i] != fixed_lines[i] {
                    diff.push_str(&format!("-{}\n", original_lines[i]));
                    diff.push_str(&format!("+{}\n", fixed_lines[i]));
                }
            } else if i < original_lines.len() {
                diff.push_str(&format!("-{}\n", original_lines[i]));
            } else if i < fixed_lines.len() {
                diff.push_str(&format!("+{}\n", fixed_lines[i]));
            }
        }
        
        diff
    }
    
    pub fn load_config(&mut self, dir: &Path) {
        if let Ok(config) = crate::config::load_config(dir) {
        }
    }
}

#[cfg(test)]
mod tests;
