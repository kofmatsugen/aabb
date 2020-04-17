mod bundle;
mod components;
#[cfg(feature = "debug")]
pub(crate) mod debug;
mod event;
pub(crate) mod system;
pub(crate) mod types;

#[cfg(feature = "debug")]
pub(crate) use components::collision::Collision;
pub use components::{collision::Collisions, last_transform::LastTransform};
pub use event::contact::{ContactEvent, ContactEventChannel};

#[cfg(feature = "debug")]
use amethyst::{
    core::TransformBundle,
    input::{InputBundle, StringBindings},
    renderer::{
        plugins::{RenderDebugLines, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    Application, GameDataBuilder,
};

fn main() -> Result<(), amethyst::Error> {
    #[cfg(feature = "debug")]
    {
        amethyst::start_logger(amethyst::LoggerConfig::default());
        let app_root = application_root_dir()?;
        let resources_dir = app_root.join("resources");
        let display_config_path = resources_dir.join("display_config.ron");

        let game_data = GameDataBuilder::default()
            .with_bundle(InputBundle::<StringBindings>::new())?
            .with_bundle(TransformBundle::new())?
            .with_bundle(bundle::AabbCollisionBundle::<()>::new())?
            .with_barrier()
            .with_bundle(
                RenderingBundle::<DefaultBackend>::new()
                    .with_plugin(
                        RenderToWindow::from_config_path(display_config_path)?
                            .with_clear([1., 1., 1., 1.0]),
                    )
                    .with_plugin(RenderDebugLines::default()),
            )?;

        let mut game = Application::new(
            resources_dir,
            debug::state::CollisionState::default(),
            game_data,
        )?;
        log::info!("start game");
        game.run();
    }

    Ok(())
}
