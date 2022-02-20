pub(crate) use application::*;
pub(crate) use challenges::*;
pub(crate) use server::*;

mod application;
mod challenges;
mod server;

pub(crate) trait Builder {
    type Target;

    fn build(self) -> Self::Target;
}

pub(crate) trait OptionalBuilder {
    type Builder: Builder;

    fn build(self) -> Option<<Self::Builder as Builder>::Target>;
}

impl<B: Builder> OptionalBuilder for Option<B> {
    type Builder = B;

    fn build(self) -> Option<<Self::Builder as Builder>::Target> {
        self.map(Builder::build)
    }
}

impl<B: Builder> OptionalBuilder for B {
    type Builder = B;

    fn build(self) -> Option<<Self::Builder as Builder>::Target> {
        Some(self.build())
    }
}
