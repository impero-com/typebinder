use std::collections::HashMap;

use ts_json_subset::{
    ident::TSIdent,
    types::{ExtendsConstraint, TsType},
};

#[derive(Debug, Default)]
pub struct GenericConstraints(HashMap<TSIdent, ExtendsConstraint>);

impl GenericConstraints {
    pub fn add_extends_constraint(&mut self, ident: TSIdent, ts_type: TsType) {
        let constraints = self.0.entry(ident).or_default();
        constraints.types.push(ts_type);
    }

    pub fn merge(&mut self, other_constraints: GenericConstraints) {
        other_constraints
            .0
            .iter()
            .for_each(|(ts_ident, constraint)| {
                let entry = self.0.entry(ts_ident.clone()).or_default();
                entry.merge(constraint);
            });
    }
}
