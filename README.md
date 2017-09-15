This repository provides the major source code examples for _Programming Rust_.

```
|-- chapter_03
|   |-- demo_c_enum          --- A simple C-like enum example
|   |-- demo_enum            --- A simple enum example
|   |-- demo_struct          --- A simple struct example
|   `-- demo_tuple           --- A simple tuple example
|-- chapter_04
|   |-- printvalue           --- Simple trait for expanding an existing type
|   |-- trig                 --- Adds trig operations to types convertable to f64
|   `-- writeint             --- Example extension trait
|-- chapter_05
|   |-- quicksort_borrow     --- A quicksort implementation that demos borrowing
|   `-- quicksort_move       --- A quicksort implementation that moves values
|-- chapter_06
|   |-- cursor               --- Example cursor implementation for a vector
|   |-- linked_list          --- Iterator implementation for a simple linked list
|   `-- zip_longest          --- Iterator adapter example for zipping without early exit
|-- chapter_07
|   |-- active_queue         --- Defer arbitrary work to a single background thread
|   |-- message_passing      --- Simple message passing example for background reads
|   `-- thread_pool          --- Defer arbitrary work to concurrent background threads
|-- chapter_08
|   |-- person_builder       --- Example Builder pattern
|   |-- state_machine_simple --- Simple state machine implementation
|   `-- state_machine_typed  --- Type-safe operations for a state machine
|-- chapter_09
|   `-- binheap              --- A tested, documented, binary heap
|-- chapter_10
|   |-- do_digest            --- Simple trait for digesting structs containing strings
|   `-- do_digest_derive     --- A custom derive procedural macro for the DoDigest trait
`-- chapter_11
    `-- export-lib           --- Example Rust lib with a C interface exported
```
