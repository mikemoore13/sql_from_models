use crate::prelude::*;
use proc_macro2::Span;

pub struct DefaultExpr {
    is_string: bool,
    expr: String,
}

impl ToTokens for DefaultExpr {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let expr = &self.expr;
        if !self.is_string {
            tokens.extend(quote!(#expr));
        } else {
            let expr = format!("{:?}", self.expr);
            let len = expr.chars().count();
            let mut out: String = "'".into();
            for char in expr.chars().skip(1).take(len - 2) {
                out.push(char);
            }
            out.push('\'');
            tokens.extend(quote!(#out))
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
