use syn::{Attribute, Error, Path};

use super::attributes::item;

pub(crate) fn get(attrs: &[Attribute]) -> Result<Path, Error> {
    let path = item::get_crate(attrs)?;
    match path {
        Some(path) => Ok(path),
        None => Ok(syn::parse_quote!(borsh)),
    }
}
