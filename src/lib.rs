use sqlite_loadable::{api, define_scalar_function, prelude::*, Result};
use xid::new;

pub fn xid(context: *mut sqlite3_context, _: &[*mut sqlite3_value]) -> Result<()> {
	let id = new();
	api::result_text(context, id.to_string())?;

	Ok(())
}

#[sqlite_entrypoint]
pub fn sqlite3_extension_init(database: *mut sqlite3) -> Result<()> {
	define_scalar_function(database, "xid", 0, xid, FunctionFlags::UTF8)?;
	Ok(())
}
