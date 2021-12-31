use bevy::{prelude::*, render::render_resource::TextureUsages};
use bevy_ecs_tilemap::prelude::*;

fn startup(mut commands: Commands, asset_server: Res<AssetServer>, mut map_query: MapQuery) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let texture_handle = asset_server.load("tiles.png");

    // Create map entity and component:
    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    // Creates a new layer builder with a layer entity.
    let (mut layer_builder, _) = LayerBuilder::new(
        &mut commands,
        LayerSettings::new(
            MapSize(5, 2),
            ChunkSize(16, 16),
            TileSize(16.0, 16.0),
            TextureSize(96.0, 16.0),
            //TextureSize(96.0, 16.0),
        ),
        0u16,
        0u16,
    );

    layer_builder.set_all(TileBundle {
        tile: Tile {
            texture_index: 1,
            ..Default::default()
        },
        ..Default::default()
    });

    // Builds the layer.
    // Note: Once this is called you can no longer edit the layer until a hard sync in bevy.
    let layer_entity = map_query.build_layer(&mut commands, layer_builder, texture_handle);

    // Required to keep track of layers for a map internally.
    map.add_layer(&mut commands, 0u16, layer_entity);

    // Spawn Map
    // Required in order to use map_query to retrieve layers/tiles.
    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(-640.0, -256.0, 0.0))
        .insert(GlobalTransform::default());
}

fn main() {
    let mut app = App::new();

    #[cfg(target_arch = "wasm32")]
    app.insert_resource(WindowDescriptor {
        width: 720.,
        height: 480.,
        canvas: Some("#bevy-canvas".to_string()),
        ..Default::default()
    });
    #[cfg(not(target_arch = "wasm32"))]
    app.insert_resource(WindowDescriptor {
        width: 720.,
        height: 480.,
        ..Default::default()
    });

    app.add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_startup_system(startup)
        .add_system(set_texture_filters_to_nearest)
        .run();
}

pub fn set_texture_filters_to_nearest(
    mut texture_events: EventReader<AssetEvent<Image>>,
    mut textures: ResMut<Assets<Image>>,
) {
    // quick and dirty, run this for all textures anytime a texture is created.
    for event in texture_events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                if let Some(mut texture) = textures.get_mut(handle) {
                    texture.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING
                        | TextureUsages::COPY_SRC
                        | TextureUsages::COPY_DST;
                }
            }
            _ => (),
        }
    }
}
