// Copyright 2026 Dora Lee
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use lsp_types::{Hover, HoverContents, MarkupContent, MarkupKind};

use crate::syntax::{self, InterfaceMember, Statement, StructBody};

use super::mojomast::MojomAst;

pub(crate) fn find_documented_name_range(ident: &str, ast: &MojomAst) -> Option<syntax::Range> {
    let mut path = Vec::new();
    for statement in &ast.mojom.stmts {
        if let Some(range) = find_statement_name_range(ident, ast, statement, &mut path) {
            return Some(range);
        }
    }
    None
}

pub(crate) fn find_documented_name_range_at_position(
    text: &str,
    position: &lsp_types::Position,
    ast: &MojomAst,
) -> Option<syntax::Range> {
    let offset = offset_from_position(text, position)?;
    for statement in &ast.mojom.stmts {
        if let Some(range) = find_statement_name_range_at_offset(offset, statement) {
            return Some(range);
        }
    }
    None
}

pub(crate) fn hover_from_syntax_range(text: &str, range: &syntax::Range) -> Option<Hover> {
    create_hover(extract_leading_documentation(text, range.start)?)
}

pub(crate) fn hover_from_lsp_range(text: &str, range: &lsp_types::Range) -> Option<Hover> {
    create_hover(extract_leading_documentation(
        text,
        offset_from_position(text, &range.start)?,
    )?)
}

fn create_hover(value: String) -> Option<Hover> {
    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value,
        }),
        range: None,
    })
}

fn find_statement_name_range(
    ident: &str,
    ast: &MojomAst,
    statement: &Statement,
    path: &mut Vec<String>,
) -> Option<syntax::Range> {
    match statement {
        Statement::Module(module) => match_name(ident, ast, &module.name, path),
        Statement::Import(_) => None,
        Statement::Interface(interface) => {
            if let Some(range) = match_name(ident, ast, &interface.name, path) {
                return Some(range);
            }

            path.push(ast.text(&interface.name).to_owned());
            for member in &interface.members {
                let range = match member {
                    InterfaceMember::Const(constant) => {
                        match_name(ident, ast, &constant.name, path)
                    }
                    InterfaceMember::Enum(enumeration) => {
                        find_enum_name_range(ident, ast, enumeration, path)
                    }
                    InterfaceMember::Method(method) => match_name(ident, ast, &method.name, path),
                };
                if range.is_some() {
                    path.pop();
                    return range;
                }
            }
            path.pop();
            None
        }
        Statement::Struct(structure) => {
            if let Some(range) = match_name(ident, ast, &structure.name, path) {
                return Some(range);
            }

            path.push(ast.text(&structure.name).to_owned());
            for member in &structure.members {
                let range = match member {
                    StructBody::Const(constant) => match_name(ident, ast, &constant.name, path),
                    StructBody::Enum(enumeration) => {
                        find_enum_name_range(ident, ast, enumeration, path)
                    }
                    StructBody::Field(field) => match_name(ident, ast, &field.name, path),
                };
                if range.is_some() {
                    path.pop();
                    return range;
                }
            }
            path.pop();
            None
        }
        Statement::Union(union) => {
            if let Some(range) = match_name(ident, ast, &union.name, path) {
                return Some(range);
            }

            path.push(ast.text(&union.name).to_owned());
            for field in &union.fields {
                if let Some(range) = match_name(ident, ast, &field.name, path) {
                    path.pop();
                    return Some(range);
                }
            }
            path.pop();
            None
        }
        Statement::Enum(enumeration) => find_enum_name_range(ident, ast, enumeration, path),
        Statement::Const(constant) => match_name(ident, ast, &constant.name, path),
    }
}

fn find_statement_name_range_at_offset(
    offset: usize,
    statement: &Statement,
) -> Option<syntax::Range> {
    match statement {
        Statement::Module(module) => match_offset(offset, &module.name),
        Statement::Import(_) => None,
        Statement::Interface(interface) => {
            if let Some(range) = match_offset(offset, &interface.name) {
                return Some(range);
            }

            for member in &interface.members {
                let range = match member {
                    InterfaceMember::Const(constant) => match_offset(offset, &constant.name),
                    InterfaceMember::Enum(enumeration) => {
                        find_enum_name_range_at_offset(offset, enumeration)
                    }
                    InterfaceMember::Method(method) => match_offset(offset, &method.name),
                };
                if range.is_some() {
                    return range;
                }
            }
            None
        }
        Statement::Struct(structure) => {
            if let Some(range) = match_offset(offset, &structure.name) {
                return Some(range);
            }

            for member in &structure.members {
                let range = match member {
                    StructBody::Const(constant) => match_offset(offset, &constant.name),
                    StructBody::Enum(enumeration) => {
                        find_enum_name_range_at_offset(offset, enumeration)
                    }
                    StructBody::Field(field) => match_offset(offset, &field.name),
                };
                if range.is_some() {
                    return range;
                }
            }
            None
        }
        Statement::Union(union) => {
            if let Some(range) = match_offset(offset, &union.name) {
                return Some(range);
            }

            for field in &union.fields {
                if let Some(range) = match_offset(offset, &field.name) {
                    return Some(range);
                }
            }
            None
        }
        Statement::Enum(enumeration) => find_enum_name_range_at_offset(offset, enumeration),
        Statement::Const(constant) => match_offset(offset, &constant.name),
    }
}

