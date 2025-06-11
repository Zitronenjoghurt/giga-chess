use crate::engine::attack_table::AttackTable;

pub mod attack_table;
pub mod bit_board;
pub mod square;

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
