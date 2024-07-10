pub mod antiafk;
pub mod system_health_monitor;

fn main() {
    // Start the anti-afk thread
    antiafk::start_antiafk();

    // Start the system health monitor
    system_health_monitor::start_health_monitor();
}
