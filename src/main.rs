#[macro_use]
extern crate log;

use amethyst::{Application, LoggerConfig};
use amethyst::GameDataBuilder;
use amethyst::config::Config;
use amethyst::core::transform::TransformBundle;
use amethyst::input::{InputBundle, StringBindings};
use amethyst::renderer::plugins::{RenderFlat2D, RenderToWindow};
use amethyst::renderer::types::DefaultBackend;
use amethyst::renderer::RenderingBundle;
use amethyst::utils::application_root_dir;

use log::LevelFilter;

mod config;
pub mod components;
pub mod map;
mod state;
mod systems;
pub mod utils;

use crate::state::GameplayState;
use crate::config::BlobsConfig;

pub fn main() -> amethyst::Result<()> {
    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let binding_path = config_dir.join("bindings.ron");
    let config_path = config_dir.join("config.ron");

    let config = BlobsConfig::load(&config_path);

    let mut logger_config = LoggerConfig::default();
    logger_config.level_filter = if config.log.debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    amethyst::start_logger(logger_config);

    let rendering_bundle = RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
            RenderToWindow::from_config_path(display_config_path)
            .with_clear([0., 0., 0., 1.]))
        .with_plugin(RenderFlat2D::default());

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;

    let mut game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::InputSystem::default(), "input_mapper", &["input_system"])
        .with(systems::AttackSystem, "attack_system", &["input_mapper"])
        .with(systems::MoveSystem, "move_system", &["attack_system", "input_mapper"])
        .with(systems::FovSystem, "fov_system", &["move_system"])
        .with(systems::InitSystem, "init_system", &["fov_system"])
        .with_bundle(rendering_bundle)?;

    if config.log.debug {
        game_data = game_data.with(
            systems::DebugSystem, "debug_system", &["init_system", "fov_system"]);
    }

    let mut game = Application::build(assets_dir, GameplayState::default())?
        .with_resource(config.map)
        .with_resource(config.fov)
        .build(game_data)?;

    game.run();

    Ok(())
}
