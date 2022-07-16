pub const SHADER: &str = r#"#version 300 es
    precision mediump float;

    uniform float u_opacity;
    in lowp vec4 v_color;
    out vec4 frag_color;

    void main() {
        frag_color = vec4(v_color.r, v_color.g, v_color.b, v_color.a * u_opacity);
    }
"#;
