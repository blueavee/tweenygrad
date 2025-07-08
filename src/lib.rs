#![allow(unused)]
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum DType {
    Float32,
    Int32,
    Bool,
    Void,
}

#[derive(Clone, Debug)]
pub enum DTypeVal {
    Float32(f32),
    Int32(i32),
    Bool(bool),
    Str(String),
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
    DefineVar,
    SINK,
}

#[derive(Debug)]
pub struct Metadata {
    name: String,
    caller: String,
    backward: bool,
}

#[derive(Debug)]
pub struct Buffer {
    device: String,
    size: i32,
}

#[derive(Debug)]
pub struct ScheduleItem {
    ast: UOp,
    bufs: Vec<Buffer>,
}

type ScheduleWithVarRes = (Vec<ScheduleItem>, HashMap<UOp, i32>);

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
                src: Some(vec![]),
                arg: Some(data),
            },
        };
    }

    pub fn assign(&mut self, x: Tensor) -> &Tensor {
        self.uop = self.uop.assign(x.uop);
        return self;
    }

    pub fn add(&mut self, x: Tensor) -> &Tensor {
        self.uop = self.uop.add(x.uop, self.dtype.clone());
        return self;
    }

    pub fn realize(&mut self) {
        //first call schedule with vars and call run schedule
    }

    pub fn kernelize(&mut self) {
        //first call schedule with vars and call run schedule
        //
        //
    }

    // pub fn schedule_with_vars(&mut self, lst: Vec<Tensor>) -> ScheduleWithVarRes {
    //     return;
    // }
}

#[derive(Clone, Debug)]
pub struct UOp {
    op: Ops,
    dtype: DType,
    src: Option<Vec<UOp>>,
    arg: Option<ArgType>,
}

impl UOp {
    fn assign(&self, x: UOp) -> UOp {
        return UOp {
            op: Ops::ASSIGN,
            dtype: self.dtype.clone(),
            src: Some(vec![self.clone(), x]),
            arg: None,
        };
    }

    fn variable(
        name: String,
        min_val: i32,
        max_val: i32,
        // dtype: DType
    ) -> UOp {
        return UOp {
            op: Ops::DefineVar,
            dtype: DType::Int32,
            src: None,
            arg: Some(ArgType::List(vec![
                DTypeVal::Str(name),
                DTypeVal::Int32(min_val),
                DTypeVal::Int32(max_val),
            ])),
        };
    }

    fn sink() -> UOp {
        return UOp {
            op: Ops::SINK,
            dtype: DType::Void,
            // src: ,
            // arg: None,
        };
    }

    fn add(&mut self, y: UOp, dtype: DType) -> UOp {
        return UOp {
            op: Ops::ADD,
            dtype: dtype,
            src: Some(vec![self.clone(), y]),
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
