use amethyst::{renderer::{
    sprite_visibility::SpriteVisibilitySortingSystem, types::DefaultBackend,
    visibility::VisibilitySortingSystem, RenderingSystem, SpriteSheet,
}, core::{SystemBundle, frame_limiter::FrameRateLimitStrategy, transform::TransformBundle}, audio::AudioBundle, ui::UiBundle, ecs::DispatcherBuilder, error::Error, prelude::*, window::{DisplayConfig, WindowBundle}, assets::{ProgressCounter, Processor}, input::{InputBundle, StringBindings}};

mod render_graph;

use render_graph::RenderGraph;

pub struct MainState {
    real_state: Option<Box<dyn SimpleState>>
}

impl SimpleState for MainState {
    fn on_start(&mut self, mut data: StateData<GameData>) {
        if let Some(ref mut state) = self.real_state {
            state.on_start(data);
        }
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        if let Some(ref mut state) = self.real_state {
            state.update(data)
        } else {
            Trans::None
        }
    }
}

pub struct PrecompiledRenderBundle {

}


// saves ~13 seconds
impl<'a, 'b> SystemBundle<'a, 'b> for PrecompiledRenderBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        builder.add_thread_local(RenderingSystem::<DefaultBackend, _>::new(
            RenderGraph::default(),
        ));
        Ok(())
    }
}


pub struct PrecompiledDefaultsBundle<'a> {
    pub key_bindings_path: &'a str,
    pub display_config: DisplayConfig,
}

impl<'a, 'b, 'c> SystemBundle<'a, 'b> for PrecompiledDefaultsBundle<'c> {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        // saves ~ 1 second
        InputBundle::<StringBindings>::new().with_bindings_from_file(self.key_bindings_path)?.build(builder)?;

        // this set saves ~ 2 seconds
        TransformBundle::new().build(builder)?;
        AudioBundle::default().build(builder)?;
        WindowBundle::from_config(self.display_config).build(builder)?;
        UiBundle::<DefaultBackend, StringBindings>::new().build(builder)?;
        Ok(())
    }
}

pub struct PrecompiledSpriteBundle {}


impl<'a, 'b> SystemBundle<'a, 'b> for PrecompiledSpriteBundle {
    // saves less than one second
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        builder.add(
            Processor::<SpriteSheet>::new(),
            "sprite_sheet_processor",
            &[],
        );
        builder.add(
            VisibilitySortingSystem::new(),
            "visibility_system",
            &["transform_system"],
        );
        builder.add(
            SpriteVisibilitySortingSystem::new(),
            "sprite_visibility_system",
            &["transform_system"],
        );
        Ok(())
    }
}
// saves ~2 seconds
pub fn start_game(resources: &str, game_data_builder: GameDataBuilder<'static, 'static>, state: Option<Box<dyn SimpleState>>) {
    let mut game: Application<GameData> = CoreApplication::build(resources, MainState{ real_state: state }).unwrap()
        .with_frame_limit(FrameRateLimitStrategy::Sleep, 60)
        .build(game_data_builder).unwrap();
    game.run();
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
