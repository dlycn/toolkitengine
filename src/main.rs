use toolkitengine::Launcher;
use toolkitengine::studio;

fn main() {
    Launcher::on(studio::CorePlugin)
        .add_plugins(studio::DebugPlugin)
        .run();

}
