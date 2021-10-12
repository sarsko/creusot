use creusot_contracts_proc::*;

pub trait Model {
    type ModelTy;
    #[logic_rust]
    fn model(self) -> Self::ModelTy;
}

impl<T: Model> Model for &T {
    type ModelTy = T::ModelTy;
    #[logic_rust]
    fn model(self) -> Self::ModelTy {
        (*self).model()
    }
}

impl<T: Model> Model for &mut T {
    type ModelTy = T::ModelTy;
    #[logic_rust]
    fn model(self) -> Self::ModelTy {
        (*self).model()
    }
}