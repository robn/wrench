struct Vector2 {
    x: float,
    y: float,
}

impl Vector2 {
    fn from_str_vec(v: &[~str]) -> Vector2 {
        Vector2::from_vec(do v.map |s| {
            match float::from_str(*s) {
                Some(f) => f,
                None => 0.0,
            }
        })
    }

    fn from_vec(v: &[float]) -> Vector2 {
        assert!(v.len() == 2);
        Vector2 {
            x: v[0],
            y: v[1],
        }
    }
}

struct Vector3 {
    x: float,
    y: float,
    z: float,
}

impl Vector3 {
    fn from_str_vec(v: &[~str]) -> Vector3 {
        Vector3::from_vec(do v.map |s| {
            match float::from_str(*s) {
                Some(f) => f,
                None => 0.0,
            }
        })
    }

    fn from_vec(v: &[float]) -> Vector3 {
        assert!(v.len() == 3);
        Vector3 {
            x: v[0],
            y: v[1],
            z: v[2],
        }
    }
}

struct Vertex {
    position: uint,
    texcoord: uint,
    normal: uint,
}

impl Vertex {
    fn from_str_vec(v: &str) -> Vertex {
        let mut idx = ~[];
        for str::each_split_char(v, '/') |s| {
            idx.push(match uint::from_str(s) {
                Some(i) => i,
                None => 0,
            });
        }
        Vertex::from_vec(idx)
    }

    fn from_vec(v: &[uint]) -> Vertex {
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
            Vertex::from_str_vec(*s)
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

fn main() {
    #[main];

    let mesh = load_wavefront("ship.obj");
    io::println(fmt!("%?", mesh));
}
