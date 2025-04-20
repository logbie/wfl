use super::Analyzer;
use crate::diagnostics::{DiagnosticReporter, Severity, WflDiagnostic};
use crate::parser::ast::{Expression, Program, Statement, Type};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct VariableUsage {
    name: String,
    defined_at: (usize, usize), // (line, column)
    used: bool,
}

#[derive(Debug, Clone)]
enum CFGNode {
    Entry,
    Exit,
    Statement {
        stmt_idx: usize,
        line: usize,
        column: usize,
    },
    Branch {
        condition_idx: usize,
        then_branch: usize,
        else_branch: Option<usize>,
        line: usize,
        column: usize,
    },
}

#[derive(Debug, Clone)]
struct ControlFlowGraph {
    nodes: Vec<CFGNode>,
    edges: HashMap<usize, Vec<usize>>, // node_idx -> [successor_idx]
    reachable: HashSet<usize>,         // Set of reachable node indices
}

impl ControlFlowGraph {
    fn new() -> Self {
        let mut cfg = Self {
            nodes: vec![CFGNode::Entry, CFGNode::Exit],
            edges: HashMap::new(),
            reachable: HashSet::new(),
        };

        cfg.reachable.insert(0);

        cfg
    }

    fn add_node(&mut self, node: CFGNode) -> usize {
        let idx = self.nodes.len();
        self.nodes.push(node);
        idx
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        self.edges.entry(from).or_insert_with(Vec::new).push(to);

        if self.reachable.contains(&from) {
            self.reachable.insert(to);
        }
    }

    fn compute_reachability(&mut self) {
        self.reachable.clear();
        self.reachable.insert(0);

        let mut changed = true;
        while changed {
            changed = false;

            for (&from, to_nodes) in &self.edges {
                if self.reachable.contains(&from) {
                    for &to in to_nodes {
                        if !self.reachable.contains(&to) {
                            self.reachable.insert(to);
                            changed = true;
                        }
                    }
                }
            }
        }
    }

    fn is_reachable(&self, node_idx: usize) -> bool {
        self.reachable.contains(&node_idx)
    }
}

pub trait StaticAnalyzer {
    fn analyze_static(&mut self, program: &Program, file_id: usize) -> Vec<WflDiagnostic>;

    fn check_unused_variables(&self, program: &Program, file_id: usize) -> Vec<WflDiagnostic>;

    fn check_unreachable_code(&self, program: &Program, file_id: usize) -> Vec<WflDiagnostic>;

    fn check_shadowing(&self, program: &Program, file_id: usize) -> Vec<WflDiagnostic>;

    fn check_inconsistent_returns(&self, program: &Program, file_id: usize) -> Vec<WflDiagnostic>;
}

impl StaticAnalyzer for Analyzer {
    fn analyze_static(&mut self, program: &Program, file_id: usize) -> Vec<WflDiagnostic> {
        let mut diagnostics = Vec::new();

        if let Err(errors) = self.analyze(program) {
            for error in errors {
                diagnostics.push(WflDiagnostic::new(
                    Severity::Error,
                    error.message.clone(),
                    None::<String>,
                    "ANALYZE-SEMANTIC".to_string(),
                    file_id,
                    error.line,
                    error.column,
                    None,
                ));
            }

            return diagnostics;
        }

        diagnostics.extend(self.check_unused_variables(program, file_id));
        diagnostics.extend(self.check_unreachable_code(program, file_id));
        diagnostics.extend(self.check_shadowing(program, file_id));
        diagnostics.extend(self.check_inconsistent_returns(program, file_id));

        diagnostics
    }

