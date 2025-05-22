use crate::domain::greeter::GreeterUseCase;

#[derive(Default)]
pub struct SayHelloUseCase;

impl GreeterUseCase for SayHelloUseCase {
    fn say_hello(&self, name: String) -> String {
        format!("Hello, {}!", name)
    }
}
