pub const SHADER: &str = r#"#version 300 es
    precision mediump float;

    uniform vec4 u_color;
    uniform float u_opacity;
    out vec4 frag_color;

    void main() {
        frag_color = vec4(u_color.r, u_color.g, u_color.b, u_color.a * u_opacity);
    }
"#;
