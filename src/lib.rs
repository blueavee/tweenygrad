#[derive(Clone, Debug)]
pub enum DType {
    Float32,
    Int32,
    Bool,
}

#[derive(Clone, Debug)]
pub enum DTypeVal {
    Float32(f32),
    Int32(i32),
    Bool(bool),
}

#[derive(Clone, Debug)]
pub enum ArgType {
    Val(DTypeVal),
    List(Vec<DTypeVal>),
}

#[derive(Clone, Debug)]
enum Ops {
    ASSIGN,
    STORE,
    ADD,
    CONST,
}

#[derive(Debug)]
pub struct Tensor {
    data: Option<Vec<DTypeVal>>,
    dtype: DType,
    // shape: Vec<u32>,
    // strides: Vec<u32>,
    uop: UOp,
}

impl Tensor {
    pub fn new(
        data: ArgType,
        dtype: DType,
        // shape: Vec<u32>
    ) -> Self {
        return Tensor {
            data: None,
            dtype: dtype.clone(),
            // shape,
            uop: UOp {
                op: Ops::CONST,
                dtype: dtype.clone(),
                src: vec![],
                arg: Some(data),
            },
        };
    }

    pub fn assign(&mut self, x: Tensor) -> &Tensor {
        self.uop = self.uop.assign(x.uop);
        return self;
    }
}

#[derive(Clone, Debug)]
pub struct UOp {
    op: Ops,
    dtype: DType,
    src: Vec<UOp>,
    arg: Option<ArgType>,
}

impl UOp {
    fn assign(&self, x: UOp) -> UOp {
        return UOp {
            op: Ops::ASSIGN,
            dtype: self.dtype.clone(),
            src: vec![self.clone(), x],
            arg: None,
        };
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    println!("Hello World");
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