    fn check_unused_variables(&self, program: &Program, file_id: usize) -> Vec<WflDiagnostic> {
        let mut diagnostics = Vec::new();
        let mut variable_usages = HashMap::new();

        for statement in &program.statements {
            self.collect_variable_declarations(statement, &mut variable_usages);
        }

        for statement in &program.statements {
            self.mark_used_variables(statement, &mut variable_usages);
        }

        for (name, usage) in variable_usages {
            if !usage.used {
                diagnostics.push(WflDiagnostic::new(
                    Severity::Warning,
                    format!("Unused variable '{}'", name),
                    Some("Consider removing this variable if it's not needed".to_string()),
                    "ANALYZE-UNUSED".to_string(),
                    file_id,
                    usage.defined_at.0,
                    usage.defined_at.1,
                    None,
                ));
            }
        }

        diagnostics
    }

    fn check_unreachable_code(&self, program: &Program, file_id: usize) -> Vec<WflDiagnostic> {
        let mut diagnostics = Vec::new();

        let mut cfg = ControlFlowGraph::new();
        self.build_cfg(program, &mut cfg);

        cfg.compute_reachability();

        for (idx, node) in cfg.nodes.iter().enumerate() {
            if !cfg.is_reachable(idx) {
                match node {
                    CFGNode::Statement { line, column, .. } => {
                        diagnostics.push(WflDiagnostic::new(
                            Severity::Warning,
                            "Unreachable code".to_string(),
                            Some("This code will never be executed".to_string()),
                            "ANALYZE-UNREACHABLE".to_string(),
                            file_id,
                            *line,
                            *column,
                            None,
                        ));
                    }
                    CFGNode::Branch { line, column, .. } => {
                        diagnostics.push(WflDiagnostic::new(
                            Severity::Warning,
                            "Unreachable branch".to_string(),
                            Some("This branch will never be executed".to_string()),
                            "ANALYZE-DEADBRANCH".to_string(),
                            file_id,
                            *line,
                            *column,
                            None,
                        ));
                    }
                    _ => {}
                }
            }
        }

        diagnostics
    }

    fn check_shadowing(&self, program: &Program, file_id: usize) -> Vec<WflDiagnostic> {
        let mut diagnostics = Vec::new();
        let mut global_scope = HashMap::new();
        let parent_scopes: Vec<HashMap<String, (usize, usize)>> = Vec::new();

        self.check_shadowing_in_statements(
            &program.statements,
            &mut global_scope,
            &parent_scopes,
            file_id,
            &mut diagnostics,
        );

        diagnostics
    }

    fn check_inconsistent_returns(&self, program: &Program, file_id: usize) -> Vec<WflDiagnostic> {
        let mut diagnostics = Vec::new();

        for statement in &program.statements {
            if let Statement::ActionDefinition {
                name,
                body,
                return_type,
                line,
                column,
                ..
            } = statement
            {
                if let Some(ret_type) = return_type {
                    if *ret_type != Type::Nothing {
                        let mut has_return = false;
                        let all_paths_return = self.check_all_paths_return(body, &mut has_return);

                        if has_return && !all_paths_return {
                            diagnostics.push(WflDiagnostic::new(
                                Severity::Warning,
                                format!("Action '{}' has inconsistent return paths", name),
                                Some("Ensure all code paths return a value".to_string()),
                                "ANALYZE-RETURN".to_string(),
                                file_id,
                                *line,
                                *column,
                                None,
                            ));
                        }
                    }
                }
            }
        }

        diagnostics
    }
}

