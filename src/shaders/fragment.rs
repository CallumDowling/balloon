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
    ty: "fragment",
    src: "
        #version 460

        layout(location = 0) in vec3 v_color;
        layout(location = 0) out vec4 f_color;

        void main() {
            f_color = vec4(v_color, 1.0);
        }
    ",
}
