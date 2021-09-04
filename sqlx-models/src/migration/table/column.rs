use crate::prelude::*;

use ast::ColumnDef;
use proc_macro::token_stream;
use sqlx_models_parser::dialect::GenericDialect;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Column {
    pub name: Ident,
    pub r#type: DataType,
    pub options: Vec<ColumnOptionDef>,
}

impl super::get_changes::Name for Column {
    fn name(&self) -> &Ident {
        &self.name
    }
}

impl Column {
    pub fn new(name: &str, r#type: DataType, options: ColumnOptionDef) -> Self {
        Column {
            name: Ident::new(name.to_lowercase()),
            r#type,
            options: vec![options],
        }
    }

    pub fn new_with_default(name: &str, r#type: DataType, op: ColumnOptionDef, def: &str) -> Self {
        let dialect = GenericDialect {};
        let tokens = tokenizer::Tokenizer::new(dialect, def);
        let parser = Parser::new(tokens, dialect);
        let expr = parser.parse_expr(&mut self);
        Column {
            name: Ident::new(name.to_lowercase()),
            r#type,
            options: vec![op, ColumnOptionDef::Default(expr)],
        }
    }
}

impl From<ColumnDef> for Column {
    fn from(col: ColumnDef) -> Self {
        Column {
            name: col.name,
            options: col.options,
            r#type: col.data_type,
        }
    }
}

impl From<Column> for ColumnDef {
    fn from(col: Column) -> Self {
        ColumnDef {
            name: col.name,
            options: col.options,
            data_type: col.r#type,
            collation: None,
        }
    }
}