use toolkitengine::Launcher;
use toolkitengine::studio;

fn main() {
<<<<<<< HEAD
    Launcher::on(studio::CorePlugin)
        .add_plugins(studio::DebugPlugin)
=======

    App::new()
        .add_plugins(InitPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(SystemPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(BasisuLoaderPlugin)
>>>>>>> parent of e94b3f5 (pass)
        .run();

}
