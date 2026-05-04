pub fn is_table_array(table: mlua::Table) -> bool {
    if table.is_empty() {
        return table.metatable().is_none();
    };
    let mut j = 1;
    for _ in table.pairs::<mlua::Value, mlua::Value>() {
        if !table.contains_key(j).unwrap_or(false) {
            return false;
        }
        j += 1;
    }
    true
}
