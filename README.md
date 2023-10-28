# Glowmesh
This is a thin mesh abstraction for glow. Its very ooga booga style: simply implemented per common vertex type. For now XYZRGBA. The _build2d module is for 2d rendering, providing functionality for triangle, quad and polygon rendering (more to come like lines etc).


examples/triangle may serve as a starting point for OpenGL application development

## Todo
* todo fix resizing on wayland if thats even possible lmao
* todo recycling handles
* todo rotozoom triangle with projection matrix... in accompanying matrix library xD
* add texture or think about what ive done wrt uvquad0. maybe gets a buffer as a texture or something like
* texture thing can bind screen buffers and shit

ISSUE - texture comes in as all 0.
fix that then we can move on eg to gball but plus this will be ready



see if using old image loading code works...