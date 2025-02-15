use crate::utils::CastErrorToAnyhow;
use crate::{FromCell, ToCell};
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
