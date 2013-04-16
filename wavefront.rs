extern mod sdl;
extern mod opengles;

use opengles::gl2;
use opengles::gl2::{GLuint, GLint, GLenum};

struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector2 {
    fn from_str_vec(v: &[~str]) -> Vector2 {
        Vector2::from_vec(do v.map |s| {
            match f32::from_str(*s) {
                Some(f) => f,
                None => 0.0,
            }
        })
    }

    fn from_vec(v: &[f32]) -> Vector2 {
        assert!(v.len() == 2);
        Vector2 {
            x: v[0],
            y: v[1],
        }
    }
}

struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector3 {
    fn from_str_vec(v: &[~str]) -> Vector3 {
        Vector3::from_vec(do v.map |s| {
            match f32::from_str(*s) {
                Some(f) => f,
                None => 0.0,
            }
        })
    }

    fn from_vec(v: &[f32]) -> Vector3 {
        assert!(v.len() == 3);
        Vector3 {
            x: v[0],
            y: v[1],
            z: v[2],
        }
    }
}

struct Vertex {
    position: u16,
    texcoord: u16,
    normal: u16,
}

impl Vertex {
    fn from_str(v: &str) -> Vertex {
        let mut idx = ~[];
        for str::each_split_char(v, '/') |s| {
            idx.push(match u16::from_str(s) {
                Some(i) => i-1,
                None => 0,
            });
        }
        Vertex::from_vec(idx)
    }

    fn from_vec(v: &[u16]) -> Vertex {
        assert!(v.len() == 3);
        Vertex {
            position: v[0],
            texcoord: v[1],
            normal: v[2],
        }
    }
}

struct Face {
    vertices: [Vertex, ..3],
}


impl Face {
    fn from_str_vec(v: &[~str]) -> Face {
        Face::from_vec(do v.map |s| {
            Vertex::from_str(*s)
        })
    }

    fn from_vec(v: &[Vertex]) -> Face {
        assert!(v.len() == 3);
        Face {
            vertices: [v[0], v[1], v[2]],
        }
    }
}

struct Mesh {
    name: ~str,
    vertices: ~[Vector3],
    texcoords: ~[Vector2],
    normals: ~[Vector3],
    faces: ~[Face],
}

fn load_wavefront(filename: &str) -> Mesh {
    let path = Path(filename);
    let r = match io::file_reader(&path) {
        Err(err) => fail!(fmt!("couldn't open %s for read: %s", filename, err)),
        Ok(r) => r,
    };

    let mut mesh = Mesh {
        name: ~"",
        vertices: ~[],
        texcoords: ~[],
        normals: ~[],
        faces: ~[],
    };

    r.each_line(|line| {
        let mut split = ~[];
        for str::each_split_char_nonempty(line, ' ') |s| {
            split.push(str::from_slice(s));
        }

        match vec::shift(&mut split) {
            ~"o"  => mesh.name = str::from_slice(split[0]),

            ~"v"  => mesh.vertices.push(Vector3::from_str_vec(split)),
            ~"vt" => mesh.texcoords.push(Vector2::from_str_vec(split)),
            ~"vn" => mesh.normals.push(Vector3::from_str_vec(split)),

            ~"f"  => mesh.faces.push(Face::from_str_vec(split)),

            _ => {}
            //unk => io::println(fmt!("unknown tag: %s", unk)),
        }
        true
    });

    mesh
}

fn make_buffer<T>(target: GLenum, data: &[T]) -> GLuint {
    let buffers = gl2::gen_buffers(1);
    let buffer = *(buffers.head());
    gl2::bind_buffer(target, buffer);
    gl2::buffer_data(target, data, gl2::STATIC_DRAW);
    buffer
}

fn make_shader(ty: GLenum, filename: &str) -> GLuint {
    let path = Path(filename);
    let r = match io::file_reader(&path) {
        Err(err) => fail!(fmt!("couldn't open %s for read: %s", filename, err)),
        Ok(r) => r,
    };
    let source = ~[r.read_whole_stream()];

    let shader = gl2::create_shader(ty);
    gl2::shader_source(shader, source);
    gl2::compile_shader(shader);

    match gl2::get_shader_iv(shader, gl2::COMPILE_STATUS) as bool {
        false => fail!(fmt!("failed to compile %s: %s", filename, gl2::get_shader_info_log(shader))),
        true => {}
    };

    shader
}

