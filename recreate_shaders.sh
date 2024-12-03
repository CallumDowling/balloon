#!/bin/bash
glslc -fshader-stage=frag src/shaders/fragment.glsl -o src/shaders/fragment.spv
glslc -fshader-stage=vert src/shaders/vertex.glsl -o src/shaders/vertex.spv
