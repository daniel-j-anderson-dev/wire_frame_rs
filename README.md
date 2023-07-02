#This is a simple wire frame viewer<br><br>

Right now there is a simple test class called Tri implemented to make sure local and global rotations are working.<br>
Eventualy I want to decouple global rotations from the window orientation.<br>
Right now the 3D -> 2D projection being used is orthographic (ignore z values).<br>

<br>##Controls:<br>
Left Shift: While held rotation is global otherwise the rotation is local<br>
<br>
W/S: x-axis rotation<br>
A/D: y-axis rotation<br>
Q/E: z-axis rotation<br>
<br>
Up/Down: x-axis translation<br>
Left/Down: y-axis translation<br>
PgDown/PgUp: z-axis translation<br>

##To build be sure to install SDL2 using the instructions here: https://crates.io/crates/sdl2