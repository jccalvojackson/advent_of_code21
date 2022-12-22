use super::utils::{calc_directory_size, DirAction, DirNode};
use daggy::{Dag, Walker};
extern crate scan_fmt;
use std::error::Error;

pub fn p7b() -> Result<(), Box<dyn Error>> {
    let dir_act = include_str!("../../data/directories.txt")
        .lines()
        .map(|line| {
            let tmp_ln = line.replace('$', "");
            let mut tmp_line = tmp_ln.trim().split(' ');
            // let mut splt = tmp_line.split(' ');
            let ffield = tmp_line.next().unwrap();
            DirAction::from_fields(ffield, tmp_line.next())
        });
    let mut dag = Dag::<DirNode, u32, u32>::new();
    let root = dag.add_node(DirNode {
        size: 0,
        name: "root".to_string(),
        is_dir: true,
    });
    dag.add_child(
        root,
        0,
        DirNode {
            size: 0,
            name: "/".to_string(),
            is_dir: true,
        },
    );
    let mut current_parent = root.clone();

    for diraction in dir_act {
        let mydira: DirAction = diraction?;
        match mydira {
            DirAction::MoveInto(dir) => {
                let child_iter = dag.children(current_parent).iter(&dag);
                for (_, child) in child_iter {
                    let cn: &DirNode = dag.node_weight(child).unwrap();
                    if cn.name == dir {
                        current_parent = child;
                        break;
                    }
                }
            }
            DirAction::List => continue,
            DirAction::Dir(name) => {
                dag.add_child(
                    current_parent,
                    0,
                    DirNode {
                        size: 0,
                        name,
                        is_dir: true,
                    },
                );
            }
            DirAction::File(size, name) => {
                dag.add_child(
                    current_parent,
                    0,
                    DirNode {
                        size,
                        name,
                        is_dir: false,
                    },
                );
            }
            DirAction::GoBack => {
                (_, current_parent) = dag.parents(current_parent).walk_next(&dag).unwrap();
            }
        }
    }
    calc_directory_size(&mut dag, root);
    let nodes = dag.raw_nodes();
    let mut answer: i32 = 0;
    for node in nodes {
        let csize = node.weight.size;
        if csize < 100000 && node.weight.is_dir {
            answer += csize;
        }
    }
    println!("answer : {}", answer);
    let rootn = dag.node_weight(root).unwrap();
    let space_needed: i32 = 30000000 - (70000000 - rootn.size);
    let mut min_size = 70000000;
    for node in nodes {
        let csize = node.weight.size;
        if csize >= space_needed && csize < min_size {
            min_size = csize;
        }
    }
    println!("min dir : {}", min_size);
    // println!("{:?}", Dot::with_config(&dag, &[Config::EdgeNoLabel]));
    Ok(())
}
