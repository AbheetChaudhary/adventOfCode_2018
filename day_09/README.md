### Method
- Implement a vector based doubly linked list which has `RefCell<Option<Node>>`, call it `memory`
- Removing nodes turns the `Option` to `None` and pushes its index in `memory.empty_slots` for quick access when there is a new node to insert

### Input file format
\<players\>;\<max_points\>
