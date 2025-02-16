use crate::utils::CastErrorToAnyhow;
use crate::{FromCell, ToCell};
use std::fmt::Display;
use tonlib_core::cell::{CellBuilder, CellParser};

#[derive(Debug, PartialEq, Default)]
pub struct Comment(String);

impl From<String> for Comment {
    fn from(value: String) -> Self {
        Comment(value)
    }
}

impl From<Comment> for String {
    fn from(value: Comment) -> Self {
        value.0
    }
}

impl Display for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ToCell for Comment {
    fn store<'a>(&self, builder: &'a mut CellBuilder) -> anyhow::Result<&'a mut CellBuilder> {
        builder.store_u32(32, 0)?;
        builder.store_string(self.0.as_str()).map_err_to_anyhow()
    }
}

impl FromCell for Comment {
    fn load(parser: &mut CellParser) -> anyhow::Result<Self> {
        parser.skip_bits(32)?;
        String::load(parser).map(Self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        assert_eq!(
            Comment::from("Hello world".to_string()),
            Comment("Hello world".to_string()),
        );
    }

    #[test]
    fn test_into() {
        assert_eq!(
            <Comment as Into<String>>::into(Comment("Hello world".to_string())),
            "Hello world".to_string()
        );
    }

    #[test]
    fn test_default() {
        assert_eq!(Comment::default(), Comment("".to_string()));
    }

    #[test]
    fn test_from_cell() {
        let cell = CellBuilder::new()
            .store_u32(32, 0)
            .unwrap()
            .store_slice("Hello world".as_bytes())
            .unwrap()
            .build()
            .unwrap();

        assert_eq!(
            Comment::from_cell(cell).unwrap(),
            Comment("Hello world".to_string())
        );
    }

    #[test]
    fn test_to_cell() {
        assert_eq!(
            Comment("Hello world".to_string()).to_cell().unwrap(),
            CellBuilder::new()
                .store_u32(32, 0)
                .unwrap()
                .store_slice("Hello world".as_bytes())
                .unwrap()
                .build()
                .unwrap(),
        );
    }

    #[test]
    fn test_from_to_cell() {
        let first_iter = Comment("Hello world".to_string());
        let cell = first_iter.to_cell().unwrap();
        let second_iter = Comment::from_cell(cell).unwrap();

        assert_eq!(first_iter, second_iter);
    }

    #[test]
    fn test_fmt() {
        let value = Comment("Hello world".to_string());
        assert_eq!("Comment(\"Hello world\")", format!("{:?}", value));
        assert_eq!("Hello world", format!("{}", value));
    }
}
