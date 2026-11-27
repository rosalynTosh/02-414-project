use std::cmp::Ordering::{Equal, Greater, Less};

use crate::{text_matcher::TextMatcher, text_matcher_unit_tests};

pub struct SuffixTreeChild {
    slice: (usize, usize),
    subtree: SuffixTree,
}

pub enum SuffixTree {
    InternalNode {
        children: Vec<SuffixTreeChild>, // todo: replace with HashMap from first char to STChild
    },
    LeafNode {
        suffix_index: usize,
    },
}

pub fn construct_suffix_tree<T: Eq + Ord>(text: &[T]) -> SuffixTree {
    SuffixTree::LeafNode { suffix_index: 0 } // TODO
}

pub struct SuffixTreeMatcher<'t, T: Eq> {
    text: &'t [T],
    suffix_tree: SuffixTree,
}

enum SuffixTreeTraversalState<'m> {
    InternalNode {
        children: &'m Vec<SuffixTreeChild>,
        i: usize,
    },
    LeafNode {
        suffix_index: usize,
    },
}

pub struct SuffixTreeIndexIterator<'m, 'p, 't, T: Eq> {
    matcher: &'m SuffixTreeMatcher<'t, T>,
    pattern: &'p [T],
    traversal_stack: Vec<SuffixTreeTraversalState<'m>>,
}

impl<'m, 'p, 't, T: Eq + Ord> Iterator for SuffixTreeIndexIterator<'m, 'p, 't, T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.traversal_stack.pop() {
                Some(SuffixTreeTraversalState::InternalNode { children, i }) => {
                    if i < children.len() {
                        match &children[i].subtree {
                            SuffixTree::InternalNode { children: subchildren } => {
                                self.traversal_stack.push(SuffixTreeTraversalState::InternalNode { children, i: i + 1 });
                                self.traversal_stack.push(SuffixTreeTraversalState::InternalNode { children: subchildren, i: 0 });
                            }
                            SuffixTree::LeafNode { suffix_index } => {
                                self.traversal_stack.push(SuffixTreeTraversalState::InternalNode { children, i: i + 1 });

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

impl<'t, T: Eq + Ord> TextMatcher<'t, T> for SuffixTreeMatcher<'t, T> {
    type IndexIterator<'i, 'p> = SuffixTreeIndexIterator<'i, 'p, 't, T> where Self: 'i, T: 'p;

    fn from_text(text: &'t [T]) -> Option<SuffixTreeMatcher<'t, T>> {
        Some(SuffixTreeMatcher {
            text,
            suffix_tree: construct_suffix_tree(text),
        })
    }

    fn find_indices_of<'i, 'p>(&'i self, pattern: &'p [T]) -> Self::IndexIterator<'i, 'p> {
        let mut suffix_tree: &SuffixTree = &self.suffix_tree;
        let mut p: usize = 0;

        loop {
            match suffix_tree {
                SuffixTree::InternalNode { children } => {
                    let mut lo: usize = 0;
                    let mut hi: usize = children.len();

                    while lo < hi {
                        let mid: usize = (lo + hi) / 2;
                        let child = &children[mid];

                        match Ord::cmp(&self.text[child.slice.0], &pattern[p]) {
                            Less => {
                                lo = mid + 1;
                            }
                            Equal => {
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
                                        break;
                                    }
                                    Greater => {
                                        for i in 1..pattern_suffix_size {
                                            if self.text[children[mid].slice.0 + i] != pattern[p + i] {
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
                                                    children,
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
                            Greater => {
                                hi = mid;
                            }
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