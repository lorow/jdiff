pub trait Model {
    type ModelActions;
    fn update(&self, action: Self::ModelActions) -> Option<Self::ModelActions>;
}
