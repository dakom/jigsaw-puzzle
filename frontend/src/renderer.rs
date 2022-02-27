pub mod item;
pub mod spritesheet;
use item::*;
mod picker;
use picker::ScenePicker;

use crate::pieces::{PiecesOrder};
use spritesheet::{PuzzleSheet, SpriteCell, SpriteSheet};
use crate::media::Media;
use crate::prelude::*;
use crate::camera::Camera;
use shipyard::*;
use nalgebra_glm::Mat4;
use web_sys::{WebGlRenderingContext, HtmlImageElement};
use std::ops::{Deref, DerefMut};
use awsm_web::webgl::{
    BufferMask,
    WebGl1Renderer,
    AttributeOptions,
    BufferData,
    BufferTarget,
    BufferUsage,
    DataType,
    Id,
    BeginMode,
    GlToggle,
    BlendFactor,
    ShaderType,
    ResizeStrategy,
    TextureTarget,
    SimpleTextureOptions,
    PixelFormat,
    WebGlTextureSource,
    VertexArray,
    NameOrLoc,
};
use crate::prelude::*;

pub type RendererViewMut<'a> = NonSendSync<UniqueViewMut<'a, SceneRenderer>>;

#[derive(Component)]
pub struct SceneRenderer {
    renderer: WebGl1Renderer,
    picker_program_id: Id,
    texture_program_id: Id,
    vao_id: Id,
    geom_buffer_id: Id,
    tex_buffer_id: Id,
    picker: Option<ScenePicker>
}

impl Deref for SceneRenderer {
    type Target = WebGl1Renderer;

    fn deref(&self) -> &WebGl1Renderer {
        &self.renderer
    }
}

