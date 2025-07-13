#![allow(unused)]
use ordered_float::OrderedFloat;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum DType {
    Float32,
    Int32,
    Bool,
    Void,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum DTypeVal {
    Float32(OrderedFloat<f32>),
    Int32(i32),
    Bool(bool),
    Str(String),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ArgType {
    Val(DTypeVal),
    List(Vec<DTypeVal>),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
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
            uop: UOp::new(Ops::CONST, dtype.clone(), Some(vec![]), Some(data)),
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
    }

    pub fn schedule_with_vars(&mut self)
    // -> ScheduleWithVarRes
    {
        // let sink = UOp.sink([self.clone()]);
        let sink = UOp::new(Ops::SINK, DType::Void, Some(vec![self.uop.clone()]), None);

        let mut remove_assing_map: HashMap<UOp, UOp> = HashMap::new();

        let toposorted = sink.toposort();

        for i in toposorted {
            if i.op == Ops::ASSIGN {
                remove_assing_map.insert(i.clone(), i.buf_uop());
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UOp {
    op: Ops,
    dtype: DType,
    src: Option<Vec<UOp>>,
    children: Option<Vec<UOp>>,
    arg: Option<ArgType>,
}

impl UOp {
    fn new(op: Ops, dtype: DType, src: Option<Vec<UOp>>, arg: Option<ArgType>) -> UOp {
        let created = UOp {
            op: op.clone(),
            dtype: dtype.clone(),
            src: src.clone(),
            arg: arg.clone(),
            children: Some(vec![]),
        };

        if src.is_some() && src.as_ref().unwrap().len() > 0 {
            if let Some(value) = src.clone().as_mut() {
                for i in value.iter_mut() {
                    if i.children.is_none() {
                        i.children = Some(vec![]);
                    }
                    i.children.as_mut().unwrap().push(created.clone());
                }
            }
        }

        return created;

        // return UOp::new {
        //     op: op,
        //     dtype: dtype,
        //     src: src,
        //     arg: arg,
        // };
    }

    fn assign(&self, x: UOp) -> UOp {
        return UOp::new(
            Ops::ASSIGN,
            self.dtype.clone(),
            Some(vec![self.clone(), x]),
            None,
        );
    }

    fn variable(
        name: String,
        min_val: i32,
        max_val: i32,
        // dtype: DType
    ) -> UOp {
        return UOp::new(
            Ops::DefineVar,
            DType::Int32,
            None,
            Some(ArgType::List(vec![
                DTypeVal::Str(name),
                DTypeVal::Int32(min_val),
                DTypeVal::Int32(max_val),
            ])),
        );
    }

    fn sink(srcs: &[UOp]) -> UOp {
        return UOp::new(Ops::SINK, DType::Void, Some(srcs.to_vec()), None);
    }

    fn add(&mut self, y: UOp, dtype: DType) -> UOp {
        return UOp::new(Ops::ADD, dtype, Some(vec![self.clone(), y]), None);
    }

    fn toposort(&self) -> Vec<UOp> {
        let mut ret: Vec<UOp> = vec![];
        let mut visited_set: HashSet<UOp> = HashSet::new();
        let mut stack: Vec<(UOp, bool)> = vec![(self.clone(), false)];

        while stack.len() > 0 {
            if let Some((node, visited)) = stack.pop() {
                if visited_set.contains(&node) {
                    continue;
                }
                if (!visited) {
                    stack.push((node.clone(), true));
                    for i in node.src.as_ref().unwrap() {
                        stack.push((i.clone(), false));
                    }
                } else {
                    visited_set.insert(node.clone());
                    ret.push(node.clone());
                }
            }
        }

        return ret;
    }

    fn buf_uop(&self) -> UOp {
        return self.src.as_ref().unwrap()[0].clone();
    }
}

pub fn apply_map_to_tensors(applied_map: HashMap<UOp, UOp>) {
    let mut all_uops: HashSet<UOp> = HashSet::new();
    let mut searhc_ops = applied_map.keys().collect::<Vec<&UOp>>();

    while searhc_ops.len() > 0 {
        let x = searhc_ops.pop().unwrap();
        if all_uops.contains(x) {
            continue;
        }
        all_uops.insert(x.clone());
        for i in x.children.as_ref().unwrap() {
            searhc_ops.push(i);
        }
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
