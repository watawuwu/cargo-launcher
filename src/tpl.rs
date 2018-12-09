use tera::{Context, Tera};

use crate::error::Result;

pub type Param = Context;

pub fn render(tpl: &str, params: &Param) -> Result<String> {
    // @TODO error sync...
    let info_plist = Tera::one_off(tpl, &params, true).unwrap();
    Ok(info_plist)
}
