use indicatif::ProgressBar;
use shaderc::CompilationArtifact;
use shaderc::CompileOptions;
use shaderc::Compiler;
use shaderc::ShaderKind;
use std::fs;
use std::fs::File;
use std::path::Path;
use tar::Builder;
use tar::Header;

use crate::error::ErrorKind;
use crate::error::ErrorType;
use crate::error::Result;

pub fn import_shader(in_path: &Path, out_path: &Path) -> Result<()> {
    println!("Compiling {:?}", in_path);

    let progress = ProgressBar::new(6);

    let shader_src = fs::read_to_string(in_path)?;
    progress.inc(1);

    let vert_bin = compile_vert(&shader_src)?;
    progress.inc(1);
    let frag_bin = compile_frag(&shader_src)?;
    progress.inc(1);

    // compress spirv shaders
    let out_path = out_path.with_extension("shader");
    let out_file = File::create(out_path)?;
    let mut archive = Builder::new(out_file);
    progress.inc(1);

    let mut vert_header = Header::new_gnu();
    vert_header.set_size(vert_bin.as_binary_u8().len() as u64);
    vert_header.set_cksum();
    archive.append_data(&mut vert_header, "vert.spv", vert_bin.as_binary_u8())?;
    progress.inc(1);

    let mut frag_header = Header::new_gnu();
    frag_header.set_size(frag_bin.as_binary_u8().len() as u64);
    frag_header.set_cksum();
    archive.append_data(&mut frag_header, "frag.spv", frag_bin.as_binary_u8())?;
    progress.inc(1);

    progress.finish_with_message("done");
    Ok(())
}

fn compile_vert(src: &str) -> Result<CompilationArtifact> {
    let vert_glsl = include_str!("../glsl/vert.glsl");
    let objects_glsl = include_str!("../glsl/objects.glsl");

    // create real glsl code
    let real_src = format!(
        "#version 450\n{}\n{}\n{}\nvoid main() {{ vertex(); }}",
        objects_glsl, vert_glsl, src
    );

    // compile glsl to spirv
    let mut compiler = Compiler::new().ok_or(ErrorType::Internal(ErrorKind::NoCompiler))?;
    let mut options = CompileOptions::new().ok_or(ErrorType::Internal(ErrorKind::NoCompiler))?;
    options.add_macro_definition("VERTEX", Some("1"));
    let artifact = compiler.compile_into_spirv(
        &real_src,
        ShaderKind::Vertex,
        "shader.vert",
        "main",
        Some(&options),
    )?;
    Ok(artifact)
}

fn compile_frag(src: &str) -> Result<CompilationArtifact> {
    let frag_c_glsl = include_str!("../glsl/frag.glsl");
    let frag_d_glsl = include_str!("../glsl/frag-d.glsl");
    let objects_glsl = include_str!("../glsl/objects.glsl");

    // create real glsl code
    let is_depth_frag = src.find("out_color").is_none();
    let frag_glsl = match is_depth_frag {
        true => frag_d_glsl,
        false => frag_c_glsl,
    };

    let real_src = format!(
        "#version 450\n{}\n{}\n{}\nvoid main() {{ fragment(); }}",
        objects_glsl, frag_glsl, src
    );

    // compile glsl to spirv
    let mut compiler = Compiler::new().ok_or(ErrorType::Internal(ErrorKind::NoCompiler))?;
    let mut options = CompileOptions::new().ok_or(ErrorType::Internal(ErrorKind::NoCompiler))?;
    options.add_macro_definition("FRAGMENT", Some("1"));
    let artifact = compiler.compile_into_spirv(
        &real_src,
        ShaderKind::Fragment,
        "shader.frag",
        "main",
        Some(&options),
    )?;
    Ok(artifact)
}
