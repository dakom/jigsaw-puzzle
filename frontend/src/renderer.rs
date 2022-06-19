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

#[derive(Component, Unique)]
pub struct SceneRenderer {
    renderer: WebGl1Renderer,
    pub programs: Programs,
    pub vaos: Vaos,
    pub buffers: Buffers,
    picker: Option<ScenePicker>
}

pub struct Programs {
    picker: Id,
    piece_draw: Id,
    piece_outline: Id,
    quad: Id,
}

pub struct Vaos {
    picker: Id,
    piece_active: Id,
    piece_bg: Id,
    border: Id,
}

pub struct Buffers {
    pub piece_active: Id,
    pub piece_bg: Id,
    pub border: Id,
    pub texture: Id,
    pub picker_color: Id
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
    PiecesBg,
    PiecesActive,
    PiecesOutline,
}

impl SceneRenderer {
    pub fn new (ctx: WebGlRenderingContext, media: &Media) -> Result<Self, awsm_web::errors::Error> {
        let mut renderer = WebGl1Renderer::new(ctx).unwrap_ext();

        //This demo is specifically using webgl1, which needs to register the extension
        //Everything else is the same API as webgl2 :)
        renderer.register_extension_instanced_arrays()?;
        renderer.register_extension_vertex_array()?;
      
        //create buffer ids
        let buffers = Buffers {
            piece_active: renderer.create_buffer()?,
            piece_bg: renderer.create_buffer()?,
            texture: renderer.create_buffer()?,
            picker_color: renderer.create_buffer()?,
            border: renderer.create_buffer()?,
        };

        //create programs
        let programs = Programs {

            picker: {
                let vertex_id = renderer.compile_shader(&media.picker_vertex_shader, ShaderType::Vertex)?;
                let fragment_id = renderer.compile_shader(&media.picker_fragment_shader, ShaderType::Fragment)?;

                renderer.compile_program(&[vertex_id, fragment_id])?
            },

            piece_draw: {
                let vertex_id = renderer.compile_shader(&media.piece_vertex_shader, ShaderType::Vertex)?;
                let fragment_id = renderer.compile_shader(&media.piece_fragment_shader, ShaderType::Fragment)?;

                renderer.compile_program(&[vertex_id, fragment_id])?
            },

            piece_outline: {
                let vertex_id = renderer.compile_shader(&media.outline_vertex_shader, ShaderType::Vertex)?;
                let fragment_id = renderer.compile_shader(&media.outline_fragment_shader, ShaderType::Fragment)?;

                renderer.compile_program(&[vertex_id, fragment_id])?
            },

            quad: {
                let vertex_id = renderer.compile_shader(&media.quad_vertex_shader, ShaderType::Vertex)?;
                let fragment_id = renderer.compile_shader(&media.quad_fragment_shader, ShaderType::Fragment)?;

                renderer.compile_program(&[vertex_id, fragment_id])?
            },

        };

        //create vertex array objects


        let vaos = Vaos {
            picker: renderer.create_vertex_array()?,
            piece_active: renderer.create_vertex_array()?,
            piece_bg: renderer.create_vertex_array()?,
            border: renderer.create_vertex_array()?,
        };
        

        // assign vaos
        // for the lookups to work by name, might as well just bind the program
        // but it might be nicer to explicitly set via hardcoded_attribute_locations....
        renderer.activate_program(programs.picker)?;
        renderer.assign_vertex_array(
            vaos.picker,
            None,
            &[
                VertexArray {
                    attribute: NameOrLoc::Name("a_geom_vertex"),
                    buffer_id: buffers.piece_active,
                    opts: AttributeOptions::new(3, DataType::Float),
                },
                VertexArray {
                    attribute: NameOrLoc::Name("a_tex_vertex"),
                    buffer_id: buffers.texture,
                    opts: AttributeOptions::new(2, DataType::Float),
                },
                VertexArray {
                    attribute: NameOrLoc::Name("a_color_vertex"),
                    buffer_id: buffers.picker_color,
                    opts: AttributeOptions::new(4, DataType::Float),
                }
            ],
        )?;

        renderer.activate_program(programs.piece_draw)?;
        renderer.assign_vertex_array(
            vaos.piece_active,
            None,
            &[
                VertexArray {
                    attribute: NameOrLoc::Name("a_geom_vertex"),
                    buffer_id: buffers.piece_active,
                    opts: AttributeOptions::new(3, DataType::Float),
                },
                VertexArray {
                    attribute: NameOrLoc::Name("a_tex_vertex"),
                    buffer_id: buffers.texture,
                    opts: AttributeOptions::new(2, DataType::Float),
                }
            ],
        )?;

        renderer.assign_vertex_array(
            vaos.piece_bg,
            None,
            &[
                VertexArray {
                    attribute: NameOrLoc::Name("a_geom_vertex"),
                    buffer_id: buffers.piece_bg,
                    opts: AttributeOptions::new(3, DataType::Float),
                },
                VertexArray {
                    attribute: NameOrLoc::Name("a_tex_vertex"),
                    buffer_id: buffers.texture,
                    opts: AttributeOptions::new(2, DataType::Float),
                }
            ],
        )?;

        renderer.activate_program(programs.quad)?;
        renderer.assign_vertex_array(
            vaos.border,
            None,
            &[
                VertexArray {
                    attribute: NameOrLoc::Name("a_geom_vertex"),
                    buffer_id: buffers.border,
                    opts: AttributeOptions::new(3, DataType::Float),
                },
            ],
        )?;



        Ok(Self { 
            renderer, 
            programs,
            buffers,
            vaos,
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


    fn draw_border(&mut self,
        camera: &UniqueView<Camera>,
        data_buffers: &DataBuffers,
    ) -> Result<(), awsm_web::errors::Error> {

        let program_id = self.programs.quad;
        self.activate_program(program_id)?;

        let (_,_,viewport_width,viewport_height) = self.get_viewport();
        self.upload_uniform_mat_4_name("u_camera", &camera.get_matrix(viewport_width as f64, viewport_height as f64).as_slice())?;
        self.upload_uniform_fvals_4_name("u_color", (1.0, 1.0, 1.0, 0.3));

        self.activate_vertex_array(self.vaos.border)?;

        self.draw_arrays(BeginMode::Triangles, 0, (data_buffers.border_vertices.len() / 3) as u32);

        Ok(())
    }

    fn draw_sprite_sheet(&mut self,
        sprite_sheet_texture_id: Id,
        media: &Media,
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
            Pass::Picker => {
                self.programs.picker
            },
            Pass::PiecesActive | Pass::PiecesBg => {
                self.programs.piece_draw
            },
            Pass::PiecesOutline => {
                self.programs.piece_outline
            },
        };

        self.activate_program(program_id)?;
        self.activate_texture_for_sampler_name(sprite_sheet_texture_id, "u_sampler")?;
        let (_,_,viewport_width,viewport_height) = self.get_viewport();
        self.upload_uniform_mat_4_name("u_camera", &camera.get_matrix(viewport_width as f64, viewport_height as f64).as_slice())?;

        match pass {
            Pass::Picker => {
                self.picker.as_mut().unwrap_ext().start(&mut self.renderer)?;
                self.draw_clear(0.0, 0.0, 0.0, 0.0);
            },
            Pass::PiecesActive => {
                self.upload_uniform_fval_name("u_alpha", 1.0);
            },
            Pass::PiecesBg => {
                self.upload_uniform_fval_name("u_alpha", 0.1);
            },
            Pass::PiecesOutline => {
                self.upload_uniform_fvals_2_name("u_size", (media.puzzle_info.atlas_width as f32, media.puzzle_info.atlas_height as f32));
            },
            _ => {}
        };

        let vao_id = match pass {
            Pass::Picker => self.vaos.picker,
            Pass::PiecesActive => self.vaos.piece_active,
            Pass::PiecesBg | Pass::PiecesOutline => self.vaos.piece_bg,
        };

        self.activate_vertex_array(vao_id)?;

        // 2 triangles per piece, 3 floats per vertex
        let pieces_vertex_len = n_pieces * 6;
        self.draw_arrays(BeginMode::Triangles, 0, pieces_vertex_len);

        if pass == Pass::Picker {
            self.picker.as_mut().unwrap_ext().finish(&mut self.renderer)?;
        }

        Ok(())
    }


    fn draw_clear(&mut self, r: f32, g: f32, b: f32, a: f32) { 
        self.gl.clear_color(r, g, b, a);

        self.clear(&[
            BufferMask::ColorBufferBit,
            BufferMask::DepthBufferBit,
        ]);

    }

}


pub fn render_sys(
    mut renderer: RendererViewMut, 
    media: MediaView,
    sprite_sheet_texture_id: UniqueView<SpriteSheetTextureId>,
    data_buffers: UniqueView<DataBuffers>,
    interactables: View<Interactable>,
    camera: UniqueView<Camera>, 
) {
    //log::info!("{} {}", media.puzzle_info.atlas_width, media.puzzle_info.atlas_height);

    let n_pieces = interactables.iter().count() as u32;
    renderer.draw_sprite_sheet(sprite_sheet_texture_id.0, &media, &camera, &data_buffers, n_pieces, Pass::Picker).unwrap_ext();
    // clears the main drawing buffer - picker is cleared when its framebuffer is set
    renderer.draw_clear(0.3, 0.3, 0.3, 1.0);
    //renderer.draw_sprite_sheet(sprite_sheet_texture_id.0, &media, &camera, &data_buffers, n_pieces, Pass::PiecesOutline).unwrap_ext();
    renderer.draw_sprite_sheet(sprite_sheet_texture_id.0, &media, &camera, &data_buffers, n_pieces, Pass::PiecesBg).unwrap_ext();
    renderer.draw_sprite_sheet(sprite_sheet_texture_id.0, &media, &camera, &data_buffers, n_pieces, Pass::PiecesActive).unwrap_ext();
    renderer.draw_border(&camera, &data_buffers).unwrap_ext();
}

