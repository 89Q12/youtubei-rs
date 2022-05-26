use super::category::CategoryTypes;

pub struct ChannelTab{
    pub title: String,
    pub selected: bool,
    pub content: Vec<CategoryTypes>
}