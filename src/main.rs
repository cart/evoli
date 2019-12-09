#[macro_use]
extern crate log;

use amethyst::assets::PrefabLoaderSystem;
use amethyst::{
    audio::DjSystem,
    gltf::GltfSceneLoaderSystem,
    prelude::*,
    utils::application_root_dir,
    window::DisplayConfig,
};

mod components;
mod resources;
mod states;
mod systems;
mod utils;

use crate::components::{combat, creatures};
use crate::resources::audio::Music;
use crate::states::loading::LoadingState;

use amethyst_precompile::{PrecompiledRenderBundle, PrecompiledDefaultsBundle, PrecompiledSpriteBundle, start_game};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let resources = application_root_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
        + "/resources";
    let display_config_path = resources.clone() + "/display_config.ron";
    let key_bindings_path = resources.clone() + "/input.ron";

    let display_config = DisplayConfig::load(display_config_path);

    // The global game data. Here we register all systems and bundles that will run for every game state. The game states
    // will define additional dispatchers for state specific systems. Note that the dispatchers will run in sequence,
    // so this setup sacrifices performance for modularity (for now).
    let game_data = GameDataBuilder::default()
        .with(
            PrefabLoaderSystem::<creatures::CreaturePrefabData>::default(),
            "creature_loader",
            &[],
        )
        .with(
            GltfSceneLoaderSystem::default(),
            "gltf_loader",
            &["creature_loader"],
        )
        .with(
            PrefabLoaderSystem::<combat::FactionPrefabData>::default(),
            "",
            &[],
        )
        .with(
            DjSystem::new(|music: &mut Music| music.music.next()),
            "dj",
            &[],
        )
        
        // .with_bundle(TransformBundle::new())?
        // .with_bundle(AudioBundle::default())?
        // .with_bundle(WindowBundle::from_config(display_config))?
        // .with_bundle(UiBundle::<DefaultBackend, StringBindings>::new())?
        .with_bundle(PrecompiledDefaultsBundle{
            key_bindings_path: &key_bindings_path,
            display_config: display_config
        })?
        .with_bundle(PrecompiledRenderBundle{})?
        .with_bundle(PrecompiledSpriteBundle{})?;

    // Set up the core application.

    start_game(&resources, game_data, Some(Box::new(LoadingState::default())));
        // CoreApplication::build(resources, LoadingState::default())?
        //     .with_frame_limit(FrameRateLimitStrategy::Sleep, 60)
        //     .build(game_data)?;
    // game.run();
    Ok(())
}
