mod highlight_names;

use highlight_names::{CLASS_NAMES, HIGHLIGHT_NAMES, HTML_ATTRS};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::sync::LazyLock;
use tree_sitter_highlight::{
    Highlight, HighlightConfiguration, HighlightEvent, Highlighter, HtmlRenderer,
};

#[napi]
pub enum Language {
    JS,
    JSX,
    TS,
    TSX,
    JSON,
    YAML,
    CSS,
    HTML,
    Regex,
    JsDoc,
    C,
    Bash,
    Rust,
}

macro_rules! language {
    ($mod: ident, $name: literal, $highlights: ident) => {{
        let mut config =
            HighlightConfiguration::new($mod::LANGUAGE.into(), $name, $mod::$highlights, "", "")
                .unwrap();
        config.configure(HIGHLIGHT_NAMES);
        config
    }};
    ($mod: ident, $name: literal, $highlights: ident, $injections: ident) => {{
        let mut config = HighlightConfiguration::new(
            $mod::LANGUAGE.into(),
            $name,
            $mod::$highlights,
            $mod::$injections,
            "",
        )
        .unwrap();
        config.configure(HIGHLIGHT_NAMES);
        config
    }};
}

static JS_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut config = HighlightConfiguration::new(
        tree_sitter_javascript::LANGUAGE.into(),
        "javascript",
        tree_sitter_javascript::HIGHLIGHT_QUERY,
        tree_sitter_javascript::INJECTIONS_QUERY,
        tree_sitter_javascript::LOCALS_QUERY,
    )
    .unwrap();
    config.configure(HIGHLIGHT_NAMES);
    config
});

static JSX_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut highlights = tree_sitter_javascript::JSX_HIGHLIGHT_QUERY.to_owned();
    highlights.push_str(tree_sitter_javascript::HIGHLIGHT_QUERY);

    let mut config = HighlightConfiguration::new(
        tree_sitter_javascript::LANGUAGE.into(),
        "jsx",
        &highlights,
        tree_sitter_javascript::INJECTIONS_QUERY,
        tree_sitter_javascript::LOCALS_QUERY,
    )
    .unwrap();

    config.configure(HIGHLIGHT_NAMES);
    config
});

static TS_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut highlights = tree_sitter_typescript::HIGHLIGHTS_QUERY.to_owned();
    highlights.push_str(tree_sitter_javascript::HIGHLIGHT_QUERY);

    let mut locals = tree_sitter_typescript::LOCALS_QUERY.to_owned();
    locals.push_str(tree_sitter_javascript::LOCALS_QUERY);

    let mut config = HighlightConfiguration::new(
        tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
        "typescript",
        &highlights,
        tree_sitter_javascript::INJECTIONS_QUERY,
        &locals,
    )
    .unwrap();

    config.configure(HIGHLIGHT_NAMES);
    config
});

static TSX_CONFIG: LazyLock<HighlightConfiguration> = LazyLock::new(|| {
    let mut highlights = tree_sitter_javascript::JSX_HIGHLIGHT_QUERY.to_owned();
    highlights.push_str(tree_sitter_typescript::HIGHLIGHTS_QUERY);
    highlights.push_str(tree_sitter_javascript::HIGHLIGHT_QUERY);

    let mut locals = tree_sitter_typescript::LOCALS_QUERY.to_owned();
    locals.push_str(tree_sitter_javascript::LOCALS_QUERY);

    let mut config = HighlightConfiguration::new(
        tree_sitter_typescript::LANGUAGE_TSX.into(),
        "tsx",
        &highlights,
        tree_sitter_javascript::INJECTIONS_QUERY,
        &locals,
    )
    .unwrap();

    config.configure(HIGHLIGHT_NAMES);
    config
});

static JSDOC_CONFIG: LazyLock<HighlightConfiguration> =
    LazyLock::new(|| language!(tree_sitter_jsdoc, "jsdoc", HIGHLIGHTS_QUERY));

static JSON_CONFIG: LazyLock<HighlightConfiguration> =
    LazyLock::new(|| language!(tree_sitter_json, "json", HIGHLIGHTS_QUERY));

static YAML_CONFIG: LazyLock<HighlightConfiguration> =
    LazyLock::new(|| language!(tree_sitter_yaml, "yaml", HIGHLIGHTS_QUERY));