impl Analyzer {
    fn collect_variable_declarations(
        &self,
        statement: &Statement,
        usages: &mut HashMap<String, VariableUsage>,
    ) {
        match statement {
            Statement::VariableDeclaration {
                name, line, column, ..
            } => {
                usages.insert(
                    name.clone(),
                    VariableUsage {
                        name: name.clone(),
                        defined_at: (*line, *column),
                        used: false,
                    },
                );
            }
            Statement::ActionDefinition {
                parameters, body, ..
            } => {
                for param in parameters {
                    usages.insert(
                        param.name.clone(),
                        VariableUsage {
                            name: param.name.clone(),
                            defined_at: (0, 0), // We don't have line/column for parameters yet
                            used: false,
                        },
                    );
                }

                for stmt in body {
                    self.collect_variable_declarations(stmt, usages);
                }
            }
            Statement::IfStatement {
                then_block,
                else_block,
                ..
            } => {
                for stmt in then_block {
                    self.collect_variable_declarations(stmt, usages);
                }

                if let Some(else_stmts) = else_block {
                    for stmt in else_stmts {
                        self.collect_variable_declarations(stmt, usages);
                    }
                }
            }
            Statement::WhileLoop { body, .. }
            | Statement::ForEachLoop { body, .. }
            | Statement::CountLoop { body, .. } => {
                for stmt in body {
                    self.collect_variable_declarations(stmt, usages);
                }
            }
            _ => {}
        }
    }

    fn mark_used_variables(
        &self,
        statement: &Statement,
        usages: &mut HashMap<String, VariableUsage>,
    ) {
        match statement {
            Statement::Assignment { name, value, .. } => {
                if let Some(usage) = usages.get_mut(name) {
                    usage.used = true;
                }

                self.mark_used_in_expression(value, usages);
            }
            Statement::IfStatement {
                condition,
                then_block,
                else_block,
                ..
            } => {
                self.mark_used_in_expression(condition, usages);

                for stmt in then_block {
                    self.mark_used_variables(stmt, usages);
                }

                if let Some(else_stmts) = else_block {
                    for stmt in else_stmts {
                        self.mark_used_variables(stmt, usages);
                    }
                }
            }
            Statement::WhileLoop {
                condition, body, ..
            } => {
                self.mark_used_in_expression(condition, usages);

                for stmt in body {
                    self.mark_used_variables(stmt, usages);
                }
            }
            Statement::ForEachLoop {
                item_name,
                collection,
                body,
                ..
            } => {
                if let Some(usage) = usages.get_mut(item_name) {
                    usage.used = true;
                }

                self.mark_used_in_expression(collection, usages);

                for stmt in body {
                    self.mark_used_variables(stmt, usages);
                }
            }
            Statement::CountLoop {
                start,
                end,
                step,
                body,
                ..
            } => {
                self.mark_used_in_expression(start, usages);
                self.mark_used_in_expression(end, usages);
                if let Some(step_expr) = step {
                    self.mark_used_in_expression(step_expr, usages);
                }

                for stmt in body {
                    self.mark_used_variables(stmt, usages);
                }
            }
            Statement::DisplayStatement { value, .. }
            | Statement::ReturnStatement {
                value: Some(value), ..
            } => {
                self.mark_used_in_expression(value, usages);
            }
            Statement::ExpressionStatement { expression, .. } => {
                self.mark_used_in_expression(expression, usages);
            }
            _ => {}
        }
    }

    fn mark_used_in_expression(
        &self,
        expression: &Expression,
        usages: &mut HashMap<String, VariableUsage>,
    ) {
        match expression {
            Expression::Variable(name, ..) => {
                if let Some(usage) = usages.get_mut(name) {
                    usage.used = true;
                }
            }
            Expression::BinaryOperation { left, right, .. } => {
                self.mark_used_in_expression(left, usages);
                self.mark_used_in_expression(right, usages);
            }
            Expression::UnaryOperation { expression, .. } => {
                self.mark_used_in_expression(expression, usages);
            }
            Expression::FunctionCall {
                function,
                arguments,
                ..
            } => {
                self.mark_used_in_expression(function, usages);
                for arg in arguments {
                    self.mark_used_in_expression(&arg.value, usages);
                }
            }
            Expression::MemberAccess { object, .. } => {
                self.mark_used_in_expression(object, usages);
            }
            Expression::IndexAccess {
                collection, index, ..
            } => {
                self.mark_used_in_expression(collection, usages);
                self.mark_used_in_expression(index, usages);
            }
            Expression::Concatenation { left, right, .. } => {
                self.mark_used_in_expression(left, usages);
                self.mark_used_in_expression(right, usages);
            }
            Expression::PatternMatch { text, pattern, .. }
            | Expression::PatternFind { text, pattern, .. }
            | Expression::PatternSplit { text, pattern, .. } => {
                self.mark_used_in_expression(text, usages);
                self.mark_used_in_expression(pattern, usages);
            }
            Expression::PatternReplace {
                text,
                pattern,
                replacement,
                ..
            } => {
                self.mark_used_in_expression(text, usages);
                self.mark_used_in_expression(pattern, usages);
                self.mark_used_in_expression(replacement, usages);
            }
            Expression::AwaitExpression { expression, .. } => {
                self.mark_used_in_expression(expression, usages);
            }
            _ => {}
        }
    }

