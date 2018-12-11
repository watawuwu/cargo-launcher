use tera::{Context, Tera};

use crate::error::Result;

pub type Param = Context;

pub fn render(tpl: &str, params: &Param) -> Result<String> {
    // @TODO error sync...
    let info_plist = Tera::one_off(tpl, &params, true).unwrap();
    Ok(info_plist)
}

#[cfg(test)]
mod tests {

    use crate::tpl::*;

    #[test]
    fn render_bore_ok() {
        let args = "name: {{name}}";
        let expected = "name: watawuwu";
        let mut params = Param::new();
        params.insert("name", "watawuwu");
        let actual = render(args, &params).unwrap();
        assert_eq!(expected, actual);
    }
}
