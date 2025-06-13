use crate::engine::attack_table::AttackTable;

pub mod attack_table;
pub mod magic_numbers;

pub struct Engine {
    pub attack_table: AttackTable,
}

impl Engine {
    pub fn initialize() -> Self {
        Self {
            attack_table: AttackTable::build(),
        }
    }
}
