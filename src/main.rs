use tweenygrad::ArgType;
use tweenygrad::DType;
use tweenygrad::DTypeVal;
use tweenygrad::Tensor; // or struct, etc.

fn main() {
    let mut x = Tensor::new(ArgType::Val(DTypeVal::Int32(3)), DType::Int32);
    let y = Tensor::new(ArgType::Val(DTypeVal::Int32(6)), DType::Int32);

    x.assign(y);

    println!("{:?}", x)
}
