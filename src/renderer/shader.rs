use std::ffi::CString;
use std::fs;
use std::ptr;

pub struct Shader {
    pub id: u32,
}


fn load_source(path: &str) -> CString {
    let src = fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Failed to read shader file: {}", path));

    CString::new(src)
        .expect("Shader source contains null bytes")
}

unsafe fn compile_shader(src: &CString, kind: u32, path: &str) -> u32 {
    let shader = gl::CreateShader(kind);
    gl::ShaderSource(shader, 1, &src.as_ptr(), ptr::null());
    gl::CompileShader(shader);

    let mut success = 0;
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

    if success == 0 {
        let mut len = 0;
        gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);

        let mut buffer = vec![0u8; len as usize];
        gl::GetShaderInfoLog(
            shader,
            len,
            ptr::null_mut(),
            buffer.as_mut_ptr() as *mut _,
        );

        panic!(
            "Shader compilation failed ({}):\n{}",
            path,
            String::from_utf8_lossy(&buffer)
        );
    }

    shader
}

impl Shader {
    pub fn from_files(vertex_path: &str, fragment_path: &str) -> Shader {
        let vertex_src = load_source(vertex_path);
        let fragment_src = load_source(fragment_path);
        
        unsafe {
            let vertex_shader = compile_shader(&vertex_src, gl::VERTEX_SHADER, vertex_path);
            let fragment_shader = compile_shader(&fragment_src, gl::FRAGMENT_SHADER, fragment_path);

            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            Self { id: program }
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}