fn make_program(vertex_shader: GLuint, fragment_shader: GLuint) -> GLuint {
    let program = gl2::create_program();
    gl2::attach_shader(program, vertex_shader);
    gl2::attach_shader(program, fragment_shader);
    gl2::link_program(program);

    match gl2::get_program_iv(program, gl2::LINK_STATUS) as bool {
        false => fail!(fmt!("failed to link shader program: %s", gl2::get_program_info_log(program))),
        true => {}
    };

    program
}


fn check_gl_error() {
    match gl2::get_error() {
        gl2::NO_ERROR => {},
        e => io::println(fmt!("GL error: %s", match e {
            gl2::INVALID_ENUM                  => ~"INVALID_ENUM",
            gl2::INVALID_VALUE                 => ~"INVALID_VALUE",
            gl2::INVALID_OPERATION             => ~"INVALID_OPERATION",
            gl2::STACK_OVERFLOW                => ~"STACK_OVERFLOW",
            gl2::STACK_UNDERFLOW               => ~"STACK_UNDERFLOW",
            gl2::OUT_OF_MEMORY                 => ~"OUT_OF_MEMORY",
            gl2::INVALID_FRAMEBUFFER_OPERATION => ~"INVALID_FRAMEBUFFER_OPERATION",
            _ => fmt!("unknown 0x%x", e as uint),
        }))
    }
}


fn main() {
    #[main];

    let mesh = load_wavefront("ship.obj");

    do sdl::start {
        sdl::init([sdl::InitVideo]);

        let info = sdl::video::get_video_info();
        let (rs, gs, bs) = match info.format.bpp {
            16      => (5, 6, 5),
            24 | 32 => (8, 8, 8),
            _       => fail!(fmt!("invalid pixel depth: %d bpp", info.format.bpp as int))
        };

        sdl::gl::set_attribute(sdl::gl::RedSize, rs);
        sdl::gl::set_attribute(sdl::gl::GreenSize, gs);
        sdl::gl::set_attribute(sdl::gl::BlueSize, bs);
        sdl::gl::set_attribute(sdl::gl::DepthSize, 24);
        sdl::gl::set_attribute(sdl::gl::DoubleBuffer, 1);
        sdl::gl::set_attribute(sdl::gl::SwapControl, 1);

        match sdl::video::set_video_mode(1440, 900, info.format.bpp as int, [], [sdl::video::OpenGL]) {
            Ok(_)    => {},
            Err(err) => fail!(fmt!("failed to set video mode: %s", err))
        };

        sdl::wm::set_caption("wavefront", "wavefront");

        let program = make_program(
            make_shader(gl2::VERTEX_SHADER, "wavefront.v.glsl"),
            make_shader(gl2::FRAGMENT_SHADER, "wavefront.f.glsl")
        );
        let attr_position = gl2::get_attrib_location(program, ~"position");

        let vertex_buffer = make_buffer(gl2::ARRAY_BUFFER,
            vec::concat(do mesh.vertices.map |v| { ~[v.x,v.y,v.z] }));
        let element_buffer = make_buffer(gl2::ELEMENT_ARRAY_BUFFER,
            vec::concat(do mesh.faces.map |f| { ~[f.vertices[0].position, f.vertices[1].position, f.vertices[2].position] }));

        loop main: {
            loop event: {
                match sdl::event::poll_event() {
                    sdl::event::QuitEvent => break main,
                    sdl::event::NoEvent   => break event,
                    sdl::event::KeyEvent(sdl::event::EscapeKey, true, _, _) => break main,
                    _                     => {}
                }
            }

            gl2::use_program(program);

            gl2::bind_buffer(gl2::ARRAY_BUFFER, vertex_buffer);
            gl2::vertex_attrib_pointer_f32(attr_position as GLuint, 3, false, 0, 0);
            gl2::enable_vertex_attrib_array(attr_position as GLuint);

            gl2::bind_buffer(gl2::ELEMENT_ARRAY_BUFFER, element_buffer);

            gl2::draw_elements(gl2::TRIANGLES, (mesh.faces.len()*3) as GLint, gl2::UNSIGNED_SHORT, None);

            gl2::bind_buffer(gl2::ARRAY_BUFFER, 0);
            gl2::bind_buffer(gl2::ELEMENT_ARRAY_BUFFER, 0);

            sdl::gl::swap_buffers();

            check_gl_error();
        }

        sdl::quit();
    }
}