    fn build_cfg(&self, program: &Program, cfg: &mut ControlFlowGraph) {
        let mut stmt_nodes = Vec::new();
        for (idx, statement) in program.statements.iter().enumerate() {
            match statement {
                Statement::IfStatement { line, column, .. } => {
                    let node_idx = cfg.add_node(CFGNode::Branch {
                        condition_idx: idx,
                        then_branch: 0,    // Placeholder, will be updated
                        else_branch: None, // Placeholder, will be updated
                        line: *line,
                        column: *column,
                    });
                    stmt_nodes.push(node_idx);
                }
                _ => {
                    let node_idx = cfg.add_node(CFGNode::Statement {
                        stmt_idx: idx,
                        line: match statement {
                            Statement::VariableDeclaration { line, .. } => *line,
                            Statement::Assignment { line, .. } => *line,
                            Statement::IfStatement { line, .. } => *line,
                            Statement::SingleLineIf { line, .. } => *line,
                            Statement::ForEachLoop { line, .. } => *line,
                            Statement::CountLoop { line, .. } => *line,
                            Statement::WhileLoop { line, .. } => *line,
                            Statement::RepeatUntilLoop { line, .. } => *line,
                            Statement::ForeverLoop { line, .. } => *line,
                            Statement::DisplayStatement { line, .. } => *line,
                            Statement::ActionDefinition { line, .. } => *line,
                            Statement::ReturnStatement { line, .. } => *line,
                            Statement::ExpressionStatement { line, .. } => *line,
                            Statement::BreakStatement { line, .. } => *line,
                            Statement::ContinueStatement { line, .. } => *line,
                            Statement::OpenFileStatement { line, .. } => *line,
                            Statement::ReadFileStatement { line, .. } => *line,
                            Statement::WriteFileStatement { line, .. } => *line,
                            Statement::CloseFileStatement { line, .. } => *line,
                            Statement::WaitForStatement { line, .. } => *line,
                            Statement::TryStatement { line, .. } => *line,
                            Statement::HttpGetStatement { line, .. } => *line,
                            Statement::HttpPostStatement { line, .. } => *line,
                        },
                        column: match statement {
                            Statement::VariableDeclaration { column, .. } => *column,
                            Statement::Assignment { column, .. } => *column,
                            Statement::IfStatement { column, .. } => *column,
                            Statement::SingleLineIf { column, .. } => *column,
                            Statement::ForEachLoop { column, .. } => *column,
                            Statement::CountLoop { column, .. } => *column,
                            Statement::WhileLoop { column, .. } => *column,
                            Statement::RepeatUntilLoop { column, .. } => *column,
                            Statement::ForeverLoop { column, .. } => *column,
                            Statement::DisplayStatement { column, .. } => *column,
                            Statement::ActionDefinition { column, .. } => *column,
                            Statement::ReturnStatement { column, .. } => *column,
                            Statement::ExpressionStatement { column, .. } => *column,
                            Statement::BreakStatement { column, .. } => *column,
                            Statement::ContinueStatement { column, .. } => *column,
                            Statement::OpenFileStatement { column, .. } => *column,
                            Statement::ReadFileStatement { column, .. } => *column,
                            Statement::WriteFileStatement { column, .. } => *column,
                            Statement::CloseFileStatement { column, .. } => *column,
                            Statement::WaitForStatement { column, .. } => *column,
                            Statement::TryStatement { column, .. } => *column,
                            Statement::HttpGetStatement { column, .. } => *column,
                            Statement::HttpPostStatement { column, .. } => *column,
                        },
                    });
                    stmt_nodes.push(node_idx);
                }
            }
        }

        if !stmt_nodes.is_empty() {
            cfg.add_edge(0, stmt_nodes[0]);
        } else {
            cfg.add_edge(0, 1);
        }

        for i in 0..stmt_nodes.len() {
            let node_idx = stmt_nodes[i];

            match &program.statements[i] {
                Statement::ReturnStatement { .. } => {
                    cfg.add_edge(node_idx, 1);
                }
                Statement::IfStatement {
                    then_block,
                    else_block,
                    ..
                } => {
                    let mut then_nodes = Vec::new();
                    for (idx, stmt) in then_block.iter().enumerate() {
                        let then_node_idx = cfg.add_node(CFGNode::Statement {
                            stmt_idx: program.statements.len() + idx,
                            line: match stmt {
                                Statement::VariableDeclaration { line, .. } => *line,
                                Statement::Assignment { line, .. } => *line,
                                Statement::IfStatement { line, .. } => *line,
                                Statement::SingleLineIf { line, .. } => *line,
                                Statement::ForEachLoop { line, .. } => *line,
                                Statement::CountLoop { line, .. } => *line,
                                Statement::WhileLoop { line, .. } => *line,
                                Statement::RepeatUntilLoop { line, .. } => *line,
                                Statement::ForeverLoop { line, .. } => *line,
                                Statement::DisplayStatement { line, .. } => *line,
                                Statement::ActionDefinition { line, .. } => *line,
                                Statement::ReturnStatement { line, .. } => *line,
                                Statement::ExpressionStatement { line, .. } => *line,
                                Statement::BreakStatement { line, .. } => *line,
                                Statement::ContinueStatement { line, .. } => *line,
                                Statement::OpenFileStatement { line, .. } => *line,
                                Statement::ReadFileStatement { line, .. } => *line,
                                Statement::WriteFileStatement { line, .. } => *line,
                                Statement::CloseFileStatement { line, .. } => *line,
                                Statement::WaitForStatement { line, .. } => *line,
                                Statement::TryStatement { line, .. } => *line,
                                Statement::HttpGetStatement { line, .. } => *line,
                                Statement::HttpPostStatement { line, .. } => *line,
                            },
                            column: match stmt {
                                Statement::VariableDeclaration { column, .. } => *column,
                                Statement::Assignment { column, .. } => *column,
                                Statement::IfStatement { column, .. } => *column,
                                Statement::SingleLineIf { column, .. } => *column,
                                Statement::ForEachLoop { column, .. } => *column,
                                Statement::CountLoop { column, .. } => *column,
                                Statement::WhileLoop { column, .. } => *column,
                                Statement::RepeatUntilLoop { column, .. } => *column,
                                Statement::ForeverLoop { column, .. } => *column,
                                Statement::DisplayStatement { column, .. } => *column,
                                Statement::ActionDefinition { column, .. } => *column,
                                Statement::ReturnStatement { column, .. } => *column,
                                Statement::ExpressionStatement { column, .. } => *column,
                                Statement::BreakStatement { column, .. } => *column,
                                Statement::ContinueStatement { column, .. } => *column,
                                Statement::OpenFileStatement { column, .. } => *column,
                                Statement::ReadFileStatement { column, .. } => *column,
                                Statement::WriteFileStatement { column, .. } => *column,
                                Statement::CloseFileStatement { column, .. } => *column,
                                Statement::WaitForStatement { column, .. } => *column,
                                Statement::TryStatement { column, .. } => *column,
                                Statement::HttpGetStatement { column, .. } => *column,
                                Statement::HttpPostStatement { column, .. } => *column,
                            },
                        });
                        then_nodes.push(then_node_idx);
                    }

                    if !then_nodes.is_empty() {
                        cfg.add_edge(node_idx, then_nodes[0]);
                        for j in 0..then_nodes.len() - 1 {
                            cfg.add_edge(then_nodes[j], then_nodes[j + 1]);
                        }
                    }

                    let mut else_nodes = Vec::new();
                    if let Some(else_stmts) = else_block {
                        for (idx, stmt) in else_stmts.iter().enumerate() {
                            let else_node_idx = cfg.add_node(CFGNode::Statement {
                                stmt_idx: program.statements.len() + then_block.len() + idx,
                                line: match stmt {
                                    Statement::VariableDeclaration { line, .. } => *line,
                                    Statement::Assignment { line, .. } => *line,
                                    Statement::IfStatement { line, .. } => *line,
                                    Statement::SingleLineIf { line, .. } => *line,
                                    Statement::ForEachLoop { line, .. } => *line,
                                    Statement::CountLoop { line, .. } => *line,
                                    Statement::WhileLoop { line, .. } => *line,
                                    Statement::RepeatUntilLoop { line, .. } => *line,
                                    Statement::ForeverLoop { line, .. } => *line,
                                    Statement::DisplayStatement { line, .. } => *line,
                                    Statement::ActionDefinition { line, .. } => *line,
                                    Statement::ReturnStatement { line, .. } => *line,
                                    Statement::ExpressionStatement { line, .. } => *line,
                                    Statement::BreakStatement { line, .. } => *line,
                                    Statement::ContinueStatement { line, .. } => *line,
                                    Statement::OpenFileStatement { line, .. } => *line,
                                    Statement::ReadFileStatement { line, .. } => *line,
                                    Statement::WriteFileStatement { line, .. } => *line,
                                    Statement::CloseFileStatement { line, .. } => *line,
                                    Statement::WaitForStatement { line, .. } => *line,
                                    Statement::TryStatement { line, .. } => *line,
                                    Statement::HttpGetStatement { line, .. } => *line,
                                    Statement::HttpPostStatement { line, .. } => *line,
                                },
                                column: match stmt {
                                    Statement::VariableDeclaration { column, .. } => *column,
                                    Statement::Assignment { column, .. } => *column,
                                    Statement::IfStatement { column, .. } => *column,
                                    Statement::SingleLineIf { column, .. } => *column,
                                    Statement::ForEachLoop { column, .. } => *column,
                                    Statement::CountLoop { column, .. } => *column,
                                    Statement::WhileLoop { column, .. } => *column,
                                    Statement::RepeatUntilLoop { column, .. } => *column,
                                    Statement::ForeverLoop { column, .. } => *column,
                                    Statement::DisplayStatement { column, .. } => *column,
                                    Statement::ActionDefinition { column, .. } => *column,
                                    Statement::ReturnStatement { column, .. } => *column,
                                    Statement::ExpressionStatement { column, .. } => *column,
                                    Statement::BreakStatement { column, .. } => *column,
                                    Statement::ContinueStatement { column, .. } => *column,
                                    Statement::OpenFileStatement { column, .. } => *column,
                                    Statement::ReadFileStatement { column, .. } => *column,
                                    Statement::WriteFileStatement { column, .. } => *column,
                                    Statement::CloseFileStatement { column, .. } => *column,
                                    Statement::WaitForStatement { column, .. } => *column,
                                    Statement::TryStatement { column, .. } => *column,
                                    Statement::HttpGetStatement { column, .. } => *column,
                                    Statement::HttpPostStatement { column, .. } => *column,
                                },
                            });
                            else_nodes.push(else_node_idx);
                        }

                        if !else_nodes.is_empty() {
                            cfg.add_edge(node_idx, else_nodes[0]);
                            for j in 0..else_nodes.len() - 1 {
                                cfg.add_edge(else_nodes[j], else_nodes[j + 1]);
                            }
                        }
                    }

                    if i < stmt_nodes.len() - 1 {
                        let next_idx = stmt_nodes[i + 1];

                        if !then_nodes.is_empty() {
                            cfg.add_edge(*then_nodes.last().unwrap(), next_idx);
                        } else {
                            cfg.add_edge(node_idx, next_idx);
                        }

                        if !else_nodes.is_empty() {
                            cfg.add_edge(*else_nodes.last().unwrap(), next_idx);
                        } else if else_block.is_some() {
                            cfg.add_edge(node_idx, next_idx);
                        }
                    } else {
                        if !then_nodes.is_empty() {
                            cfg.add_edge(*then_nodes.last().unwrap(), 1);
                        } else {
                            cfg.add_edge(node_idx, 1);
                        }

                        if !else_nodes.is_empty() {
                            cfg.add_edge(*else_nodes.last().unwrap(), 1);
                        } else if else_block.is_some() {
                            cfg.add_edge(node_idx, 1);
                        }
                    }

                    continue;
                }
                _ => {
                    if i < stmt_nodes.len() - 1 {
                        cfg.add_edge(node_idx, stmt_nodes[i + 1]);
                    } else {
                        cfg.add_edge(node_idx, 1);
                    }
                }
            }
        }
    }

