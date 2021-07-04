
# TODO LIST

## For 0.2.0:

- Improve interface to hide attributes of TreeModel and methods of TreeNode.
- All iterators must return a tuple with node and node index.
- Put a hash map on each node, containing node content (key) and index (value), to speed up find_node process.
- Convert function arguments from &String to &str.
- Methods that return a Result should generate an error type implementing the std::error::Error trait, instead of a simple String.
- Enable doctest.

## Backlog:

- Allow using the BufReader directly to read data from the tree, instead of parsing and generating a model in mem. For very big trees.
