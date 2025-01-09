use crate::prelude::*;
use proc_macro2::Span;

pub struct DefaultExpr {
    is_string: bool,
    expr: String,
}

impl ToTokens for DefaultExpr {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        // If this is a numeric or bool literal, just emit it
        if !self.is_string {
            let expr = &self.expr;
            tokens.extend(quote!(#expr));
        } else {
            // It's a string, so produce a valid SQL string literal.
            // e.g. if `expr` is "" (empty), you want `''`
            let sql_snippet = format!("'{}'", self.expr);
            let lit_str = syn::LitStr::new(&sql_snippet, proc_macro2::Span::call_site());
            tokens.extend(quote!(#lit_str));
        }
    }
}

impl Parse for DefaultExpr {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        use sql_from_models_parser::{dialect::*, parser::Parser, tokenizer::*};

        let span = Span::call_site();
        let mut is_string = false;

        // Parse one Rust literal: bool, int, float, or string
        let expr = match input.parse::<Lit>() {
            Ok(Lit::Bool(boolean)) => boolean.value().to_string(),
            Ok(Lit::Int(int))     => int.to_string(),
            Ok(Lit::Float(float)) => float.to_string(),
            Ok(Lit::Str(string))  => {
                is_string = true;
                string.value() // e.g. "" or "some text"
            }
            Ok(_) | Err(_) => {
                return Err(Error::new(
                    input.span(),
                    "Expected string, boolean, or numeric literal"
                ));
            }
        };

        // If the literal is an *empty string* `""`, convert it to `''` for the SQL parser
        let expr_for_parser = if is_string && expr.is_empty() {
            "''"
        } else {
            &expr
        };

        // Pass `expr_for_parser` to your SQL parser
        let mut lexer = Tokenizer::new(&GenericDialect {}, expr_for_parser);
        let tokens = lexer.tokenize().map_err(|err| {
            syn::Error::new(span, format!("Failed to tokenize default expression: {:?}", err))
        })?;

        Parser::new(tokens, &GenericDialect {})
            .parse_expr()
            .map_err(|err| {
                syn::Error::new(span, format!("Failed to parse default expression: {}", err))
            })?;

        // Return the original `expr` (which might be ""), along with `is_string`.
        Ok(DefaultExpr { is_string, expr })
    }
}
