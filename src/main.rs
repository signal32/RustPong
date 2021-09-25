mod pong;
mod systems;

use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

use crate::pong::Pong;
use amethyst::core::TransformBundle;
use amethyst::input::{InputBundle, StringBindings};


fn main() -> amethyst::Result<()> {
    println!("Hello, world!");

    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config = app_root.join("conf").join("display.ron");
    let input_config = app_root.join("conf").join("input.ron");
    let assets_dir=  app_root.join("res");

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(input_config)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::PaddleSystem, "paddle_system", &["input_system"])
        .with(systems::MoveBallsSystem, "ball_system", &[])
        .with(systems::BounceSystem, "bounce_system", &["paddle_system", "ball_system"])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(RenderToWindow::from_config_path(display_config)?
                    .with_clear([0.05, 0.05, 0.05, 1.0])
                )
                .with_plugin(RenderFlat2D::default())
        )?;

    let mut game = Application::new(assets_dir, Pong::default(), game_data)?;
    game.run();



    Ok(())
}
