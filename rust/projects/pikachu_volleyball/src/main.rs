use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    input::{InputBundle, StringBindings},
    ui::{RenderUi, UiBundle}, 
    audio::{AudioBundle, DjSystemDesc}
};

mod pikachuvolleyball;
use crate::pikachuvolleyball::PikachuVolleyball;

mod systems;

mod audio;
use crate::audio::Music;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;

    let config_dir = app_root.join("config");
    let assets_dir = app_root.join("assets");
    let config_display_path = config_dir.join("display.ron");
    let config_binding_path = config_dir.join("bindings_config.ron");

    let input_bundle = InputBundle::<StringBindings>::new()
    .with_bindings_from_file(config_binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(input_bundle)?
        .with(systems::PlayerSystem, "player_system", &["input_system"])
        .with(systems::PokeballSystem, "pokeball_system", &[])
        .with(
            systems::BounceSystem,
            "collision_system", 
            &["player_system", "pokeball_system"]
        )
        .with(systems::WinnerSystem, "winner_system", &["pokeball_system"])
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(AudioBundle::default())?
        .with_system_desc(DjSystemDesc::new(|music: &mut Music| music.music.next()), "dj_system", &[])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(config_display_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?;
    let mut game = Application::new(assets_dir, PikachuVolleyball, game_data)?;
    game.run();
    Ok(())
}
