use etagere::*;

fn main() {
    let mut atlas = AtlasAllocator::new(size2(1000, 1000));

    let a = atlas.allocate(size2(100, 1000)).unwrap();
    let b = atlas.allocate(size2(900, 200)).unwrap();

    atlas.deallocate(a.id);

    let c = atlas.allocate(size2(300, 200)).unwrap();

    assert_eq!(c.rectangle, atlas.get(c.id));

    atlas.deallocate(c.id);
    atlas.deallocate(b.id);
}
