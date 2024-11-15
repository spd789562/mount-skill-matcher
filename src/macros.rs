pub mod macros {
    macro_rules! get_node_from_path {
        ($node:ident, $path:expr) => {
            $node
                .read()
                .unwrap()
                .at_path($path)
                .expect("missing node at path")
        };
    }

    pub(crate) use get_node_from_path;
}