impl DerefMut for SceneRenderer {
    fn deref_mut(&mut self) -> &mut WebGl1Renderer {
        &mut self.renderer
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pass {
    Picker,
    Forward
}

impl SceneRenderer {
    pub fn new (ctx: WebGlRenderingContext, media: &Media) -> Result<Self, awsm_web::errors::Error> {
        let mut renderer = WebGl1Renderer::new(ctx).unwrap_ext();

        //This demo is specifically using webgl1, which needs to register the extension
        //Everything else is the same API as webgl2 :)
        renderer.register_extension_instanced_arrays()?;
        renderer.register_extension_vertex_array()?;
        
        let vertex_id = renderer.compile_shader(&media.vertex_shader, ShaderType::Vertex)?;
        let picker_fragment_id = renderer.compile_shader(&media.picker_fragment_shader, ShaderType::Fragment)?;
        let texture_fragment_id = renderer.compile_shader(&media.texture_fragment_shader, ShaderType::Fragment)?;
        
        let picker_program_id = renderer.compile_program(&[vertex_id, picker_fragment_id])?;
        let texture_program_id = renderer.compile_program(&[vertex_id, texture_fragment_id])?;

        let vao_id = renderer.create_vertex_array()?;

        //create quad data and get a buffer id
        let geom_buffer_id = renderer.create_buffer()?;
        let tex_buffer_id = renderer.create_buffer()?;

        renderer.upload_buffer(
            geom_buffer_id,
            BufferData::new(
                &QUAD_GEOM_UNIT,
                BufferTarget::ArrayBuffer,
                BufferUsage::StaticDraw,
            ),
        )?;
        //renderer.upload_buffer_to_attribute_name(
            //geom_id,
            //BufferData::new(
                //&QUAD_GEOM_UNIT,
                //BufferTarget::ArrayBuffer,
                //BufferUsage::StaticDraw,
                //),
                //"a_vertex",
                //&AttributeOptions::new(2, DataType::Float),
                //)?;


        renderer.assign_vertex_array(
            vao_id,
            None,
            &[
                VertexArray {
                    attribute: NameOrLoc::Name("a_geom_vertex"),
                    buffer_id: geom_buffer_id,
                    opts: AttributeOptions::new(2, DataType::Float),
                },
                VertexArray {
                    attribute: NameOrLoc::Name("a_tex_vertex"),
                    buffer_id: tex_buffer_id,
                    opts: AttributeOptions::new(2, DataType::Float),
                }
            ],
        )?;

        Ok(Self { 
            renderer, 
            vao_id, 
            tex_buffer_id,
            geom_buffer_id,
            texture_program_id, 
            picker_program_id, 
            picker: None
        } )
    }

    pub fn create_img_texture(&mut self, img: &HtmlImageElement) -> Result<Id, awsm_web::errors::Error> {
        let id = self.create_texture()?;
        self.assign_simple_texture(
            id,
            TextureTarget::Texture2d,
            &SimpleTextureOptions {
                pixel_format: PixelFormat::Rgba,
                ..SimpleTextureOptions::default()
            },
            &WebGlTextureSource::ImageElement(&img),
        )?;

        Ok(id)
    }

    pub fn resize(&mut self, camera: &mut UniqueViewMut<Camera>, strategy: ResizeStrategy) {
        if self.renderer.resize(strategy) {
            if let Some(picker) = self.picker.take() {
                picker.destroy(&mut self.renderer).unwrap_ext();
            }
            let (_, _, width, height) = self.renderer.get_viewport();
            self.picker = Some(ScenePicker::new(&mut self.renderer, width, height).unwrap_ext());
        }
    }

    pub fn get_picker_index(&mut self, x: u32, y: u32) -> Result<Option<u32>, awsm_web::errors::Error> {
        match self.picker.as_ref() {
            None => Ok(None),
            Some(picker) => {
                let color = picker.get_color(&mut self.renderer, x, y)?;
                let index = ((color[0] as u32) << 16) | ((color[1] as u32) << 8) | (color[2] as u32);
                if index > 0 {
                    Ok(Some(index - 1))
                } else {
                    Ok(None)
                }
            }
        }
    }

    fn draw_sprite_sheet(&mut self,
        sprite_sheet: &SpriteSheet,
        sprite_order: &[EntityId],
        camera: &UniqueView<Camera>,
        world_transforms: &View<WorldTransform>,
        sprite_cells: &View<SpriteCell>, 
        interactables: &View<Interactable>,
        pass: Pass,
    ) -> Result<(), awsm_web::errors::Error> {

        let program_id = match pass {
            Pass::Picker => {
                if let Some(picker) = &mut self.picker {
                    picker.start(&mut self.renderer)?;
                    self.gl.clear_color(0.0, 0.0, 0.0, 0.0);
                    self.picker_program_id
                } else {
                    return Ok(());
                }
            },
            Pass::Forward => {
                self.gl.clear_color(0.3, 0.3, 0.3, 1.0);
                self.texture_program_id
            }
        };

        self.clear(&[
            BufferMask::ColorBufferBit,
            BufferMask::DepthBufferBit,
        ]);

        self.set_depth_mask(false);
        self.toggle(GlToggle::Blend, true);
        self.toggle(GlToggle::DepthTest, true);
        self.set_blend_func(BlendFactor::SrcAlpha, BlendFactor::OneMinusSrcAlpha);


        self.activate_program(program_id)?;
        self.activate_texture_for_sampler_name(sprite_sheet.texture_id, "u_sampler")?;

        let (_,_,viewport_width,viewport_height) = self.get_viewport();
        self.upload_uniform_mat_4_name("u_camera", &camera.get_matrix(viewport_width as f64, viewport_height as f64).as_slice())?;


        //renderer.upload_uniform_mat_4("u_size", &scaling_mat.as_slice())?;

        for entity_id in sprite_order {
            if let Ok((transform, sprite_cell, interactable)) = (world_transforms, sprite_cells, interactables).get(*entity_id) {
                self.draw_cell_geom(transform, &sprite_sheet, sprite_cell)?;
                match pass {
                    Pass::Picker => {
                        let index = interactable.0 + 1;
                        let divisor = 0xFF as f32;
                        let r = (0xFF & (index >> 16)) as f32 / divisor;
                        let g = (0xFF & (index >> 8)) as f32 / divisor;
                        let b = (0xFF & index) as f32 / divisor; 
                        self.upload_uniform_fvals_4_name("u_color", (r,g,b,1.0))?;
                    },
                    Pass::Forward => {
                    }
                }

                self.activate_vertex_array(self.vao_id).unwrap_ext();
                self.draw_arrays(BeginMode::TriangleStrip, 0, 4);
            }
        }


        if pass == Pass::Picker {
            if let Some(picker) = &mut self.picker {
                picker.finish(&mut self.renderer)?;
            }
        }
        Ok(())
    }


    fn draw_cell_geom(&mut self, transform: &WorldTransform, sheet: &SpriteSheet, cell: &SpriteCell) -> Result<(), awsm_web::errors::Error> {
        let mut scratch:[f32;16] = [0.0;16];

        transform.write_to_vf32(&mut scratch);
        self.upload_uniform_mat_4_name("u_model", &scratch)?;
        //self.upload_uniform_fvals_2_name("u_full_size", (sheet.width, sheet.height))?;
        self.upload_uniform_fvals_2_name("u_cell_size", (cell.width, cell.height))?;
        //self.upload_uniform_fvals_2_name("u_coord", (cell.x, cell.y))?;

        self.upload_buffer(
                self.tex_buffer_id,
                BufferData::new(
                    cell.uvs,
                    BufferTarget::ArrayBuffer,
                    BufferUsage::DynamicDraw,
                ),
        ).unwrap_throw();
        Ok(())
    }
}


pub fn render_sys(
    mut renderer: RendererViewMut, 
    pieces_order: UniqueView<PiecesOrder>, 
    puzzle_sheet: UniqueView<PuzzleSheet>, 
    camera: UniqueView<Camera>, 
    world_transforms: View<WorldTransform>, 
    sprite_cells:View<SpriteCell>,
    interactables:View<Interactable>,
) {
    renderer.draw_sprite_sheet(&puzzle_sheet, &pieces_order, &camera, &world_transforms, &sprite_cells, &interactables, Pass::Picker).unwrap_ext();
    renderer.draw_sprite_sheet(&puzzle_sheet, &pieces_order, &camera, &world_transforms, &sprite_cells, &interactables, Pass::Forward).unwrap_ext();
}

