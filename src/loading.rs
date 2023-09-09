use bevy::asset::{Asset, HandleId};

// bevy_rapier3d::prelude::*
use {crate::comment, bevy::gltf::Gltf};

use {bevy::prelude::*, bevy_asset_loader::prelude::*, bevy_kira_audio::AudioSource,
     seldom_fn_plugin};

// while asset_server.get_load_state(&m) != LoadState::Loaded {}
// #[derive(Resource)]
// struct Lander(Mesh);
// #[derive(Default, Resource)]
// struct Number(u32);

#[derive(AssetCollection, Resource)]
struct MyAssets {
  // #[asset(path = "images", collection(mapped))]
  // folder: HashMap<String, HandleUntyped>,
  // #[asset(paths("images/player.png", "images/tree.png"),
  //         collection(typed, mapped))]
  // files_typed: HashMap<String, Handle<Image>>,
  // #[asset(key = "files_untyped", collection(mapped))]
  // dynamic_files_untyped: HashMap<String, HandleUntyped>,
  // #[asset(key = "files_typed", collection(typed, mapped))]
  // dynamic_files_typed: HashMap<String, Handle<Image>>
  #[asset(path = "assets/images", key = "files_typed", collection(typed, mapped))]
  images_by_path: HashMap<String, Handle<Image>>
}
fn load_meshes(asset_server: Res<AssetServer>) {
  let lander = asset_server.load::<Mesh, _>("lunarlander.gltf");
}
// #[derive(AssetCollection, Resource)]
// pub struct PlanetTexture {
//   #[asset(path = "resources/aarau.png")]
//   pub texture: Handle<Image>
// }
// #[derive(Resource)]
// pub struct Images(pub bevy::utils::HashMap<String, Image>);
// fn load_images(asset_server: Res<AssetServer>, mut im: ResMut<Images>) {
//   let assets = asset_server.load_folder("Assets/images")
//                            .unwrap()
//                            .map(|r: Handle<Image>| r);
//   im.0.extend();

//   // let lander = asset_server.load::<Mesh, _>("lunarlander.gltf");
// }
pub fn loading_plugin(app: &mut App) {
  app.add_startup_system(load_meshes)
     .add_startup_system(load_images)
     .init_resource::<Number>()
     .add_system(spawn_meshes);
}
pub mod Squirrel {
  struct Squirrel {
    tail_length_meters: f32,
    hidden_nuts: u32,
    speed_meters_per_second: f32
  }
  pub fn new() -> Squirrel {
    Squirrel { tail_length_meters: 0.1,
               hidden_nuts: 0,
               speed_meters_per_second: 1.2 }
  }
}

// {
//   Squirrel::new()
// }
comment! {

  /// Helper resource for tracking our asset
  #[derive(Resource)]
  struct MyAssetPack(Handle<Gltf>);

  fn load_gltf(mut commands: Commands, ass: Res<AssetServer>) {
    let gltf = ass.load("lunarlander.gltf");
    commands.insert_resource(MyAssetPack(gltf));
  }
  fn spawn_gltf_objects(mut c: Commands, my: Res<MyAssetPack>, assets_gltf: Res<Assets<Gltf>>) {
    // if the GLTF has loaded, we can navigate its contents
    if let Some(gltf) = assets_gltf.get(&my.0) {
      // spawn the first scene in the file
      c.spawn(SceneBundle { scene: gltf.scenes[0].clone(),
                            ..default() });

      // spawn the scene named "YellowCar"
      c.spawn(SceneBundle { scene: gltf.named_scenes["YellowCar"].clone(),
                            transform: Transform::from_xyz(1.0, 2.0, 3.0),
                            ..default() });

      // PERF: the `.clone()`s are just for asset handles, don't worry :)
    }
  }
}
// #[derive(AssetCollection, Resource)]
// pub struct FontAssets {
//   #[asset(path = "fonts/FiraSans-Bold.ttf")]
//   pub fira_sans: Handle<Font>,
// }

// #[derive(AssetCollection, Resource)]
// pub struct AudioAssets {
//   #[asset(path = "audio/flying.ogg")]
//   pub flying: Handle<AudioSource>,
// }