static CSS_CONFIG: LazyLock<HighlightConfiguration> =
    LazyLock::new(|| language!(tree_sitter_css, "css", HIGHLIGHTS_QUERY));

static HTML_CONFIG: LazyLock<HighlightConfiguration> =
    LazyLock::new(|| language!(tree_sitter_html, "html", INJECTIONS_QUERY));

static REGEX_CONFIG: LazyLock<HighlightConfiguration> =
    LazyLock::new(|| language!(tree_sitter_regex, "regex", HIGHLIGHTS_QUERY));

static C_CONFIG: LazyLock<HighlightConfiguration> =
    LazyLock::new(|| language!(tree_sitter_c, "c", HIGHLIGHT_QUERY));

static BASH_CONFIG: LazyLock<HighlightConfiguration> =
    LazyLock::new(|| language!(tree_sitter_bash, "bash", HIGHLIGHT_QUERY));

static RUST_CONFIG: LazyLock<HighlightConfiguration> =
    LazyLock::new(|| language!(tree_sitter_rust, "rust", HIGHLIGHTS_QUERY));

impl Language {
    fn highlight_config(&self) -> &'static HighlightConfiguration {
        match self {
            Language::JS => &JS_CONFIG,
            Language::JSX => &JSX_CONFIG,
            Language::TS => &TS_CONFIG,
            Language::TSX => &TSX_CONFIG,
            Language::JSON => &JSON_CONFIG,
            Language::YAML => &YAML_CONFIG,
            Language::CSS => &CSS_CONFIG,
            Language::HTML => &HTML_CONFIG,
            Language::Regex => &REGEX_CONFIG,
            Language::JsDoc => &JSDOC_CONFIG,
            Language::C => &C_CONFIG,
            Language::Bash => &BASH_CONFIG,
            Language::Rust => &RUST_CONFIG,
        }
    }

    fn from_name(name: &str) -> Option<Language> {
        Some(match name {
            "js" | "javascript" => Language::JS,
            "jsx" => Language::JSX,
            "ts" | "typescript" => Language::TS,
            "tsx" => Language::TSX,
            "json" => Language::JSON,
            "yaml" => Language::YAML,
            "css" => Language::CSS,
            "html" => Language::HTML,
            "regex" => Language::Regex,
            "jsdoc" => Language::JsDoc,
            "c" => Language::C,
            "bash" => Language::Bash,
            "sh" => Language::Bash,
            "rust" => Language::Rust,
            "rs" => Language::Rust,
            _ => return None,
        })
    }
}

#[napi]
pub fn from_alias(name: String) -> Option<Language> {
    Language::from_name(&name)
}

#[napi]
pub fn highlight(code: String, language: Language) -> String {
    let config = language.highlight_config();
    let mut highlighter = Highlighter::new();
    let highlights = highlighter
        .highlight(config, code.as_bytes(), None, None, |lang| {
            Language::from_name(lang).map(|l| l.highlight_config())
        })
        .unwrap();

    let mut renderer = HtmlRenderer::new();
    renderer
        .render(highlights, code.as_bytes(), &|highlight, res| {
            res.extend_from_slice(HTML_ATTRS[highlight.0].as_bytes())
        })
        .unwrap();
    unsafe { String::from_utf8_unchecked(renderer.html) }
}

#[derive(Debug)]
#[napi(object)]
pub struct HastProperties {
    pub class_name: String,
}

#[derive(Debug)]
#[napi(object)]
pub struct HastNode {
    #[napi(js_name = "type")]
    pub kind: String,
    pub tag_name: String,
    pub properties: HastProperties,
    pub children: Vec<Either<HastNode, HastTextNode>>,
}

#[derive(Debug)]
#[napi(object)]
pub struct HastTextNode {
    #[napi(js_name = "type")]
    pub kind: String,
    pub value: String,
}

