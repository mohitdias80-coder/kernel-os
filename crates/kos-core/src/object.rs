pub trait KernelObject {
    type Id;

    fn id(&self) -> Self::Id;
}

