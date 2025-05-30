pub(crate) mod adding_field_with_default;
pub(crate) mod adding_foreign_key_constraint;
pub(crate) mod adding_not_null_field;
pub(crate) mod adding_primary_key_constraint;
pub(crate) mod adding_required_field;
pub(crate) mod ban_alter_domain_with_add_constraint;
pub(crate) mod ban_char_field;
pub(crate) mod ban_concurrent_index_creation_in_transaction;
pub(crate) mod ban_create_domain_with_constraint;
pub(crate) mod ban_drop_column;
pub(crate) mod ban_drop_database;
pub(crate) mod ban_drop_not_null;
pub(crate) mod ban_drop_table;
pub(crate) mod changing_column_type;
pub(crate) mod constraint_missing_not_valid;
pub(crate) mod disallow_unique_constraint;
pub(crate) mod prefer_big_int;
pub(crate) mod prefer_bigint_over_int;
pub(crate) mod prefer_bigint_over_smallint;
pub(crate) mod prefer_identity;
pub(crate) mod prefer_robust_stmts;
pub(crate) mod prefer_text_field;
pub(crate) mod prefer_timestamptz;
pub(crate) mod renaming_column;
pub(crate) mod renaming_table;
pub(crate) mod require_concurrent_index_creation;
pub(crate) mod require_concurrent_index_deletion;
// xtask:new-lint:mod-decl

pub(crate) use adding_field_with_default::adding_field_with_default;
pub(crate) use adding_foreign_key_constraint::adding_foreign_key_constraint;
pub(crate) use adding_not_null_field::adding_not_null_field;
pub(crate) use adding_primary_key_constraint::adding_primary_key_constraint;
pub(crate) use adding_required_field::adding_required_field;
pub(crate) use ban_alter_domain_with_add_constraint::ban_alter_domain_with_add_constraint;
pub(crate) use ban_char_field::ban_char_field;
pub(crate) use ban_concurrent_index_creation_in_transaction::ban_concurrent_index_creation_in_transaction;
pub(crate) use ban_create_domain_with_constraint::ban_create_domain_with_constraint;
pub(crate) use ban_drop_column::ban_drop_column;
pub(crate) use ban_drop_database::ban_drop_database;
pub(crate) use ban_drop_not_null::ban_drop_not_null;
pub(crate) use ban_drop_table::ban_drop_table;
pub(crate) use changing_column_type::changing_column_type;
pub(crate) use constraint_missing_not_valid::constraint_missing_not_valid;
pub(crate) use disallow_unique_constraint::disallow_unique_constraint;
pub(crate) use prefer_big_int::prefer_big_int;
pub(crate) use prefer_bigint_over_int::prefer_bigint_over_int;
pub(crate) use prefer_bigint_over_smallint::prefer_bigint_over_smallint;
pub(crate) use prefer_identity::prefer_identity;
pub(crate) use prefer_robust_stmts::prefer_robust_stmts;
pub(crate) use prefer_text_field::prefer_text_field;
pub(crate) use prefer_timestamptz::prefer_timestamptz;
pub(crate) use renaming_column::renaming_column;
pub(crate) use renaming_table::renaming_table;
pub(crate) use require_concurrent_index_creation::require_concurrent_index_creation;
pub(crate) use require_concurrent_index_deletion::require_concurrent_index_deletion;
// xtask:new-lint:export
