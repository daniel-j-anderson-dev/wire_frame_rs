<h1>This is a simple wire frame viewer</h1><br><br>

Right now there is a simple test class called Tri implemented to make sure local and global rotations are working.<br>
Eventualy I want to decouple global rotations from the window orientation.<br>
Right now the 3D -> 2D projection being used is orthographic (ignore z values).<br>

<h2>Controls:</h2>
<ul>
    <li>Left Shift: While held rotation is global otherwise the rotation is local</li>
    <li>W/S: x-axis rotation</li>
    <li>A/D: y-axis rotation</li>
    <li>Q/E: z-axis rotation</li>
    <li>Up/Down: x-axis translation</li>
    <li>Left/Down: y-axis translation</li>
    <li>PgDown/PgUp: z-axis translation</li>
</ul>

<h1>To build be sure to install SDL2 using the instructions here: https://crates.io/crates/sdl2<h1>