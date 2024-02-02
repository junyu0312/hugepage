#![feature(allocator_api)]
#![feature(alloc_layout_extra)]

pub mod hugepage;

#[test]
fn test() {
    use hugepage::HugeTlbAllocator;

    let mut vec = Vec::<u64, HugeTlbAllocator>::with_capacity_in(0, HugeTlbAllocator);

    for i in 0..((16 << 30) / 8) {
        vec.push(i);
    }
}
