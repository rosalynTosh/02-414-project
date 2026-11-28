use std::{cmp::Ordering::{Equal, Greater, Less}, collections::HashMap, hash::Hash, unreachable};

use crate::{text_matcher::TextMatcher, text_matcher_unit_tests};

pub struct STChild<T: Eq + Hash + Copy> {
    slice: (usize, usize),
    subtree: SuffixTree<T>,
}

pub enum SuffixTree<T: Eq + Hash + Copy> {
    InternalNode {
        children: HashMap<T, STChild<T>>,
    },
    LeafNode {
        suffix_index: usize,
    },
}

struct PSTChild<'m, T: Eq + Hash + Copy> {
    slice: (usize, Option<usize>),
    subtree: PartialSuffixTree<'m, T>,
}

struct PSTInternalNode<'m, T: Eq + Hash + Copy> {
    children: HashMap<T, PSTChild<'m, T>>,
    suffix_link: Option<&'m PartialSuffixTree<'m, T>>,
}

enum PartialSuffixTree<'m, T: Eq + Hash + Copy> {
    InternalNode(PSTInternalNode<'m, T>),
    LeafNode {
        suffix_index: usize,
        suffix_link: Option<&'m PartialSuffixTree<'m, T>>
    }
}

pub fn construct_suffix_tree<T: Eq + Hash + Copy>(text: &[T]) -> SuffixTree<T> {
    let init_child_node = PartialSuffixTree::LeafNode {
        suffix_index: 0,
        suffix_link: None
    };

    let mut init_root_node_children = HashMap::new();
    init_root_node_children.insert(text[0], PSTChild {
        slice: (0, None),
        subtree: init_child_node,
    });

    let mut init_root_internal_node = PSTInternalNode {
        children: init_root_node_children,
        suffix_link: None,
    };
    init_root_internal_node.suffix_link = Some(&init_child_node);

    let mut suffix_tree_uc = PartialSuffixTree::InternalNode(init_root_internal_node);
    let mut last_phase_finish_node: &PartialSuffixTree<T> = &init_child_node;
    let mut last_phase_finish_child_offset: Option<(&PSTChild<T>, usize)> = None;

    'phases: for (i, ch) in text.iter().enumerate() {
        match (last_phase_finish_node, last_phase_finish_child_offset) {
            (PartialSuffixTree::InternalNode(PSTInternalNode { children, suffix_link }), None) => {
                if children.contains_key(ch) {
                    // Case C

                    continue 'phases;
                } else {
                    // Case B

                    // children.insert(*ch, PSTChild {
                    //     slice: (i, None),
                    //     subtree: PartialSuffixTree::LeafNode {
                    //         suffix_index: i,
                    //         suffix_link: (),
                    //     },
                    // });

                    todo!();
                }
            }
            (PartialSuffixTree::InternalNode(PSTInternalNode { children, suffix_link }), Some((child, offset))) => {
                if &text[child.slice.0 + offset] == ch {
                    // Case D

                    continue 'phases;
                } else {
                    // Case E

                    todo!();
                }
            }
            (PartialSuffixTree::LeafNode { suffix_index, suffix_link }, _) => {
                unreachable!();
            }
        }
    }

    SuffixTree::LeafNode { suffix_index: 0 }
}

pub struct SuffixTreeMatcher<'t, T: Eq + Hash + Copy> {
    text: &'t [T],
    suffix_tree: SuffixTree<T>,
}

enum SuffixTreeTraversalState<'m, T: Eq + Hash + Copy> {
    InternalNode {
        children: Vec<&'m STChild<T>>,
        i: usize,
    },
    LeafNode {
        suffix_index: usize,
    },
}

pub struct SuffixTreeIndexIterator<'m, 'p, 't, T: Eq + Hash + Copy> {
    matcher: &'m SuffixTreeMatcher<'t, T>,
    pattern: &'p [T],
    traversal_stack: Vec<SuffixTreeTraversalState<'m, T>>,
}

