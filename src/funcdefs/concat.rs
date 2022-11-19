pub trait Concat<T> {
    fn concat_0(left: &[T], right: &[T]) -> Vec<T>;
    fn concat_2(left: Vec<T>, right: Vec<T>) -> Vec<T>;
    fn concat_3<'a>(left: &'a [T], right: &'a [T]) -> &'a [T];
    fn concat_4(left: &[T], right: &[T]) -> [T];
}
