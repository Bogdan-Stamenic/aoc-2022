use std::collections::HashMap;
use petgraph::{graph::{DiGraph, NodeIndex}, visit::DfsPostOrder};

#[derive(Clone, Copy, Debug)]
pub enum FileNodeType {
    Dir,
    File(u32),//has size
}

#[derive(Clone, Debug)]
pub struct FileNode {
    name : String,
    node : FileNodeType,
}

pub struct MyGraph {
    ftree : DiGraph<FileNode, ()>,
    root_node_idx : NodeIndex,
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> MyGraph {
    let mut file_tree = DiGraph::<FileNode, ()>::new();
    let mut cwd = Vec::<String>::new();
    let mut dir_to_node_index = HashMap::<String,NodeIndex>::new();
    for line in input.lines() {
        match line.bytes().skip(2).next().unwrap() {
            b'c' => {// cd
                if line.bytes().skip(5).next().unwrap() == b'.' {// cd ..
                    cwd.pop();
                } else if line.bytes().skip(5).next().unwrap() == b'/' {
                    /* cd / (is root directory) */
                    let new_node = FileNode {
                        name : "/".to_string(),
                        node : FileNodeType::Dir,
                    };
                    let new_node_index = file_tree.add_node(new_node.clone());
                    dir_to_node_index.insert(new_node.name, new_node_index);
                    cwd.push("/".to_string());
                } else {// cd <some-directory>
                    cwd.push(line[5..].to_string());
                }
            },
            b'l' => {/* ls -> noop */},
            b'r' => {// dir from ls cmd
                /* Use full dir path to avoid duplicate names overwriting each other */
                let mut dir_full_path: String = cwd.join("/").to_string();
                dir_full_path.push('/');
                dir_full_path.push_str(&line[4..]);
                let new_node = FileNode {
                    name : dir_full_path.clone(),
                    node : FileNodeType::Dir,
                };
                let new_node_index = file_tree.add_node(new_node);
                let cwd_node_index : NodeIndex = *dir_to_node_index.get(&cwd.join("/")).unwrap();
                dir_to_node_index.insert(dir_full_path, new_node_index);
                file_tree.add_edge(cwd_node_index, new_node_index, ());
            },
            _ => {// probably size and filename
                let mut contents = line.split(' ').rev();
                /* Use full file path to avoid duplicate names overwriting each other */
                let mut file_full_path: String = cwd.join("/").to_string();
                file_full_path.push('/');
                file_full_path.push_str(&contents.next().unwrap());
                let new_node = FileNode {
                    name : file_full_path.clone(),
                    node : FileNodeType::File(contents.next().unwrap().parse::<u32>().unwrap()),
                };
                let new_node_index = file_tree.add_node(new_node.clone());
                let cwd_node_index : NodeIndex = *dir_to_node_index.get(&cwd.join("/")).unwrap();
                dir_to_node_index.insert(file_full_path, new_node_index);
                file_tree.add_edge(cwd_node_index, new_node_index, ());
            },
        }
    }
    MyGraph {
        ftree : file_tree,
        root_node_idx : *dir_to_node_index.get("/").unwrap(),
    }
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &MyGraph) -> u32 {
    let MyGraph {ftree: file_tree, root_node_idx: start} = input;
    let mut dir_sizes = HashMap::<String,u32>::new();
    let mut dfs_post = DfsPostOrder::new(&file_tree, *start);
    /* Graph traversal; build sizes from the bottom up */
    while let Some(dfs) = dfs_post.next(&file_tree) {
        let curr_file_node = &file_tree[dfs];
        let mut dir_size = 0;
        match curr_file_node.node {
            /* Is dir: sum up sizes of neighbours */
            FileNodeType::Dir => {
                for neigh_idx in file_tree.neighbors(dfs) {
                    let foo = &file_tree[neigh_idx];
                    dir_size += match foo.node {
                        FileNodeType::Dir => *dir_sizes.get(&foo.name as &str).unwrap(),
                        FileNodeType::File(size) => size,
                    };
                }
                dir_sizes.insert(curr_file_node.name.clone(), dir_size);
            },
            /* Is file: noop */
            FileNodeType::File(_) => (),
        };
    }
    dir_sizes
        .iter()
        .map(|(_,val)| *val)
        .filter(|x| *x <= 100_000u32)
        .sum()
}

//#[aoc(day7, part2)]
//pub fn solve_part2(input: &str) -> u32 

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
    #[test]
    fn test_day7_generator() {
        let input: MyGraph = input_generator(TEST_INPUT);
        assert_eq!(input.ftree.node_count(),14)
    }
    #[test]
    fn test_day7p1() {
        let input: MyGraph = input_generator(TEST_INPUT);
        assert_eq!(solve_part1(&input), 95437);
    }
}
