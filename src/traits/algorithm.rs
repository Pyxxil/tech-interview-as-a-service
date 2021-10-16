pub(crate) trait Algorithm {
    type Input;
    type Output;

    fn step(&self);

    fn run(&self, input: Self::Input) -> Self::Output;
}
