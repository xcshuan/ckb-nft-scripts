use ckb_std::{
    ckb_constants::Source,
    ckb_types::{bytes::Bytes, prelude::*},
    high_level::{load_cell_type, QueryIter},
};

pub fn count_cells_with_type_args(source: Source, condition: &dyn Fn(&Bytes) -> bool) -> usize {
    QueryIter::new(load_cell_type, source)
        .filter(|type_opt| match type_opt {
            Some(type_) => {
                let type_args: Bytes = type_.args().unpack();
                condition(&type_args)
            }
            None => false,
        })
        .count()
}
