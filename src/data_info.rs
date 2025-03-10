use std::collections::HashMap;

use crate::struct_info::{self, StructInfo};

struct DataInfo {
    vector_indicies: Vec<usize>,
    scalar_indicies: Vec<usize>,
    map: HashMap<String, (usize, usize)>,
}

impl DataInfo {
    pub fn from_struct_info(struct_info: StructInfo) -> Self;
    pub fn vector_indicies(&self) -> &Vec<usize>;
    pub fn scalar_indicies(&self) -> &Vec<usize>;
    pub fn max_vector_index_by_type(
        &self,
        type_idents: Vec<&Ident>,
    ) -> &Vec<usize>;
    pub fn max_scalar_index_by_type(
        &self,
        type_idents: Vec<&Ident>,
    ) -> &Vec<usize>;
}
