pub type Hp = u32;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DamageType {
    Arrow,
    Magic,
    Sword,
    Ray,
}
