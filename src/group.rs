use std::env;

use proc_macro::{
    Delimiter,
    Group,
    Ident,
    Literal,
    Punct,
    Spacing,
    Span,
    TokenStream,
    TokenTree,
};

macro_rules! fine {
    ($e:expr) => {
        match $e {
            Ok(o) => o,
            Err(e) => return e,
        }
    };
}

enum RunStrategy {
    NotDisabled,
    OnlyEnabled,
}

struct GroupConf {
    strategy: RunStrategy,
    disabled: Vec<String>,
    enabled: Vec<String>,
}

impl GroupConf {
    pub fn parse() -> Result<Self, TokenStream> {
        fn split_and_parse(s: String) -> Vec<String> {
            s.split(' ').map(String::from).collect()
        }

        let strategy_str = env::var("GROUP_RUN_STRATEGY")
            .unwrap_or_else(|_| String::from("not_disabled"));
        let strategy = match strategy_str.as_str() {
            "not_disabled" => RunStrategy::NotDisabled,
            "only_enabled" => RunStrategy::OnlyEnabled,
            _ => {
                return Err(compile_err(
                    "GROUP_RUN_STRATEGY must be either only_enabled or \
                     not_disabled",
                ))
            }
        };
        let disabled = env::var("GROUP_DISABLE")
            .map(split_and_parse)
            .unwrap_or(Vec::new());
        let enabled = env::var("GROUP_ENABLE")
            .map(split_and_parse)
            .unwrap_or(Vec::new());

        Ok(Self {
            strategy,
            disabled,
            enabled,
        })
    }
}

fn compile_err(message: &str) -> TokenStream {
    [
        TokenTree::Ident(Ident::new("compile_error", Span::mixed_site())),
        TokenTree::Punct(Punct::new('!', Spacing::Alone)),
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            [TokenTree::Literal(Literal::string(message))]
                .into_iter()
                .collect(),
        )),
    ]
    .into_iter()
    .collect()
}

fn parse_groups(args: TokenStream) -> Result<Vec<String>, TokenStream> {
    let mut need_punct = false;
    let mut groups = Vec::new();
    for token in args {
        match token {
            TokenTree::Ident(ident) if !need_punct => {
                groups.push(ident.to_string());
            }

            TokenTree::Punct(punct)
                if need_punct && punct.as_char() == ',' => {}
            TokenTree::Ident(..) | TokenTree::Punct(..) if need_punct => {
                return Err(compile_err("Comma expected"))
            }
            _ => return Err(compile_err("Ident expected")),
        }

        need_punct = !need_punct;
    }

    Ok(groups)
}

pub fn group_impl(args: TokenStream, stream: TokenStream) -> TokenStream {
    let groups = fine! { parse_groups(args) };
    let conf = fine! { GroupConf::parse() };

    let scheduled = match conf.strategy {
        RunStrategy::NotDisabled => {
            !groups.iter().any(|g| conf.disabled.contains(g))
        }

        RunStrategy::OnlyEnabled => {
            conf.enabled
                .iter()
                .filter(|g| groups.contains(g))
                .count()
                == groups.len()
        }
    };

    if scheduled {
        stream
    } else {
        let ts: [TokenTree; 0] = [];
        ts.into_iter().collect()
    }
}
