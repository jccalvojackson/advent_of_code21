extern crate daggy;
use daggy::NodeIndex;
use daggy::{Dag, Walker};
extern crate scan_fmt;
use std::error::Error;

#[derive(Debug)]
pub enum DirAction {
    MoveInto(String),  // search
    GoBack,            // get parent node
    Dir(String),       // add child
    File(i32, String), // add child
    List,              // continue
}

impl DirAction {
    pub fn from_fields(ffield: &str, sfield: Option<&str>) -> Result<DirAction, Box<dyn Error>> {
        match ffield {
            "cd" => match sfield {
                Some(fld) => match fld {
                    ".." => Ok(DirAction::GoBack),
                    _ => Ok(DirAction::MoveInto(fld.to_string())),
                },
                _ => Err("bad parse".into()),
            },
            "dir" => match sfield {
                Some(fld) => Ok(DirAction::Dir(fld.to_string())),
                _ => Err("bad parse".into()),
            },
            "ls" => Ok(DirAction::List),
            _ => {
                let fsize: i32 = ffield.parse().unwrap();
                match sfield {
                    Some(fld) => Ok(DirAction::File(fsize, fld.to_string())),
                    _ => Err("bad parse".into()),
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct DirNode {
    pub size: i32,
    pub name: String,
    pub is_dir: bool,
}

impl DirNode {
    fn update_size(&mut self, increase: i32) {
        // ^^^ Here
        self.size += increase;
    }
}

pub fn calc_directory_size(dag: &mut Dag<DirNode, u32>, node: NodeIndex) {
    let cn = dag.node_weight(node).unwrap();
    if cn.size > 0 {
        return;
    }
    drop(cn);
    let mut child_iter = dag.children(node); //.iter(&dag);
    loop {
        let new_value;
        match child_iter.walk_next(&dag) {
            None => break,
            Some((_, child)) => {
                calc_directory_size(dag, child);
                let childn: &DirNode = dag.node_weight(child).unwrap();
                new_value = childn.size;
            }
        }
        let cn = dag.node_weight_mut(node).unwrap();
        cn.update_size(new_value)
    }
}
