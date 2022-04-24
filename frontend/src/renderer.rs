pub mod spritesheet;
pub mod picker;
use picker::*;

use crate::{media::{PuzzleInfo, MediaView}, buffers::DataBuffers};
use spritesheet::SpriteSheetTextureId;
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

pub type RendererView<'a> = NonSendSync<UniqueView<'a, SceneRenderer>>;
pub type RendererViewMut<'a> = NonSendSync<UniqueViewMut<'a, SceneRenderer>>;

#[derive(Component)]
pub struct SceneRenderer {
    renderer: WebGl1Renderer,
    picker_program_id: Id,
    forward_program_id: Id,
    outline_program_id: Id,
    forward_vao_id: Id,
    picker_vao_id: Id,
    outline_vao_id: Id,
    pub geom_buffer_id: Id,
    pub outline_buffer_id: Id,
    pub tex_buffer_id: Id,
    pub color_buffer_id: Id,
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
    Forward,
    Outline,
    Debug
}

impl SceneRenderer {
    pub fn new (ctx: WebGlRenderingContext, media: &Media) -> Result<Self, awsm_web::errors::Error> {
        let mut renderer = WebGl1Renderer::new(ctx).unwrap_ext();

        //This demo is specifically using webgl1, which needs to register the extension
        //Everything else is the same API as webgl2 :)
        renderer.register_extension_instanced_arrays()?;
        renderer.register_extension_vertex_array()?;
      
        //create buffer ids
        let model_buffer_id = renderer.create_buffer()?;
        let geom_buffer_id = renderer.create_buffer()?;
        let outline_buffer_id = renderer.create_buffer()?;
        let tex_buffer_id = renderer.create_buffer()?;
        let color_buffer_id = renderer.create_buffer()?;

        // Setup forward
        let forward_program_id = {
            let vertex_id = renderer.compile_shader(&media.forward_vertex_shader, ShaderType::Vertex)?;
            let fragment_id = renderer.compile_shader(&media.forward_fragment_shader, ShaderType::Fragment)?;

            renderer.compile_program(&[vertex_id, fragment_id])?
        };

        let forward_vao_id = renderer.create_vertex_array()?;

        renderer.assign_vertex_array(
            forward_vao_id,
            None,
            &[
                VertexArray {
                    attribute: NameOrLoc::Name("a_geom_vertex"),
                    buffer_id: geom_buffer_id,
                    opts: AttributeOptions::new(3, DataType::Float),
                },
                VertexArray {
                    attribute: NameOrLoc::Name("a_tex_vertex"),
                    buffer_id: tex_buffer_id,
                    opts: AttributeOptions::new(2, DataType::Float),
                }
            ],
        )?;

        // Setup picker
        let picker_program_id = {
            let vertex_id = renderer.compile_shader(&media.picker_vertex_shader, ShaderType::Vertex)?;
            let fragment_id = renderer.compile_shader(&media.picker_fragment_shader, ShaderType::Fragment)?;

            renderer.compile_program(&[vertex_id, fragment_id])?
        };

        let picker_vao_id = renderer.create_vertex_array()?;

        renderer.assign_vertex_array(
            picker_vao_id,
            None,
            &[
                VertexArray {
                    attribute: NameOrLoc::Name("a_geom_vertex"),
                    buffer_id: geom_buffer_id,
                    opts: AttributeOptions::new(3, DataType::Float),
                },
                VertexArray {
                    attribute: NameOrLoc::Name("a_tex_vertex"),
                    buffer_id: tex_buffer_id,
                    opts: AttributeOptions::new(2, DataType::Float),
                },
                VertexArray {
                    attribute: NameOrLoc::Name("a_color_vertex"),
                    buffer_id: color_buffer_id,
                    opts: AttributeOptions::new(4, DataType::Float),
                }
            ],
        )?;

        // Setup outline
        let outline_program_id = {
            let vertex_id = renderer.compile_shader(&media.outline_vertex_shader, ShaderType::Vertex)?;
            let fragment_id = renderer.compile_shader(&media.outline_fragment_shader, ShaderType::Fragment)?;

            renderer.compile_program(&[vertex_id, fragment_id])?
        };

        let outline_vao_id = renderer.create_vertex_array()?;

        renderer.assign_vertex_array(
            outline_vao_id,
            None,
            &[
                VertexArray {
                    attribute: NameOrLoc::Name("a_geom_vertex"),
                    buffer_id: outline_buffer_id,
                    opts: AttributeOptions::new(3, DataType::Float),
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
            forward_vao_id, 
            picker_vao_id, 
            outline_vao_id, 
            tex_buffer_id,
            geom_buffer_id,
            color_buffer_id,
            outline_buffer_id,
            forward_program_id, 
            picker_program_id, 
            outline_program_id, 
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
        sprite_sheet_texture_id: Id,
        camera: &UniqueView<Camera>,
        data_buffers: &DataBuffers,
        n_pieces: u32,
        pass: Pass,
    ) -> Result<(), awsm_web::errors::Error> {


        self.toggle(GlToggle::Blend, true);
        self.set_blend_func(BlendFactor::SrcAlpha, BlendFactor::OneMinusSrcAlpha);
        self.set_depth_func(awsm_web::webgl::CmpFunction::Less);
        self.set_depth_mask(true);
        self.toggle(GlToggle::DepthTest, true);

        let program_id = match pass {
            Pass::Picker | Pass::Debug => {
                self.picker_program_id
            },
            Pass::Forward => {
                self.forward_program_id
            },
            Pass::Outline => {
                self.outline_program_id
            }
        };

        self.activate_program(program_id)?;
        self.activate_texture_for_sampler_name(sprite_sheet_texture_id, "u_sampler")?;
        let (_,_,viewport_width,viewport_height) = self.get_viewport();
        self.upload_uniform_mat_4_name("u_camera", &camera.get_matrix(viewport_width as f64, viewport_height as f64).as_slice())?;

        match pass {
            Pass::Picker | Pass::Debug => {
                if pass == Pass::Picker {
                    self.picker.as_mut().unwrap_ext().start(&mut self.renderer)?;
                }
                self.gl.clear_color(0.0, 0.0, 0.0, 0.0);
                self.activate_vertex_array(self.picker_vao_id).unwrap_ext();
            },
            Pass::Forward => {
                self.gl.clear_color(0.3, 0.3, 0.3, 1.0);
                self.activate_vertex_array(self.forward_vao_id).unwrap_ext();
            }
            Pass::Outline => {
                self.gl.clear_color(0.3, 0.3, 0.3, 1.0);
                self.activate_vertex_array(self.outline_vao_id).unwrap_ext();
            }
        };

        if pass != Pass::Outline {
            //Clear with the selected color
            self.clear(&[
                BufferMask::ColorBufferBit,
                BufferMask::DepthBufferBit,
            ]);
        }


        // 2 attribute floats per vertex
        let pieces_vertex_len = n_pieces * 6;
        self.draw_arrays(BeginMode::Triangles, 0, pieces_vertex_len);

        if pass == Pass::Picker {
            self.picker.as_mut().unwrap_ext().finish(&mut self.renderer)?;
        }

        Ok(())
    }
}


pub fn render_sys(
    mut renderer: RendererViewMut, 
    sprite_sheet_texture_id: UniqueView<SpriteSheetTextureId>,
    data_buffers: UniqueView<DataBuffers>,
    interactables: View<Interactable>,
    camera: UniqueView<Camera>, 
) {
    let n_pieces = interactables.iter().count() as u32;
    renderer.draw_sprite_sheet(sprite_sheet_texture_id.0, &camera, &data_buffers, n_pieces, Pass::Picker).unwrap_ext();
    renderer.draw_sprite_sheet(sprite_sheet_texture_id.0, &camera, &data_buffers, n_pieces, Pass::Forward).unwrap_ext();
    renderer.draw_sprite_sheet(sprite_sheet_texture_id.0, &camera, &data_buffers, n_pieces, Pass::Outline).unwrap_ext();
    //renderer.draw_sprite_sheet(sprite_sheet_texture_id.0, &camera, &data_buffers, n_pieces, Pass::Debug).unwrap_ext();
}

