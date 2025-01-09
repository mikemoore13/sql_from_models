use crate::prelude::*;
use proc_macro2::Span;

pub struct DefaultExpr {
    is_string: bool,
    expr: String,
}

impl ToTokens for DefaultExpr {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        if !self.is_string {
            // Numeric/bool literal like `42` or `true`: emit verbatim
            let expr = &self.expr;
            tokens.extend(quote!(#expr));
        } else {
            // String literal like "" or "hello"
            // Wrap the string in single quotes for SQL
            let sql_snippet = format!("'{}'", self.expr);
            // Make a *Rust* string literal from that snippet
            let lit_str = syn::LitStr::new(&sql_snippet, proc_macro2::Span::call_site());

            // Emit a valid Rust string literal (e.g. "'hello'")
            tokens.extend(quote!(#lit_str));
        }
    }
}

impl Parse for DefaultExpr {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        use sql_from_models_parser::{dialect::*, parser::Parser, tokenizer::*};

        let span = Span::call_site();
        let mut is_string = false;

        // Directly parse one literal (e.g. `0`, `"hello"`, `42.0`, etc.)
        let expr = match input.parse::<Lit>() {
            Ok(Lit::Bool(boolean)) => boolean.value().to_string(),
            Ok(Lit::Int(int))     => int.to_string(),
            Ok(Lit::Float(float)) => float.to_string(),
            Ok(Lit::Str(string))  => {
                is_string = true;
                string.value()
            }
            Ok(lit) => {
                return Err(Error::new(
                    lit.span(),
                    "Expected string, boolean, or numeric literal",
                ));
            }
            Err(err) => {
                return Err(Error::new(
                    err.span(),
                    "Expected string, boolean, or numeric literal",
                ));
            }
        };

        // Optionally verify that the literal is a valid SQL expression:
        let mut lexer = Tokenizer::new(&GenericDialect {}, &expr);
        let tokens = lexer.tokenize().map_err(|err| {
            syn::Error::new(span, format!("Failed to tokenize default expression: {:?}", err))
        })?;

        Parser::new(tokens, &GenericDialect {})
            .parse_expr()
            .map_err(|err| {
                syn::Error::new(span, format!("Failed to parse default expression: {}", err))
            })?;

        Ok(DefaultExpr { is_string, expr })
    }
}
