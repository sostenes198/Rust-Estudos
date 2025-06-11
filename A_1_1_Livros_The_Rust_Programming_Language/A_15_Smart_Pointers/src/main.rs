// Smart Pointers
// Box<T>, for allocating values on the heap
// Rc<T>, a reference counting type that enables multiple ownership
// Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time

mod box_sample;
mod custom_smart_pointer_box;
mod leak_memory_sample;
mod rc_reference_counting_sample;
mod ref_cell_sample;
mod weak_reference_sample;

fn main() {
    // box_sample::sample();
    // custom_smart_pointer_box::sample();
    // rc_reference_counting_sample::sample();
    // ref_cell_sample::sample();
    // leak_memory_sample::sample();
    weak_reference_sample::sample();
}
