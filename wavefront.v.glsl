attribute vec4 position;

void main() {
    gl_Position = vec4(position.xyz, 20.0);
}