fn find_enum_name_range(
    ident: &str,
    ast: &MojomAst,
    enumeration: &syntax::Enum,
    path: &mut Vec<String>,
) -> Option<syntax::Range> {
    if let Some(range) = match_name(ident, ast, &enumeration.name, path) {
        return Some(range);
    }

    path.push(ast.text(&enumeration.name).to_owned());
    for value in &enumeration.values {
        if let Some(range) = match_name(ident, ast, &value.name, path) {
            path.pop();
            return Some(range);
        }
    }
    path.pop();
    None
}

fn find_enum_name_range_at_offset(
    offset: usize,
    enumeration: &syntax::Enum,
) -> Option<syntax::Range> {
    if let Some(range) = match_offset(offset, &enumeration.name) {
        return Some(range);
    }

    for value in &enumeration.values {
        if let Some(range) = match_offset(offset, &value.name) {
            return Some(range);
        }
    }
    None
}

fn match_name(
    ident: &str,
    ast: &MojomAst,
    name_range: &syntax::Range,
    path: &[String],
) -> Option<syntax::Range> {
    let name = ast.text(name_range);
    let qualified_name = if path.is_empty() {
        name.to_owned()
    } else {
        format!("{}.{}", path.join("."), name)
    };

    if ident == name || ident == qualified_name {
        Some(name_range.clone())
    } else {
        None
    }
}

fn match_offset(offset: usize, name_range: &syntax::Range) -> Option<syntax::Range> {
    if name_range.start <= offset && offset <= name_range.end {
        Some(name_range.clone())
    } else {
        None
    }
}

fn offset_from_position(text: &str, position: &lsp_types::Position) -> Option<usize> {
    let target_line = position.line as usize;
    let target_col = position.character as usize;
    let mut offset = 0;
    for (line, content) in text.split('\n').enumerate() {
        if line == target_line {
            return Some(offset + target_col.min(content.len()));
        }
        offset += content.len() + 1;
    }
    None
}

fn extract_leading_documentation(text: &str, declaration_offset: usize) -> Option<String> {
    let lines = line_ranges(text);
    let mut line = line_index_for_offset(&lines, declaration_offset)?;
    if line == 0 {
        return None;
    }

    line -= 1;
    while is_attribute_line(line_text(text, lines[line])) {
        if line == 0 {
            return None;
        }
        line -= 1;
    }

    let trimmed = line_text(text, lines[line]).trim();
    if trimmed.starts_with("//") {
        return collect_line_comments(text, &lines, line);
    }
    if trimmed.ends_with("*/") {
        return collect_block_comment(text, &lines, line);
    }
    None
}

fn line_ranges(text: &str) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let mut start = 0;
    for line in text.split_inclusive('\n') {
        let end = start + line.trim_end_matches(&['\r', '\n']).len();
        ranges.push((start, end));
        start += line.len();
    }
    if text.is_empty() {
        ranges.push((start, start));
    }
    ranges
}

fn line_index_for_offset(lines: &[(usize, usize)], offset: usize) -> Option<usize> {
    lines
        .iter()
        .position(|(start, end)| *start <= offset && offset <= *end)
}

fn line_text(text: &str, range: (usize, usize)) -> &str {
    &text[range.0..range.1]
}

fn is_attribute_line(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with('[') && trimmed.ends_with(']')
}

fn collect_line_comments(text: &str, lines: &[(usize, usize)], mut line: usize) -> Option<String> {
    let mut docs = Vec::new();
    loop {
        let content = line_text(text, lines[line]).trim();
        if !content.starts_with("//") {
            break;
        }
        docs.push(content.trim_start_matches("//").trim_start().to_owned());
        if line == 0 {
            break;
        }
        line -= 1;
    }
    docs.reverse();
    normalize_doc_lines(docs)
}

fn collect_block_comment(text: &str, lines: &[(usize, usize)], mut line: usize) -> Option<String> {
    let mut docs = Vec::new();
    loop {
        let content = line_text(text, lines[line]);
        docs.push(content.to_owned());
        if content.contains("/*") || line == 0 {
            break;
        }
        line -= 1;
    }
    docs.reverse();

    let joined = docs.join("\n");
    let start = joined.find("/*").map(|idx| idx + 2).unwrap_or(0);
    let end = joined.rfind("*/").unwrap_or(joined.len());
    let lines = joined[start..end]
        .lines()
        .map(|line| line.trim().trim_start_matches('*').trim_start().to_owned())
        .collect();
    normalize_doc_lines(lines)
}

fn normalize_doc_lines(mut lines: Vec<String>) -> Option<String> {
    while lines.first().map(|line| line.trim().is_empty()).unwrap_or(false) {
        lines.remove(0);
    }
    while lines.last().map(|line| line.trim().is_empty()).unwrap_or(false) {
        lines.pop();
    }

    if lines.is_empty() {
        None
    } else {
        Some(lines.join("\n"))
    }
}
