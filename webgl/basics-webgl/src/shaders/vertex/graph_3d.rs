pub const SHADER: &str = r#"#version 300 es
    in vec4 a_position;
    in float a_y;
    in vec3 a_vertex_normal;

    uniform mat4 u_normals_rotation;
    uniform mat4 u_projection;
    
    out lowp vec4 v_color;

    void main() {
        gl_Position = u_projection * vec4(a_position.x, a_y, a_position.z, 1.0);

        vec3 ambient_light = vec3(0.2, 0.2, 0.2);
        vec3 directional_light_color = vec3(1, 1, 1);
        vec3 directional_vector = normalize(vec3(-0.85, 0.8, 0.75));

        vec4 transformed_normal = u_normals_rotation * vec4(a_vertex_normal, 1.0);
        float directional = max(dot(transformed_normal.xyz, directional_vector), 0.0);
        vec3 v_lighting = ambient_light + (directional_light_color * directional);
        vec3 base_color = vec3(0.5, 0.5, 0.8);

        v_color = vec4(base_color * v_lighting, 1.0);
    }
"#;