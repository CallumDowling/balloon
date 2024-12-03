//Copyright 2024 Callum Dowling
//
//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
//limitations under the License.

vulkano_shaders::shader! {
    ty: "vertex",
    src: r"

        #version 450

        layout(location = 0) in vec3 position;
        layout(location = 1) in vec3 color;
        layout(location = 2) in vec3 normal;

        layout(location = 0) out vec3 v_color;

        layout(set = 0, binding = 0) uniform VPData {
            mat4 model;
            mat4 view;
            mat4 projection;
        } uniforms;

        void main() {
            v_color = color;
            mat4 worldview = uniforms.view * uniforms.model;
            gl_Position = uniforms.projection * worldview * vec4(position, 1.0);
        }
    ",
}
