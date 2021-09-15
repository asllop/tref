
# TODO LIST


- Join Tree and TreeModel. TreeModel is only used to get access to iterators. We want `Forest::new_tree(...)` to return the tree, and use this tree ref to call `set_root`, `link_node`, etc. Actually we want to move these methods to the tree model. Doesn't make sense having 2 models for a tree.
- Put a hash map on each node, containing node content (key) and index (value), to speed up find_node process.
- Methods that return a Result should generate an error type implementing the std::error::Error trait, instead of a simple String.
- Enable doctest.
- Improve interface to hide attributes of TreeModel and methods of TreeNode from user, but not from other TREF components.

## Backlog:

- Allow using the BufReader directly to read data from the tree, instead of parsing and generating a model in mem. For very big trees.
