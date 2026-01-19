fn main() {
	reproducible_panic::install();
	panic!("Oh no!");
}