impl<'m, 'p, 't, T: Eq + Hash + Copy> Iterator for SuffixTreeIndexIterator<'m, 'p, 't, T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.traversal_stack.pop() {
                Some(SuffixTreeTraversalState::InternalNode { children, i }) => {
                    if i < children.len() {
                        match &children[i].subtree {
                            SuffixTree::InternalNode { children: subchildren } => {
                                self.traversal_stack.push(SuffixTreeTraversalState::InternalNode {
                                    children,
                                    i: i + 1,
                                });
                                self.traversal_stack.push(SuffixTreeTraversalState::InternalNode {
                                    children: subchildren.values().collect::<Vec<_>>(),
                                    i: 0,
                                });
                            }
                            SuffixTree::LeafNode { suffix_index } => {
                                self.traversal_stack.push(SuffixTreeTraversalState::InternalNode {
                                    children,
                                    i: i + 1,
                                });

                                return Some(*suffix_index);
                            }
                        }
                    }
                }
                Some(SuffixTreeTraversalState::LeafNode { suffix_index }) => {
                    return Some(suffix_index);
                }
                None => {
                    return None;
                }
            }
        }
    }
}

impl<'t, T: Eq + Hash + Copy> TextMatcher<'t, T> for SuffixTreeMatcher<'t, T> {
    type IndexIterator<'i, 'p> = SuffixTreeIndexIterator<'i, 'p, 't, T> where Self: 'i, T: 'p;

    fn from_text(text: &'t [T]) -> Option<SuffixTreeMatcher<'t, T>> {
        Some(SuffixTreeMatcher {
            text,
            suffix_tree: construct_suffix_tree(text),
        })
    }

    fn find_indices_of<'i, 'p>(&'i self, pattern: &'p [T]) -> Self::IndexIterator<'i, 'p> {
        let mut suffix_tree: &SuffixTree<T> = &self.suffix_tree;
        let mut p: usize = 0;

        loop {
            match suffix_tree {
                SuffixTree::InternalNode { children } => {
                    match children.get(&pattern[p]) {
                        Some(child) => {
                            let child_slice_size = child.slice.1 - child.slice.0;
                            let pattern_suffix_size = pattern.len() - p;

                            match Ord::cmp(&child_slice_size, &pattern_suffix_size) {
                                Less | Equal => {
                                    for i in 1..child_slice_size {
                                        if self.text[child.slice.0 + i] != pattern[p + i] {
                                            // No matches
                                            return SuffixTreeIndexIterator {
                                                matcher: self,
                                                pattern,
                                                traversal_stack: vec![],
                                            };
                                        }
                                    }

                                    suffix_tree = &child.subtree;
                                    p += child_slice_size;
                                }
                                Greater => {
                                    for i in 1..pattern_suffix_size {
                                        if self.text[child.slice.0 + i] != pattern[p + i] {
                                            // No matches
                                            return SuffixTreeIndexIterator {
                                                matcher: self,
                                                pattern,
                                                traversal_stack: vec![],
                                            };
                                        }
                                    }

                                    // Matches
                                    return SuffixTreeIndexIterator {
                                        matcher: self,
                                        pattern,
                                        traversal_stack: vec![match &child.subtree {
                                            SuffixTree::InternalNode { children } => SuffixTreeTraversalState::InternalNode {
                                                children: children.values().collect::<Vec<_>>(),
                                                i: 0,
                                            },
                                            SuffixTree::LeafNode { suffix_index } => SuffixTreeTraversalState::LeafNode {
                                                suffix_index: *suffix_index
                                            }
                                        }],
                                    };
                                }
                            }
                        }
                        None => {
                            // No matches
                            return SuffixTreeIndexIterator {
                                matcher: self,
                                pattern,
                                traversal_stack: vec![],
                            };
                        }
                    }
                }
                SuffixTree::LeafNode { suffix_index } => {
                    // One match
                    return SuffixTreeIndexIterator {
                        matcher: self,
                        pattern,
                        traversal_stack: vec![SuffixTreeTraversalState::LeafNode { suffix_index: *suffix_index }],
                    };
                }
            }
        }
    }
}

text_matcher_unit_tests!(SuffixTreeMatcher);