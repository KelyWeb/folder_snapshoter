mod app;
use app::Application;


fn main() {
    
    Application::start(std::env::args().collect::<Vec<_>>());
}