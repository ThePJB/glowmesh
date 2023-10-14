Use minvect for the types
Probs gonna have to make minmat for calculating the projection matrix hehe. use ortho etc for camera shit

the only thing im tossing up is whether to represent mat4 as a &[f32; 16] or as a [Vec4; 4] - column vectors
Column vectors is the based way of understanding.
for consumption to opengl probs want it as f32s...

matrix for fixed 2d cam
matrix for moving 2d cam
matrix for 3d cam - thats what i did in okbloomer



i guess u want some struct implement shit as event handler or w/e