    fn check_shadowing_in_statements(
        &self,
        statements: &[Statement],
        current_scope: &mut HashMap<String, (usize, usize)>,
        parent_scopes: &[HashMap<String, (usize, usize)>],
        file_id: usize,
        diagnostics: &mut Vec<WflDiagnostic>,
    ) {
        for statement in statements {
            match statement {
                Statement::VariableDeclaration {
                    name, line, column, ..
                } => {
                    for scope in parent_scopes.iter() {
                        if let Some(&(def_line, def_col)) = scope.get(name) {
                            diagnostics.push(WflDiagnostic::new(
                                Severity::Warning,
                                format!(
                                    "Variable '{}' shadows another variable with the same name",
                                    name
                                ),
                                Some(format!(
                                    "Previously defined at line {}, column {}",
                                    def_line, def_col
                                )),
                                "ANALYZE-SHADOW".to_string(),
                                file_id,
                                *line,
                                *column,
                                None,
                            ));
                            break;
                        }
                    }

                    if let Some(&(def_line, def_col)) = current_scope.get(name) {
                        diagnostics.push(WflDiagnostic::new(
                            Severity::Warning,
                            format!(
                                "Variable '{}' shadows another variable with the same name",
                                name
                            ),
                            Some(format!(
                                "Previously defined at line {}, column {}",
                                def_line, def_col
                            )),
                            "ANALYZE-SHADOW".to_string(),
                            file_id,
                            *line,
                            *column,
                            None,
                        ));
                    }

                    current_scope.insert(name.clone(), (*line, *column));
                }
                Statement::ActionDefinition {
                    parameters, body, ..
                } => {
                    let mut action_scope = HashMap::new();

                    for param in parameters {
                        action_scope.insert(param.name.clone(), (0, 0)); // We don't have line/column for parameters yet
                    }

                    let mut new_parent_scopes = parent_scopes.to_vec();
                    new_parent_scopes.push(current_scope.clone());

                    self.check_shadowing_in_statements(
                        body,
                        &mut action_scope,
                        &new_parent_scopes,
                        file_id,
                        diagnostics,
                    );
                }
                Statement::IfStatement {
                    then_block,
                    else_block,
                    ..
                } => {
                    let mut then_scope = HashMap::new();

                    let mut new_parent_scopes = parent_scopes.to_vec();
                    new_parent_scopes.push(current_scope.clone());

                    self.check_shadowing_in_statements(
                        then_block,
                        &mut then_scope,
                        &new_parent_scopes,
                        file_id,
                        diagnostics,
                    );

                    if let Some(else_stmts) = else_block {
                        let mut else_scope = HashMap::new();
                        self.check_shadowing_in_statements(
                            else_stmts,
                            &mut else_scope,
                            &new_parent_scopes,
                            file_id,
                            diagnostics,
                        );
                    }
                }
                Statement::WhileLoop { body, .. }
                | Statement::ForEachLoop { body, .. }
                | Statement::CountLoop { body, .. } => {
                    let mut loop_scope = HashMap::new();

                    let mut new_parent_scopes = parent_scopes.to_vec();
                    new_parent_scopes.push(current_scope.clone());

                    self.check_shadowing_in_statements(
                        body,
                        &mut loop_scope,
                        &new_parent_scopes,
                        file_id,
                        diagnostics,
                    );
                }
                _ => {}
            }
        }
    }

