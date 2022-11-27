use super::{base::Base, traits::{get::Get, delete::Delete, edit::Edit}};

pub struct Done{
    pub super_struct: Base,
}

impl Done{
    pub fn new(title: &str) -> Done {
        let base: Base = Base::new(title, "done");
        Done{super_struct: base}
    }
}

impl Get for Done{}
impl Delete for Done{}
impl Edit for Done{}