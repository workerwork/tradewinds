pub trait Specification<T: ?Sized> {
    fn is_satisfied_by(&self, candidate: &T) -> bool;
    fn message(&self) -> String; // 违规时的提示信息
}
