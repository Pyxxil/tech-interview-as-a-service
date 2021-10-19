pub(crate) trait Algorithm {
    type Input;
    type Output;
    type Step;
    type Action<'a>;

    fn step(&self, step: Self::Action<'_>) -> Self::Step;

    fn run(self, input: Self::Input) -> Self::Output;
}
