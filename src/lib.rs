use bevy::math::vec2;
use bevy::math::vec4;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::AsBindGroup;
use bevy::render::render_resource::ShaderType;
use bevy::sprite::Anchor;
use bevy::sprite::Material2d;
use bevy::sprite::Material2dPlugin;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::sprite::Mesh2dHandle;

pub mod prelude {
    pub use crate::TextureAtlasTilemap;
    pub use crate::TextureAtlasTilemapGeometry;
    pub use crate::TextureAtlasTilemapBundle;
    pub use crate::TextureAtlasTilemapPlugin;
}

pub const TILESTRIP_LEN: usize = 1000;

#[derive(Clone)]
#[derive(Component)]
pub struct TextureAtlasTilemap {
    pub width: usize,
    pub height: usize,
    pub atlas_indices: Vec<usize>,
}

impl Default for TextureAtlasTilemap {
    fn default() -> Self {
        Self {
            width: 1,
            height: 1,
            atlas_indices: vec![0],
        }
    }
}

impl TextureAtlasTilemap {
    pub fn new(width: usize, height: usize, initial_tile: usize) -> Self {
        Self {
            width,
            height,
            atlas_indices: vec![initial_tile; width * height]
        }
    }

    #[inline]
    pub fn index(&self, [x, y]: [usize; 2]) -> usize {
        y * self.width + x
    }
}

impl std::ops::Index<[usize; 2]> for TextureAtlasTilemap {
    type Output=usize;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.atlas_indices[self.index(index)]    
    }
}

impl std::ops::IndexMut<[usize; 2]> for TextureAtlasTilemap {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        let index = self.index(index);
        &mut self.atlas_indices[index]
    }
}

#[derive(Component)] 
pub struct TextureAtlasTilemapGeometry {
    pub tile_size: Vec2,
    pub anchor: Anchor,
}

impl Default for TextureAtlasTilemapGeometry {
    fn default() -> Self {
        Self { 
            tile_size: Vec2::splat(16.0),
            anchor: Anchor::BottomLeft,
        }
    }
}

#[derive(Default)]
#[derive(Bundle)]
pub struct TextureAtlasTilemapBundle {
    pub tilemap: TextureAtlasTilemap,
    pub tilemap_geometry: TextureAtlasTilemapGeometry,
    pub texture_atlas: Handle<TextureAtlas>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

pub const TILESTRIP_SHADER_HANDLE: HandleUntyped = HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 7269585014426076914);

#[derive(Clone, Debug)]
#[derive(ShaderType)]
pub struct TilestripUniform<const STRIP_LEN: usize = TILESTRIP_LEN> {
    pub len: f32,
    pub atlas_rects: [Vec4; STRIP_LEN],
}

impl <const N: usize> TilestripUniform<N> {
    pub fn new(len: usize) -> Self {
        Self {
            len: len as f32,
            atlas_rects: [vec4(0., 0., 1., 1.); N],
        }
    }
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-abab-8225-97e2a3f054e0"]
pub struct TilestripMaterial {
    #[texture(0)]
    #[sampler(1)]
    texture: Handle<Image>,
    #[uniform(2)]
    tilestrip_uniform: TilestripUniform,
}

impl Material2d for TilestripMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/tilestrip_fragment_shader.wgsl".into()
    }
}

fn spawn_tilestrips(
    mut commands: Commands,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut tilestrip_materials: ResMut<Assets<TilestripMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    tile_map_query: Query<(
        Entity,
        &TextureAtlasTilemap,
        &TextureAtlasTilemapGeometry,
        &Handle<TextureAtlas>,
    ), 
        Added<TextureAtlasTilemap>,
    >,
) { 
    tile_map_query.for_each(|(tilemap_id, tilemap, geometry, atlas_handle)| {
        let texture_atlas = texture_atlases.get(atlas_handle).unwrap();        
        let output_size = geometry.tile_size * vec2(tilemap.width as f32, tilemap.height as f32);
        let position = (-0.5 * Vec2::ONE + geometry.anchor.as_vec()) * output_size;
        let mut translation = position;
        let mut tilemap_index = 0usize;
        for _ in 0..tilemap.height {
            for start_x in (0..tilemap.width).step_by(TILESTRIP_LEN) {
                let remaining_tiles_in_row = tilemap.width - start_x;
                let strip_len = remaining_tiles_in_row.min(TILESTRIP_LEN);
                let mut tilestrip_uniform = TilestripUniform::<TILESTRIP_LEN>::new(strip_len);
                for i in 0..strip_len {
                    let atlas_index = tilemap.atlas_indices[tilemap_index];
                    let atlas_rect = texture_atlas.textures[atlas_index];
                    let min = atlas_rect.min / texture_atlas.size;
                    let max = atlas_rect.max / texture_atlas.size; 
                    tilestrip_uniform.atlas_rects[i] = (min, max).into();
                    tilemap_index += 1;
                }
                let strip_size = geometry.tile_size * vec2(strip_len as f32, 1.0);
                let material = tilestrip_materials.add(                    
                    TilestripMaterial {
                        texture: texture_atlas.texture.clone(),
                        tilestrip_uniform,
                    }
                );
                let mesh = meshes.add(Mesh::from(shape::Quad { size: strip_size, flip: false }));
                let transform = Transform::from_translation((translation + 0.5 * strip_size).extend(0.0));
                let tilestrip_id = commands
                    .spawn_bundle(MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(mesh),
                        material,
                        transform,
                        ..Default::default()
                    })
                    .id();
                commands.entity(tilemap_id).add_child(tilestrip_id);
                translation.x += strip_size.x;
            }
            translation.x = position.x;
            translation.y += geometry.tile_size.y;
        }
    });
}

pub struct TextureAtlasTilemapPlugin;

impl Plugin for TextureAtlasTilemapPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(Material2dPlugin::<TilestripMaterial>::default())
        .add_stage_after(
            CoreStage::Update, 
            "spawn_tilestrips", 
            SystemStage::parallel()
        )
        .add_system_to_stage("spawn_tilestrips", spawn_tilestrips);  
    }
}