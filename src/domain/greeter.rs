pub trait GreeterUseCase: Send + Sync + 'static {
    fn say_hello(&self, name: String) -> String;
}
