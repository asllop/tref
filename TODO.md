
# TODO LIST

## For 0.3.0

- Put a hash map on each node, containing node content (key) and index (value), to speed up find_node process.
- Methods that return a Result should generate an error type implementing the std::error::Error trait, instead of a simple String.
- Enable doctest.
- Improve interface to hide attributes of TreeModel and methods of TreeNode from user, but not from other TREF components.

## Backlog:

- Allow using the BufReader directly to read data from the tree, instead of parsing and generating a model in mem. For very big trees.
