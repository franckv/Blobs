#[macro_use]
extern crate log;

use amethyst::Application;
use amethyst::GameDataBuilder;
use amethyst::core::transform::TransformBundle;
use amethyst::input::{InputBundle, StringBindings};
use amethyst::renderer::plugins::{RenderFlat2D, RenderToWindow};
use amethyst::renderer::types::DefaultBackend;
use amethyst::renderer::RenderingBundle;
use amethyst::utils::application_root_dir;

mod blobs;
mod components;
mod map;
mod systems;

use crate::blobs::Blobs;

pub fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let binding_path = config_dir.join("bindings.ron");

    let rendering_bundle = RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
            RenderToWindow::from_config_path(display_config_path)
            .with_clear([0., 0., 0., 1.]))
        .with_plugin(RenderFlat2D::default());

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::InputSystem::default(), "input_mapper", &["input_system"])
        .with(systems::MoveSystem, "move_system", &["input_mapper"])
        .with_bundle(rendering_bundle)?;

    let mut game = Application::new(assets_dir, Blobs::default(), game_data)?;

    game.run();

    Ok(())
}
