use sled::{ ConfigBuilder, Tree };

pub fn db () -> Tree {
    let config = ConfigBuilder::new()
        .path("/db".to_owned())
        .cache_capacity(10_000_000_000)
        .use_compression(true)
        .flush_every_ms(Some(1000))
        .snapshot_after_ops(100_000)
        .build();

    return Tree::start(config).unwrap();
}