#[napi]
pub fn highlight_hast(code: String, language: Language) -> HastNode {
    let config = language.highlight_config();
    let mut highlighter = Highlighter::new();
    let highlights = highlighter
        .highlight(config, code.as_bytes(), None, None, |lang| {
            Language::from_name(lang).map(|l| l.highlight_config())
        })
        .unwrap();

    let mut stack = Vec::new();
    stack.push(HastNode {
        kind: "element".into(),
        tag_name: "span".into(),
        properties: HastProperties {
            class_name: "source".into(),
        },
        children: Vec::new(),
    });

    for event in highlights {
        match event.unwrap() {
            HighlightEvent::HighlightStart(highlight) => {
                let node = HastNode {
                    kind: "element".into(),
                    tag_name: "span".into(),
                    properties: HastProperties {
                        class_name: CLASS_NAMES[highlight.0].to_owned(),
                    },
                    children: Vec::new(),
                };
                stack.push(node);
            }
            HighlightEvent::Source { start, end } => {
                let slice = &code[start..end];
                let parent = stack.last_mut().unwrap();
                if let Some(Either::B(text_node)) = parent.children.last_mut() {
                    text_node.value.push_str(slice);
                } else {
                    let text_node = HastTextNode {
                        kind: "text".into(),
                        value: slice.into(),
                    };
                    parent.children.push(Either::B(text_node));
                }
            }
            HighlightEvent::HighlightEnd => {
                let node = stack.pop().unwrap();
                let parent = stack.last_mut().unwrap();
                parent.children.push(Either::A(node));
            }
        }
    }

    stack.pop().unwrap()
}

// https://github.com/shikijs/shiki/tree/main/packages/types/src/tokens.ts

#[derive(Debug)]
#[napi(object)]
pub struct ThemedToken {
    pub content: String,
    pub offset: u32,
    pub html_attrs: Option<ClassAttr>,
}

#[derive(Debug)]
#[napi(object)]
pub struct TokensResult {
    pub tokens: Vec<Vec<ThemedToken>>,
}

#[derive(Debug)]
#[napi(object)]
pub struct ClassAttr {
    pub class: String,
}

#[napi]
pub fn highlight_tokens(code: String, language: Language) -> TokensResult {
    let config = language.highlight_config();
    let mut highlighter = Highlighter::new();
    let code_bytes = code.as_bytes();

    let highlights_events = highlighter
        .highlight(config, code_bytes, None, None, |lang| {
            Language::from_name(lang).map(|l| l.highlight_config())
        })
        .unwrap();

    let mut lines: Vec<Vec<ThemedToken>> = Vec::new();
    let mut current_line: Vec<ThemedToken> = Vec::new();
    let mut highlight_stack: Vec<Highlight> = Vec::new();

    let mut current_utf16_offset: u32 = 0;

    for event in highlights_events {
        match event.unwrap() {
            HighlightEvent::HighlightStart(h) => {
                highlight_stack.push(h);
            }
            HighlightEvent::HighlightEnd => {
                highlight_stack.pop();
            }
            HighlightEvent::Source { start, end } => {
                let content_bytes = &code_bytes[start..end];
                let content_str = String::from_utf8_lossy(content_bytes);

                let mut last_pos = 0;
                for (i, c) in content_str.char_indices() {
                    if c == '\n' {
                        // 处理换行符之前的文本
                        if i > last_pos {
                            let text = &content_str[last_pos..i];
                            let utf16_len = text.encode_utf16().count() as u32;

                            current_line.push(create_token(
                                text.to_string(),
                                current_utf16_offset,
                                &highlight_stack,
                            ));
                            current_utf16_offset += utf16_len;
                        }

                        // 换行逻辑：结束当前行，开启新行
                        // 注意：'\n' 占用 1 个 UTF-16 单元
                        lines.push(std::mem::take(&mut current_line));
                        current_utf16_offset += 1;
                        last_pos = i + c.len_utf8();
                    }
                }

                // 处理剩余文本
                if last_pos < content_str.len() {
                    let text = &content_str[last_pos..];
                    let utf16_len = text.encode_utf16().count() as u32;

                    current_line.push(create_token(
                        text.to_string(),
                        current_utf16_offset,
                        &highlight_stack,
                    ));
                    current_utf16_offset += utf16_len;
                }
            }
        }
    }

    if !current_line.is_empty() || lines.is_empty() {
        lines.push(current_line);
    }

    TokensResult { tokens: lines }
}

fn create_token(content: String, offset: u32, stack: &[Highlight]) -> ThemedToken {
    let html_attrs: Option<ClassAttr> = stack.last().map(|h| ClassAttr {
        class: CLASS_NAMES[h.0].to_owned(),
    });

    ThemedToken {
        content,
        offset,
        html_attrs,
    }
}
