use super::unary;
use crate::{
    codegen::{
        dialect::gpu::{ClampOperation, Item, Operation, Variable},
        Compiler,
    },
    element::WgpuElement,
    tensor::WgpuTensor,
    unary,
};

pub(crate) fn clamp<C: Compiler, E: WgpuElement, const D: usize>(
    input: WgpuTensor<E, D>,
    min_value: E,
    max_value: E,
) -> WgpuTensor<E, D> {
    unary!(
        operation: |elem| Operation::Clamp(ClampOperation {
            input: Variable::Input(0, Item::Scalar(elem)),
            min_value: Variable::Scalar(0, Item::Scalar(elem)),
            max_value: Variable::Scalar(1, Item::Scalar(elem)),
            out: Variable::Local(0, Item::Scalar(elem)),
        }),
        compiler: C,
        scalar 2
    );

    unary::<Ops<C, E>, OpsInplace<C, E>, E, D>(input, Some(&[min_value, max_value]), true)
}

#[cfg(test)]
mod tests {
    use crate::tests::{ReferenceBackend, TestBackend};
    use burn_tensor::{Distribution, Tensor};

    #[test]
    fn clamp_should_match_reference() {
        let input = Tensor::<TestBackend, 4>::random(
            [1, 5, 32, 32],
            Distribution::Default,
            &Default::default(),
        );
        let input_ref =
            Tensor::<ReferenceBackend, 4>::from_data(input.to_data(), &Default::default());

        let output = input.clamp(0.3, 0.7);

        output
            .into_data()
            .assert_approx_eq(&input_ref.clamp(0.3, 0.7).into_data(), 3);
    }
}