    fn check_all_paths_return(&self, statements: &[Statement], has_return: &mut bool) -> bool {
        if statements.is_empty() {
            return false;
        }

        for (i, statement) in statements.iter().enumerate() {
            match statement {
                Statement::ReturnStatement { .. } => {
                    *has_return = true;
                    return i == statements.len() - 1;
                }
                Statement::IfStatement {
                    then_block,
                    else_block,
                    ..
                } => {
                    let then_returns = self.check_all_paths_return(then_block, has_return);

                    let else_returns = if let Some(else_stmts) = else_block {
                        self.check_all_paths_return(else_stmts, has_return)
                    } else {
                        false
                    };

                    if then_returns && else_returns && i == statements.len() - 1 {
                        return true;
                    }
                }
                _ => {}
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::Literal;

    #[test]
    fn test_unused_variable() {
        let program = Program {
            statements: vec![
                Statement::VariableDeclaration {
                    name: "unused".to_string(),
                    value: Expression::Literal(Literal::Integer(10), 1, 1),
                    line: 1,
                    column: 1,
                },
                Statement::VariableDeclaration {
                    name: "used".to_string(),
                    value: Expression::Literal(Literal::Integer(20), 2, 1),
                    line: 2,
                    column: 1,
                },
                Statement::DisplayStatement {
                    value: Expression::Variable("used".to_string(), 3, 9),
                    line: 3,
                    column: 1,
                },
            ],
        };

        let mut analyzer = Analyzer::new();
        let mut reporter = DiagnosticReporter::new();
        let file_id = reporter.add_file("test.wfl", "");

        let diagnostics = analyzer.check_unused_variables(&program, file_id);

        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].message.contains("unused"));
        assert_eq!(diagnostics[0].code, "ANALYZE-UNUSED");
    }

    #[test]
    fn test_inconsistent_returns() {
        let program = Program {
            statements: vec![Statement::ActionDefinition {
                name: "inconsistent".to_string(),
                parameters: vec![],
                body: vec![Statement::IfStatement {
                    condition: Expression::Literal(Literal::Boolean(true), 2, 5),
                    then_block: vec![Statement::ReturnStatement {
                        value: Some(Expression::Literal(Literal::Integer(1), 3, 9)),
                        line: 3,
                        column: 5,
                    }],
                    else_block: Some(vec![Statement::DisplayStatement {
                        value: Expression::Literal(Literal::String("No return".to_string()), 5, 9),
                        line: 5,
                        column: 5,
                    }]),
                    line: 2,
                    column: 1,
                }],
                return_type: Some(Type::Number),
                line: 1,
                column: 1,
            }],
        };

        let mut analyzer = Analyzer::new();
        let mut reporter = DiagnosticReporter::new();
        let file_id = reporter.add_file("test.wfl", "");

        let diagnostics = analyzer.check_inconsistent_returns(&program, file_id);

        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].message.contains("inconsistent"));
        assert_eq!(diagnostics[0].code, "ANALYZE-RETURN");
    }
}
